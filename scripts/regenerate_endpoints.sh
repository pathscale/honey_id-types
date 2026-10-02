#!/bin/sh
set -eu

contract_root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$contract_root"

# Check paired sources when using the coordinated checkout. The published
# shared crate can also regenerate from its own frozen sources standalone.
if [ -d ../auth.honey.id-backend/config ]; then
    cmp config/schema_lists/000_public/001_auth_api.ron ../auth.honey.id-backend/config/schema_lists/000_public/001_auth_api.ron
    cmp config/schema_lists/022_app/022_app_settings_api.ron ../auth.honey.id-backend/config/schema_lists/022_app/022_app_settings_api.ron
    cmp config/structs.ron ../auth.honey.id-backend/config/structs.ron
fi

endpoint-gen --config-dir config/
cp generated/model.rs src/types/generated.rs
