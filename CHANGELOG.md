# Changelog

All notable changes to this project will be documented in this file.
## [1.3.5] - 2026-03-12

### Features

- Upgrade worktable to 0.9.0-alpha3

### Miscellaneous Tasks

- Add required package information
- Update readme
- Update Cargo.lock

## [1.3.4] - 2026-03-06

### Bug Fixes

- Fix CI
- Fix CI again
- Remove clippy for now, we need to make changes to endpointgen for some of the warnings
- Public ID cannot be unique for tokens since one user can have multiple tokens
- Don't make the auth_api_key an option. We should fail immediately if it is not configured

### Draft

- Implement honey client (#3)

### Features

- Add all public/api key types, with nice re-exports
- Improve FE services.json with new endpoint-gen
- Allow app BEs to specify their own local endpoint for auth, without duplicating all of the auth endpoint logic
- Add support for receiving user info after signups, separate from the existing token receiving callback flow
- Improve auth api key validation error
- Add `admin_pub_id` to the main honey config for use in apps

### Miscellaneous Tasks

- Pre-release


