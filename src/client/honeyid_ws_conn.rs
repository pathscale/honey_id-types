use endpoint_libs::libs::ws::{WsClient, WsClientBuilder, WsResponseGeneric, WsVersionMode};
use eyre::bail;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::enums::HoneyEndpointMethodCode;
use crate::types::error::{HoneyIdError, HoneyIdResult};

pub struct HoneyIdConnection {
    client: WsClient,
}

impl std::fmt::Debug for HoneyIdConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HoneyIdConnection").finish_non_exhaustive()
    }
}

impl HoneyIdConnection {
    pub async fn connect(addr: &Url, auth: Option<&str>) -> HoneyIdResult<HoneyIdConnection> {
        let (client, _) = WsClientBuilder::new()
            .mode(WsVersionMode::Auto)
            .protocol_header(auth.unwrap_or(""))
            .build(addr.as_str())
            .await
            .map_err(eyre::Report::from)?;
        Ok(HoneyIdConnection { client })
    }

    /// Used specifically for [HoneyEndpointMethodCode] endpoints that are defined within this project
    pub async fn send_request<T: Serialize>(&mut self, method: HoneyEndpointMethodCode, params: T) -> eyre::Result<()> {
        self.send_request_raw(method as u32, params).await
    }

    /// Used for compatibility with code that doesn't call HoneyEndpointMethodCode endpoints
    pub async fn send_request_raw<T: Serialize>(&mut self, method: u32, params: T) -> eyre::Result<()> {
        self.client.send_req(method, params).await
    }

    pub async fn receive_response<T>(&mut self) -> eyre::Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let raw = self.client.recv_raw().await?;
        let resp: WsResponseGeneric<T> = serde_json::from_value(raw)?;
        match resp {
            WsResponseGeneric::Immediate(resp) => Ok(resp.params),
            WsResponseGeneric::Error(err) => {
                bail!(HoneyIdError::new(
                    endpoint_libs::libs::error_code::ErrorCode::new(err.code),
                    err.params.to_string()
                ))
            }
            _ => bail!("Unexpected response from server"),
        }
    }
}
