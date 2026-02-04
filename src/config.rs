use secrecy::SecretString;
use serde::Deserialize;
use smart_default::SmartDefault;
use smart_serde_default::smart_serde_default;
use url::Url;
use uuid::Uuid;

/// Configuration for `honey.id` client.
///
/// Implements [`Deserialize`] so could be easily used as entry for other
/// config.
#[smart_serde_default]
#[derive(Clone, Debug, Deserialize, SmartDefault)]
pub struct HoneyIdConfig {
    /// `honey.id` WebSocket [`Url`].
    ///
    /// Set default to `wss://api.honey.id:443`.
    #[smart_default(Url::parse("wss://api.honey.id:443").unwrap())]
    #[serde(deserialize_with = "util::deserialize_base_url")]
    pub addr: Url,

    /// Public ID of the API app that is defined in the Auth server's config. This must match that value
    pub app_public_id: Uuid,

    /// `App` API key, which you can retrieve after `App` creation in `honey.id`
    /// UI.
    #[serde(deserialize_with = "util::deserialize_secret_string")]
    #[serde(default)]
    pub app_api_key: Option<SecretString>,

    /// `Auth` API key, which you can retrieve after `App` creation in
    /// `honey.id` UI.
    ///
    /// Will be used by `Auth` for callback endpoints authorization.
    #[serde(deserialize_with = "util::deserialize_secret_string")]
    #[serde(default)]
    pub auth_api_key: Option<SecretString>,
}

mod util {
    use std::borrow::Cow;

    use secrecy::SecretString;
    use serde::{Deserialize, Deserializer};
    use url::Url;

    /// Deserializes into an [`Option`]`<`[`Url`]`>`, ensuring it's a [base URL].
    ///
    /// [base URL]: Url::cannot_be_a_base
    pub fn deserialize_base_url<'de, D: Deserializer<'de>>(d: D) -> Result<Url, D::Error> {
        use serde::de::Error as _;

        let c = Cow::<'_, str>::deserialize(d)?;
        Url::parse(&format!("{}/", c.trim_end_matches('/')))
            .map_err(|e| D::Error::custom(format!("Invalid `Url`: {e}")))
    }

    pub fn deserialize_secret_string<'de, D: Deserializer<'de>>(
        d: D,
    ) -> Result<Option<SecretString>, D::Error> {
        let c = Cow::<'_, str>::deserialize(d)?;
        Ok(Some(SecretString::from(c.to_string())))
    }
}
