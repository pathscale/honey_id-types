use eyre::{Result, ensure};
use serde::{Deserialize, Serialize};
use std::fmt;
use url::Url;

const MAX_ARTIFACT_BYTES: usize = 64 * 1024;
const LAYOUT: &str = "databucket-v3/legacy-snapshot-v1/uniform";

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationV1 {
    pub format_version: u32,
    pub environment: String,
    pub service: Service,
    pub cutover_id: String,
    pub layout: String,
    pub destination: Destination,
    pub source: SourceEvidence,
    pub fence: FenceEvidence,
    pub tables: Vec<PublishedTable>,
}

/// A destination endpoint is an HTTPS origin with no userinfo, query, fragment,
/// or non-root path.
#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Destination {
    pub endpoint: String,
    pub bucket: String,
    pub region: String,
    pub prefix: String,
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SourceEvidence {
    pub writer_revision: String,
    pub profile: String,
    pub inventory_sha256: String,
    pub conversion_report_sha256: String,
}

/// Receipt references use `sha256:<64 lowercase hex digits>` and identify evidence
/// by content digest; they must not contain a URL or other locator.
#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FenceEvidence {
    pub source_ref: String,
    pub destination_ref: String,
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PublishedTable {
    pub name: String,
    pub schema_version: u32,
    pub allocator: Allocator,
    pub initial_manifest_blake3: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Service {
    Auth,
    Api,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Allocator {
    U64,
    I64,
    Unit,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TableSpec {
    pub name: &'static str,
    pub schema_version: u32,
    pub allocator: Allocator,
}

impl Service {
    fn as_str(self) -> &'static str {
        match self {
            Self::Auth => "auth",
            Self::Api => "api",
        }
    }
}

impl fmt::Debug for PublicationV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PublicationV1")
            .field("format_version", &self.format_version)
            .field("environment", &"<redacted>")
            .field("service", &self.service)
            .field("cutover_id", &"<redacted>")
            .field("layout", &"<redacted>")
            .field("destination", &self.destination)
            .field("source", &self.source)
            .field("fence", &self.fence)
            .field("tables", &self.tables)
            .finish()
    }
}

impl fmt::Debug for Destination {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Destination")
            .field("endpoint", &"<redacted>")
            .field("bucket", &"<redacted>")
            .field("region", &"<redacted>")
            .field("prefix", &"<redacted>")
            .finish()
    }
}

impl fmt::Debug for SourceEvidence {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SourceEvidence")
            .field("writer_revision", &"<redacted>")
            .field("profile", &"<redacted>")
            .field("inventory_sha256", &"<redacted>")
            .field("conversion_report_sha256", &"<redacted>")
            .finish()
    }
}

impl fmt::Debug for FenceEvidence {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FenceEvidence")
            .field("source_ref", &"<redacted>")
            .field("destination_ref", &"<redacted>")
            .finish()
    }
}

impl fmt::Debug for PublishedTable {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PublishedTable")
            .field("name", &"<redacted>")
            .field("schema_version", &self.schema_version)
            .field("allocator", &self.allocator)
            .field("initial_manifest_blake3", &"<redacted>")
            .finish()
    }
}

impl PublicationV1 {
    fn validate(&self) -> Result<()> {
        ensure!(self.format_version == 1, "unsupported publication format version");
        ensure!(
            matches!(self.environment.as_str(), "dev" | "prod"),
            "unsupported publication environment"
        );
        ensure!(self.layout == LAYOUT, "unsupported publication layout");
        ensure!(
            is_lower_hex(&self.cutover_id, 32),
            "cutover_id must be 32 lowercase hexadecimal characters"
        );
        validate_destination(&self.destination)?;
        ensure!(
            self.destination.prefix == expected_prefix(&self.environment, self.service, &self.cutover_id),
            "destination prefix does not match publication identity"
        );
        ensure!(
            is_git_revision(&self.source.writer_revision),
            "writer_revision must be a full lowercase Git commit ID"
        );
        ensure!(
            valid_identifier(&self.source.profile),
            "source profile is empty or invalid"
        );
        ensure!(
            is_lower_hex(&self.source.inventory_sha256, 64),
            "inventory_sha256 must be 64 lowercase hexadecimal characters"
        );
        ensure!(
            is_lower_hex(&self.source.conversion_report_sha256, 64),
            "conversion_report_sha256 must be 64 lowercase hexadecimal characters"
        );
        validate_evidence_ref(&self.fence.source_ref)?;
        validate_evidence_ref(&self.fence.destination_ref)?;
        ensure!(!self.tables.is_empty(), "publication must contain tables");
        let mut previous: Option<&str> = None;
        for table in &self.tables {
            ensure!(valid_table_name(&table.name), "invalid table name");
            ensure!(table.schema_version > 0, "table schema version must be positive");
            ensure!(
                is_lower_hex(&table.initial_manifest_blake3, 64),
                "table manifest digest must be 64 lowercase hexadecimal characters"
            );
            if let Some(prev) = previous {
                ensure!(
                    prev < table.name.as_str(),
                    "table names must be unique and strictly sorted"
                );
            }
            previous = Some(&table.name);
        }
        Ok(())
    }
}

pub fn decode_v1(bytes: &[u8]) -> Result<PublicationV1> {
    ensure!(bytes.len() <= MAX_ARTIFACT_BYTES, "publication artifact exceeds 64 KiB");
    let artifact: PublicationV1 =
        serde_json::from_slice(bytes).map_err(|_| eyre::eyre!("invalid publication artifact JSON"))?;
    artifact.validate()?;
    Ok(artifact)
}

pub fn encode_v1(artifact: &PublicationV1) -> Result<Vec<u8>> {
    artifact.validate()?;
    let bytes = serde_json::to_vec(artifact).map_err(|_| eyre::eyre!("could not encode publication artifact"))?;
    ensure!(bytes.len() <= MAX_ARTIFACT_BYTES, "publication artifact exceeds 64 KiB");
    Ok(bytes)
}

pub fn validate_binding(
    artifact: &PublicationV1,
    service: Service,
    destination: &Destination,
    expected: &[TableSpec],
) -> Result<()> {
    artifact.validate()?;
    ensure!(
        artifact.service == service,
        "publication service does not match runtime service"
    );
    ensure!(
        artifact.destination == *destination,
        "publication destination does not match configured destination"
    );

    let mut specs = expected.to_vec();
    specs.sort_by(|left, right| left.name.cmp(right.name));
    for pair in specs.windows(2) {
        ensure!(
            pair[0].name != pair[1].name,
            "compiled table profile contains duplicate names"
        );
    }
    ensure!(
        specs.len() == artifact.tables.len(),
        "publication table set has the wrong size"
    );
    for (spec, published) in specs.iter().zip(&artifact.tables) {
        ensure!(
            spec.name == published.name,
            "publication table set does not match compiled profile"
        );
        ensure!(
            spec.schema_version == published.schema_version,
            "published table schema version does not match compiled profile"
        );
        ensure!(
            spec.allocator == published.allocator,
            "published table allocator does not match compiled profile"
        );
    }
    Ok(())
}

fn validate_destination(destination: &Destination) -> Result<()> {
    let parsed = Url::parse(&destination.endpoint).map_err(|_| eyre::eyre!("invalid destination endpoint"))?;
    ensure!(parsed.scheme() == "https", "dev destination endpoint must use HTTPS");
    ensure!(parsed.host_str().is_some(), "destination endpoint must include a host");
    ensure!(
        parsed.username().is_empty() && parsed.password().is_none(),
        "destination endpoint must not contain userinfo"
    );
    ensure!(
        parsed.query().is_none() && parsed.fragment().is_none(),
        "destination endpoint must not contain a query or fragment"
    );
    ensure!(parsed.path() == "/", "destination endpoint must not contain a path");
    ensure!(
        destination.bucket.bytes().all(is_safe_component_byte) && !destination.bucket.is_empty(),
        "destination bucket is empty or invalid"
    );
    ensure!(
        destination.region == "auto",
        "effective destination region must be auto"
    );
    ensure!(
        !destination.prefix.is_empty() && !destination.prefix.starts_with('/') && !destination.prefix.ends_with('/'),
        "destination prefix is not canonical"
    );
    ensure!(
        destination
            .prefix
            .split('/')
            .all(|part| !part.is_empty() && part != "." && part != ".." && part.bytes().all(is_safe_component_byte)),
        "destination prefix is not canonical"
    );
    Ok(())
}

fn expected_prefix(environment: &str, service: Service, cutover_id: &str) -> String {
    format!("honey/{environment}/{}/{cutover_id}", service.as_str())
}

fn validate_evidence_ref(value: &str) -> Result<()> {
    // Receipt references are content digests, never locations or bearer credentials.
    ensure!(
        value.len() == 71 && value.starts_with("sha256:") && is_lower_hex(&value[7..], 64),
        "fence evidence reference must be sha256:<64 lowercase hexadecimal characters>"
    );
    Ok(())
}

fn is_git_revision(value: &str) -> bool {
    (value.len() == 40 || value.len() == 64) && is_lower_hex(value, value.len())
}

fn is_lower_hex(value: &str, length: usize) -> bool {
    value.len() == length
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn valid_identifier(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"._-".contains(&byte))
}

fn valid_table_name(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value.as_bytes()[0].is_ascii_lowercase()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}

fn is_safe_component_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || b"._-".contains(&byte)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn artifact() -> PublicationV1 {
        PublicationV1 {
            format_version: 1,
            environment: "dev".into(),
            service: Service::Auth,
            cutover_id: "0123456789abcdef0123456789abcdef".into(),
            layout: LAYOUT.into(),
            destination: Destination {
                endpoint: "https://s3.example.test".into(),
                bucket: "honey-dev".into(),
                region: "auto".into(),
                prefix: "honey/dev/auth/0123456789abcdef0123456789abcdef".into(),
            },
            source: SourceEvidence {
                writer_revision: "a".repeat(40),
                profile: "auth-dev-reviewed-v1".into(),
                inventory_sha256: "b".repeat(64),
                conversion_report_sha256: "c".repeat(64),
            },
            fence: FenceEvidence {
                source_ref: format!("sha256:{}", "1".repeat(64)),
                destination_ref: format!("sha256:{}", "2".repeat(64)),
            },
            tables: vec![PublishedTable {
                name: "app_config".into(),
                schema_version: 5,
                allocator: Allocator::U64,
                initial_manifest_blake3: "d".repeat(64),
            }],
        }
    }

    #[test]
    fn round_trip_and_binding_ignore_expected_order() {
        let mut value = artifact();
        value.tables.push(PublishedTable {
            name: "user".into(),
            schema_version: 6,
            allocator: Allocator::U64,
            initial_manifest_blake3: "e".repeat(64),
        });
        let bytes = encode_v1(&value).unwrap();
        assert_eq!(decode_v1(&bytes).unwrap(), value);
        let expected = [
            TableSpec {
                name: "user",
                schema_version: 6,
                allocator: Allocator::U64,
            },
            TableSpec {
                name: "app_config",
                schema_version: 5,
                allocator: Allocator::U64,
            },
        ];
        validate_binding(&value, Service::Auth, &value.destination, &expected).unwrap();
    }

    #[test]
    fn rejects_unknown_and_duplicate_fields() {
        let valid = String::from_utf8(encode_v1(&artifact()).unwrap()).unwrap();
        let unknown = valid.replacen("{\"format_version\":1,", "{\"unexpected\":0,\"format_version\":1,", 1);
        assert!(decode_v1(unknown.as_bytes()).is_err());
        let duplicate = valid.replacen(
            "{\"format_version\":1,",
            "{\"format_version\":1,\"format_version\":1,",
            1,
        );
        assert!(decode_v1(duplicate.as_bytes()).is_err());
    }

    #[test]
    fn rejects_unsorted_tables_bad_endpoint_and_oversize_input() {
        let mut value = artifact();
        value.tables = vec![
            PublishedTable {
                name: "user".into(),
                schema_version: 6,
                allocator: Allocator::U64,
                initial_manifest_blake3: "e".repeat(64),
            },
            value.tables.remove(0),
        ];
        assert!(encode_v1(&value).is_err());
        let mut value = artifact();
        value.destination.endpoint = "https://user:pass@s3.example.test/path?x=1".into();
        assert!(encode_v1(&value).is_err());
        assert!(decode_v1(&vec![b' '; MAX_ARTIFACT_BYTES + 1]).is_err());
    }

    #[test]
    fn accepts_only_digest_fence_references() {
        let valid = format!("sha256:{}", "a".repeat(64));
        assert!(validate_evidence_ref(&valid).is_ok());

        for invalid in [
            "source-42",
            "https://e.example/fence?token=example-secret",
            "https://user:pass@e.example/fence",
            "sha256:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
            "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        ] {
            assert!(validate_evidence_ref(invalid).is_err());
        }
    }

    #[test]
    fn debug_and_validation_errors_redact_untrusted_values() {
        let mut value = artifact();
        value.fence.source_ref = "https://e.example/fence?token=private-value".into();
        let debug = format!("{value:?}");
        assert!(!debug.contains("private-value"));

        value.fence.source_ref = format!("sha256:{}", "1".repeat(64));
        value.tables[0].name = "private-table-name".into();
        let error = encode_v1(&value).unwrap_err().to_string();
        assert!(!error.contains("private-table-name"));
    }

    #[test]
    fn destination_endpoint_rejects_paths_even_without_query_or_userinfo() {
        let mut value = artifact();
        value.destination.endpoint = "https://s3.example.test/private-credential".into();
        assert!(encode_v1(&value).is_err());
    }

    #[test]
    fn rejects_binding_mismatches() {
        let value = artifact();
        let wrong = [TableSpec {
            name: "app_config",
            schema_version: 4,
            allocator: Allocator::U64,
        }];
        assert!(validate_binding(&value, Service::Auth, &value.destination, &wrong).is_err());
        assert!(validate_binding(&value, Service::Api, &value.destination, &[]).is_err());
    }
}
