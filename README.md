# VRChat API in rust

<img align="right" width="256" height="256" src="https://github.com/onlivfe/vrc_rs/raw/main/logo.png"/>

[![License](https://img.shields.io/crates/l/vrc.svg)](https://github.com/onlivfe/vrc_rs/src/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/vrc.svg)](https://crates.io/crates/vrc)
[![Docs](https://docs.rs/vrc/badge.svg)](https://docs.rs/crate/vrc/)

WIP rust models of [VRChat](https://store.steampowered.com/app/438100/VRChat/)'s API.

This is fully unofficial and in no way affiliated, endorsed, supported, or created by VRChat.

Huge shoutout to the MIT licensed [vrchatapi](https://vrchatapi.github.io), which this crate borrows a lot from.
If you're looking for a more mature project, please consider looking at their [generated rust crate](https://github.com/vrchatapi/vrchatapi-rust) instead.

I disliked the generated nature of it though, and wanted more strong Rust-like things for some things, which is why this crate exists.

There's quite a bit missing, PRs to improve the situation, even beyond the plans:

| Status | Category | Planned |
| --- | --- | --- |
| Implemented | Authentication | More testing |
| None | Avatars | Getting a specific avatar only |
| None | Economy | None |
| None | Favorites | Eventual implementation |
| None | Files | None |
| Partial | Friends | Implementation soon |
| None | Groups | None, at least in the near term |
| None | Invites | Listing invites only |
| Partial models only | Instances | Implementation soon |
| None | Notifications | Eventual implementation |
| None | Permissions | None |
| None | System | Eventual implementation of system statistics |
| Partial | Users | Planned to be implemented soon |
| None | Worlds | None, at least in the near term |

The categories are from the awesome [VRChat API Docs](https://vrchatapi.github.io/docs/api/) project.

## Testing

The integration tests contact the live API.
That's why they are ignored by default.

Pretty much all of them require authentication.

Sadly not all the things can even be reliably tested without creating a mock API.
Which in turn defeats the purpose of the tests in the first place.
So only some of the behavior is actually tested, though improvements to it are welcome.

### Creating a user session manually

Getting the authentication for VRC takes a few steps due to it's janky usage of cookies, and 2fa requiring another request.

You can use the `get_auth.py` script for convenience.

### Running ignored tests

Make sure that you've got:

- an internet connection
- a valid `user-sesion.json`

Then just run the tests;

```sh
# A specific test with output logging
cargo test --all-features current_user -- --exact --ignored --nocapture
# All tests
cargo test --all-features -- --ignored
```

## License

Note that the license is `MPL-2.0` instead of the more common `MIT OR Apache-2.0`.
