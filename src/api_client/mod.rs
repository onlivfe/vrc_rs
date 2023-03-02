//! An optional API client feature using `reqwest`
//!
//! Besides using this, you could instead easily implement your own client using
//! a different HTTP library with the [`racal::Queryable`](racal::Queryable)
//! trait. Though this does additionally support unwrapping the message/data of
//! the `VRChat` API responses.
//!
//! If you're implementing your own API client, you need to implement two
//! possible API states:
//!
//! 1. [`vrc::query::NoAuthentication`](crate::query::NoAuthentication)
//!
//! > Doesn't require authentication but still needs to be rate limited
//! > properly.
//!
//! 2. [`vrc::model::UserSession`](crate::model::UserSession)
//!
//! > Requires the `Authorization` header in addition to the rate limiting.

use std::num::NonZeroU32;

use governor::{
	clock::DefaultClock,
	middleware::NoOpMiddleware,
	state::{InMemoryState, NotKeyed},
	Quota,
	RateLimiter,
};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use racal::reqwest::ApiError;
use reqwest::{header::HeaderMap, Client, RequestBuilder};

use crate::query::{Authenticating, Authentication};

type NormalRateLimiter =
	RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>;

#[must_use]
fn http_rate_limiter() -> NormalRateLimiter {
	// ~5 seconds per request sustained over one minute, allowing up to a request
	// per second in bursts.
	RateLimiter::direct(
		Quota::per_minute(NonZeroU32::try_from(12).unwrap())
			.allow_burst(NonZeroU32::try_from(5).unwrap()),
	)
}

/// The main API client without authentication
pub struct UnauthenticatedVRC {
	user_agent: String,
	http: Client,
	rate_limiter: NormalRateLimiter,
	auth: Authenticating,
}

#[async_trait::async_trait]
impl racal::reqwest::ApiClient<Authenticating> for UnauthenticatedVRC {
	fn state(&self) -> &Authenticating { &self.auth }

	fn client(&self) -> &reqwest::Client { &self.http }

	async fn before_request(
		&self, req: RequestBuilder,
	) -> Result<RequestBuilder, racal::reqwest::ApiError> {
		self.rate_limiter.until_ready().await;
		Ok(req)
	}
}

/// The main API client with authentication
pub struct AuthenticatedVRC {
	user_agent: String,
	http: Client,
	rate_limiter: NormalRateLimiter,
	auth: Authentication,
}

#[async_trait::async_trait]
impl racal::reqwest::ApiClient<Authentication> for AuthenticatedVRC {
	fn state(&self) -> &Authentication { &self.auth }

	fn client(&self) -> &reqwest::Client { &self.http }

	async fn before_request(
		&self, req: RequestBuilder,
	) -> Result<RequestBuilder, racal::reqwest::ApiError> {
		self.rate_limiter.until_ready().await;
		Ok(req)
	}
}

impl AuthenticatedVRC {
	/// Creates an API client
	fn http_client(
		user_agent: &str, auth: &Authentication,
	) -> Result<Client, ApiError> {
		use serde::ser::Error;

		let builder = Client::builder();
		let mut headers = HeaderMap::new();
		headers
			.insert(reqwest::header::ACCEPT, "application/json".parse().unwrap());
		headers.insert(
			reqwest::header::CONTENT_TYPE,
			"application/json".parse().unwrap(),
		);

		let mut cookie =
			"apiKey=".to_owned() + crate::API_KEY + "; auth=" + &auth.token;
		if let Some(second_factor) = &auth.second_factor_token {
			cookie.push_str("; twoFactorAuth=");
			cookie.push_str(second_factor);
		}

		headers.insert(
			reqwest::header::COOKIE,
			cookie.parse().map_err(|_| {
				serde_json::Error::custom("Couldn't turn auth into a cookie header")
			})?,
		);

		Ok(builder.user_agent(user_agent).default_headers(headers).build()?)
	}

	/// Removes authentication to the API client
	///
	/// # Errors
	///
	/// If deserializing user agent fails.
	pub fn downgrade(
		self, auth: impl Into<Authenticating>,
	) -> Result<UnauthenticatedVRC, ApiError> {
		let auth = auth.into();
		Ok(UnauthenticatedVRC {
			http: UnauthenticatedVRC::http_client(&self.user_agent, &auth)?,
			rate_limiter: self.rate_limiter,
			user_agent: self.user_agent,
			auth,
		})
	}

	/// Creates a new authenticated VRC API client
	///
	/// # Errors
	///
	/// If deserializing user agent into a header fails
	pub fn new(
		user_agent: String, auth: impl Into<Authentication> + Send,
	) -> Result<Self, ApiError> {
		let auth = auth.into();
		Ok(Self {
			http: Self::http_client(&user_agent, &auth)?,
			rate_limiter: http_rate_limiter(),
			user_agent,
			auth,
		})
	}

	/// Changes the second factor token.
	///
	/// Login flow should be started with an unauthenticated client,
	/// then upgraded to an authenticated one if it fails with 2FA missing,
	/// and a verify 2FA code request should be made to get the token,
	/// which should then be used with this.
	///
	/// Not most rust-like API, but the VRC authentication is quite a mess
	/// already,  with how it's dependent on cookies and such.
	///
	/// # Errors
	///
	/// If deserializing user agent or authentication fails.
	pub fn change_second_factor(
		self, second_factor_token: impl Into<Option<String>>,
	) -> Result<Self, ApiError> {
		let mut auth = self.auth;
		auth.second_factor_token = second_factor_token.into();
		Ok(Self {
			http: Self::http_client(&self.user_agent, &auth)?,
			rate_limiter: self.rate_limiter,
			user_agent: self.user_agent,
			auth,
		})
	}
}

impl UnauthenticatedVRC {
	/// Creates an API client
	fn http_client(
		user_agent: &str, auth: &Authenticating,
	) -> Result<Client, ApiError> {
		use base64::Engine as _;
		use serde::ser::Error;

		let builder = Client::builder();
		let mut headers = HeaderMap::new();
		headers
			.insert(reqwest::header::ACCEPT, "application/json".parse().unwrap());
		headers.insert(
			reqwest::header::CONTENT_TYPE,
			"application/json".parse().unwrap(),
		);

		headers.insert(
			reqwest::header::COOKIE,
			format!("apiKey={}", crate::API_KEY).parse().map_err(|_| {
				serde_json::Error::custom("Couldn't turn auth into a cookie header")
			})?,
		);

		// This is dumb...
		let auth = "Basic ".to_owned()
			+ &base64::engine::general_purpose::URL_SAFE.encode(
				utf8_percent_encode(&auth.username, NON_ALPHANUMERIC).to_string()
					+ ":" + &utf8_percent_encode(&auth.password, NON_ALPHANUMERIC)
					.to_string(),
			);
		headers.insert(
			reqwest::header::AUTHORIZATION,
			auth.parse().map_err(|_| {
				serde_json::Error::custom("Couldn't turn username into a header")
			})?,
		);

		Ok(builder.user_agent(user_agent).default_headers(headers).build()?)
	}

	/// Adds authentication to the API client
	///
	/// # Errors
	///
	/// If deserializing user agent or authentication fails.
	pub fn upgrade(
		self, auth: impl Into<Authentication> + Send,
	) -> Result<AuthenticatedVRC, ApiError> {
		let auth = auth.into();
		Ok(AuthenticatedVRC {
			http: AuthenticatedVRC::http_client(&self.user_agent, &auth)?,
			rate_limiter: self.rate_limiter,
			user_agent: self.user_agent,
			auth,
		})
	}

	/// Creates a new VRC API client
	///
	/// # Errors
	///
	/// If deserializing user agent into a header fails
	pub fn new(
		user_agent: String, auth: impl Into<Authenticating>,
	) -> Result<Self, ApiError> {
		let auth = auth.into();
		Ok(Self {
			http: Self::http_client(&user_agent, &auth)?,
			rate_limiter: http_rate_limiter(),
			user_agent,
			auth,
		})
	}
}
