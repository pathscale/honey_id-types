# Honey ID Types

This crate provides type definitions for all externally used endpoints of Honey.id-Auth Server

## Usage

Types required for Public or API Key access to Honey Auth server are exported from the main lib of this library crate. They can therefore be accessed as follows:

```rust
use honey_id_types::api_key_endpoints::connect::ApiKeyConnectRequest;
```

The modules have been structured so as to make all types easy to find and completely unambiguous.

## Development

If new types need to be added, follow this process:

1. Edit the appropriate `.ron` files in [config](./config/schema_lists/)
2. Ensure the numbering of the schema list and endpoint match the existing pattern
3. Run [regenerate_endpoints.sh](./scripts/regenerate_endpoints.sh)
4. Export the types using the existing pattern if they will be added to one of the existing categories, or add a new module and ensure the final exported type matches the pattern in the code block shown above
5. Ensure the build and CI passes before releasing a new version and/or publishing to crates.io