# honey_id-types

[![Crates.io](https://img.shields.io/crates/v/honey_id-types)](https://crates.io/crates/honey_id-types)
[![License: MIT](https://img.shields.io/crates/l/honey_id-types)](LICENSE)

Shared types and utilities for integrating with the [honey.id](https://honey.id) Auth server. Used across Pathscale projects that connect to honey.id for authentication.

Provides:
- Generated request/response types for all honey.id WebSocket endpoints
- Handler traits for Auth→App and User→App communication patterns
- A `HoneyIdClient` for connecting to and communicating with the Auth server
- `HoneyIdConfig` for configuring the client

## Usage

Add to your `Cargo.toml`:

```toml
honey_id-types = "1.3.4"
```

Endpoint request/response types are organized by category under `endpoints`:

```rust
use honey_id_types::endpoints::connect::HoneyApiKeyConnectRequest;
use honey_id_types::endpoints::auth_flow::HoneySubmitPasswordRequest;
```

### Configuration

```rust
use honey_id_types::HoneyIdConfig;

let config = HoneyIdConfig {
    addr: "wss://api.honey.id:443".parse()?,
    app_public_id: my_app_uuid,
    auth_api_key: my_api_key,
    admin_pub_id: None,
};
```

### Client

```rust
use honey_id_types::HoneyIdClient;

let client = HoneyIdClient::new(config).await?;
```

## Modules

- `endpoints` — request/response types for all honey.id WebSocket endpoints, grouped by flow (`connect`, `auth_flow`, `callback`, etc.)
- `enums` — shared enum types
- `id_entities` — identity-related types
- `handlers` — handler traits for implementing Auth→App and User→App message handling in your service

## Development

Endpoint types are generated from `.ron` schema files using `endpoint-gen`. To add or modify endpoint types:

1. Edit the relevant `.ron` files in [`config/`](./config/)
2. Run the regeneration script:
   ```sh
   ./scripts/regenerate_endpoints.sh
   ```
3. Re-export any new types through the appropriate module in `src/types/endpoints.rs` (or add a new module following the existing pattern)
4. Ensure the build passes before releasing

## Version Compatibility

`honey_id-types` is versioned alongside `endpoint-libs` and `endpoint-gen`. **Minor versions must match** across all three crates in a project.

For example, `honey_id-types 1.3.x` must be paired with `endpoint-libs 1.3.x` and `endpoint-gen 1.3.x`.

## Releasing

Releases are managed with [`cargo-release`](https://github.com/crate-ci/cargo-release) and [`git-cliff`](https://github.com/orhun/git-cliff). Both must be installed:

```sh
cargo install cargo-release git-cliff
```

To cut a release:

```sh
./scripts/release.sh [--skip-bump] <patch|minor|major>
```

The script will:
1. Run `cargo release --execute <level>` — bumps the version in `Cargo.toml`, regenerates `CHANGELOG.md`, and commits everything as `chore(release): vX.Y.Z`.
2. Open your `$EDITOR` with the auto-generated tag notes (from `git-cliff`) for review.
3. Create an annotated tag using the edited notes as the tag body (shown as GitHub Release notes).
4. Push the commit and tag.
5. Prompt whether to publish to crates.io.

To preview what `cargo-release` would do without making changes:

```sh
cargo release patch  # omit --execute for a dry run
```
