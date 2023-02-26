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

use governor::{
	clock::DefaultClock,
	middleware::NoOpMiddleware,
	state::{InMemoryState, NotKeyed},
	Quota,
	RateLimiter,
};
use racal::{Queryable, RequestMethod};
use reqwest::{header::HeaderMap, Client};
use serde::de::DeserializeOwned;
use std::num::NonZeroU32;

use crate::query::{Authentication, NoAuthentication};

/// An error that may happen with an API query
#[derive(Debug)]
pub enum ApiError {
	/// An error happened with serialization
	Serde(serde_json::Error),
	/// An error happened with the request itself
	Reqwest(reqwest::Error),
}

impl From<serde_json::Error> for ApiError {
	fn from(err: serde_json::Error) -> Self {
		Self::Serde(err)
	}
}

impl From<reqwest::Error> for ApiError {
	fn from(err: reqwest::Error) -> Self {
		Self::Reqwest(err)
	}
}

type NormalRateLimiter =
	RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>;

/// The main API client without authentication
pub struct UnauthenticatedVRC {
	user_agent: String,
	http: Client,
	rate_limiter: NormalRateLimiter,
}

/// The main API client with authentication
pub struct AuthenticatedVRC {
	user_agent: String,
	http: Client,
	rate_limiter: NormalRateLimiter,
	auth: Authentication,
}

async fn base_query<R, FromState: Send, T>(
	http: &Client,
	api_state: FromState,
	rate_limiter: &NormalRateLimiter,
	queryable: T,
) -> Result<R, ApiError>
where
	R: DeserializeOwned,
	T: Queryable<FromState, R> + Send + Sync,
{
	let mut request = http.request(
		match queryable.method(&api_state) {
			RequestMethod::Get => reqwest::Method::GET,
			RequestMethod::Head => reqwest::Method::HEAD,
			RequestMethod::Patch => reqwest::Method::PATCH,
			RequestMethod::Post => reqwest::Method::POST,
			RequestMethod::Put => reqwest::Method::PUT,
			RequestMethod::Delete => reqwest::Method::DELETE,
		},
		queryable.url(&api_state),
	);
	if let Some(body) = queryable.body(&api_state) {
		request = request.body(body?);
	}

	rate_limiter.until_ready().await;
	let response = request.send().await?.error_for_status()?;
	// TODO: Figure out if there are any extra rate limit headers to respect

	let bytes = response.bytes().await?;
	Ok(queryable.deserialize(&bytes)?)
}

#[must_use]
fn http_rate_limiter() -> NormalRateLimiter {
	// ~5 seconds per request sustained over one minute, allowing up to a request
	// per second in bursts.
	RateLimiter::direct(
		Quota::per_minute(NonZeroU32::try_from(12).unwrap())
			.allow_burst(NonZeroU32::try_from(5).unwrap()),
	)
}

impl AuthenticatedVRC {
	/// Creates an API client
	fn http_client(user_agent: &str, auth: &Authentication) -> Result<Client, ApiError> {
		use serde::ser::Error;

		let builder = Client::builder();
		let mut headers = HeaderMap::new();

		// TODO: authentication

		Ok(builder.user_agent(user_agent).default_headers(headers).build()?)
	}

	/// Removes authentication to the API client
	///
	/// # Errors
	///
	/// If deserializing user agent fails.
	pub fn downgrade(self) -> Result<UnauthenticatedVRC, ApiError> {
		Ok(UnauthenticatedVRC {
			http: UnauthenticatedVRC::http_client(&self.user_agent)?,
			rate_limiter: self.rate_limiter,
			user_agent: self.user_agent,
		})
	}

	/// Creates a new authenticated VRC API client
	///
	/// # Errors
	///
	/// If deserializing user agent into a header fails
	pub fn new(
		user_agent: String,
		auth: impl Into<Authentication> + Send,
	) -> Result<Self, ApiError> {
		let auth = auth.into();
		Ok(Self {
			http: Self::http_client(&user_agent, &auth)?,
			rate_limiter: http_rate_limiter(),
			user_agent,
			auth,
		})
	}

	/// Sends a query to the VRC API
	///
	/// # Errors
	///
	/// If something with the request failed.
	pub async fn query<'a, R, FromState, T>(&'a self, queryable: T) -> Result<R, ApiError>
	where
		R: DeserializeOwned,
		FromState: From<&'a Authentication> + Send,
		T: Queryable<FromState, R> + Send + Sync,
	{
		let state = FromState::from(&self.auth);
		base_query(&self.http, state, &self.rate_limiter, queryable).await
	}
}

impl UnauthenticatedVRC {
	/// Creates an unauthenticated API client
	fn http_client(user_agent: &str) -> Result<Client, ApiError> {
		Ok(Client::builder().user_agent(user_agent).build()?)
	}

	/// Adds authentication to the API client
	///
	/// # Errors
	///
	/// If deserializing user agent or authentication fails.
	pub fn upgrade(
		self,
		auth: impl Into<Authentication> + Send,
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
	pub fn new(user_agent: String) -> Result<Self, ApiError> {
		Ok(Self {
			http: Self::http_client(&user_agent)?,
			rate_limiter: http_rate_limiter(),
			user_agent,
		})
	}

	/// Sends a query to the VRC API
	///
	/// # Errors
	///
	/// If something with the request failed.
	pub async fn query<'a, R, FromState, T>(&'a self, queryable: T) -> Result<R, ApiError>
	where
		R: DeserializeOwned,
		FromState: From<&'a NoAuthentication> + Send,
		T: Queryable<FromState, R> + Send + Sync,
	{
		let state = FromState::from(&NoAuthentication {});
		base_query(&self.http, state, &self.rate_limiter, queryable).await
	}
}
