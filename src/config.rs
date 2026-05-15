use psc_nanoid::{Nanoid, alphabet::Base62Alphabet};
use secrecy::SecretString;
use serde::Deserialize;
use smart_default::SmartDefault;
use url::Url;

/// Configuration for `honey.id` client.
///
/// Implements [`Deserialize`] so could be easily used as entry for other
/// config.
#[derive(Clone, Debug, Deserialize, SmartDefault)]
#[serde(default)]
pub struct HoneyIdConfig {
    /// `honey.id` WebSocket [`Url`].
    ///
    /// Set default to `wss://api.honey.id:443`.
    #[default(Url::parse("wss://api.honey.id:443").unwrap())]
    pub addr: Url,

    /// Public ID of the API app that is defined in the Auth server's config. This must match that value
    #[default(Nanoid::new())]
    pub app_public_id: Nanoid<16, Base62Alphabet>,

    /// `Auth` API key, which you can retrieve after `App` creation in
    /// `honey.id` UI.
    ///
    /// Will be used by `Auth` for callback endpoints authorization.
    pub auth_api_key: SecretString,

    /// The Public ID of the user that will be the first "Admin" within the app.
    #[serde(default)]
    pub admin_pub_id: Option<Nanoid<16, Base62Alphabet>>,
}
