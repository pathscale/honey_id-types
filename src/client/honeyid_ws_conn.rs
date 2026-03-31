use endpoint_libs::libs::ws::{WsClient, WsResponseGeneric};
use eyre::bail;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::enums::HoneyEndpointMethodCode;
use crate::types::error::{HoneyIdError, HoneyIdResult};

#[derive(Debug)]
pub struct HoneyIdConnection {
    client: WsClient,
}

impl HoneyIdConnection {
    pub async fn connect(addr: &Url, auth: Option<&str>) -> HoneyIdResult<HoneyIdConnection> {
        let client = WsClient::new(addr.as_str(), auth.unwrap_or(""), None)
            .await
            .map_err(eyre::Report::from)?;
        Ok(HoneyIdConnection { client })
    }

    /// Used specifically for [HoneyEndpointMethodCode] endpoints that are defined within this project
    pub async fn send_request<T: Serialize>(
        &mut self,
        method: HoneyEndpointMethodCode,
        params: T,
    ) -> eyre::Result<()> {
        self.send_request_raw(method as u32, params).await
    }

    /// Used for compatibility with code that doesn't call HoneyEndpointMethodCode endpoints
    pub async fn send_request_raw<T: Serialize>(
        &mut self,
        method: u32,
        params: T,
    ) -> eyre::Result<()> {
        self.client.send_req(method, params).await
    }

    pub async fn receive_response<T>(&mut self) -> eyre::Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let raw = self.client.recv_raw().await?;
        match raw {
            WsResponseGeneric::Immediate(resp) => Ok(serde_json::from_value(resp.params)?),
            WsResponseGeneric::Error(err) => {
                bail!(HoneyIdError::new(
                    endpoint_libs::libs::error_code::ErrorCode::new(err.code),
                    err.params.to_string()
                ))
            }
            other => bail!("Unexpected response from server: {:?}", other),
        }
    }
}
