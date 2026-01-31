use secrecy::SecretString;
use serde::Deserialize;
use smart_default::SmartDefault;
use url::Url;
use uuid::Uuid;

/// Configuration for `honey.id` client.
///
/// Implements [`Deserialize`] so could be easily used as entry for other
/// config.
#[derive(Clone, Debug, Deserialize, SmartDefault)]
pub struct HoneyIdConfig {
    /// `honey.id` WebSocket [`Url`].
    ///
    /// Set default to `wss://api.honey.id:443`.
    #[default(Url::parse("wss://api.honey.id:443").unwrap())]
    #[serde(deserialize_with = "util::deserialize_base_url")]
    pub addr: Url,

    /// Public ID of your `App`, which you can retrieve after `App` creation in
    /// `honey.id` UI.
    pub app_public_id: Uuid,

    /// `App` API key, which you can retrieve after `App` creation in `honey.id`
    /// UI.
    #[serde(deserialize_with = "util::deserialize_secret_string")]
    pub app_api_key: SecretString,

    /// `Auth` API key, which you can retrieve after `App` creation in
    /// `honey.id` UI.
    ///
    /// Will be used by `Auth` for callback endpoints authorization.
    #[serde(deserialize_with = "util::deserialize_secret_string")]
    pub auth_api_key: SecretString,
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
    ) -> Result<SecretString, D::Error> {
        let c = Cow::<'_, str>::deserialize(d)?;
        Ok(SecretString::from(c.to_string()))
    }
}
