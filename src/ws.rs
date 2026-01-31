use endpoint_libs::libs::error_code::ErrorCode;
use endpoint_libs::libs::ws::WsResponseError;
use eyre::bail;
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::handshake::client::Request;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};
use url::Url;

use crate::enums::EndpointMethodCode;
use crate::error::{HoneyIdError, HoneyIdResult};

pub struct HoneyIdConnection {
    stream: WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>,
}

impl HoneyIdConnection {
    pub async fn connect(addr: Url, auth: Option<String>) -> HoneyIdResult<HoneyIdConnection> {
        let mut request = Request::builder()
            .uri(addr.to_string())
            .method("GET")
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")
            .header("Sec-WebSocket-Version", "13");
        if let Some(header) = auth {
            request = request.header("Sec-WebSocket-Protocol", header)
        }
        let request = request.body(()).expect("headers should be valid");

        let (stream, _) = connect_async(request).await.map_err(eyre::Report::from)?;
        Ok(HoneyIdConnection { stream })
    }

    pub async fn send_request<T: Serialize>(
        &mut self,
        method: EndpointMethodCode,
        params: T,
    ) -> eyre::Result<()> {
        #[derive(Serialize, Deserialize, Debug)]
        #[serde(tag = "seq", rename = "1")]
        struct ApiMessage<T> {
            method: u32,
            params: T,
        }

        let json = serde_json::to_string(&ApiMessage {
            method: method as u32,
            params,
        })?;

        self.stream.send(Message::Text(json.into())).await?;

        Ok(())
    }

    pub async fn receive_response<T>(&mut self) -> eyre::Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let response: Value = match self.stream.next().await {
            Some(Ok(msg)) => {
                let text = msg.into_text()?;
                serde_json::from_str(&text)?
            }
            Some(Err(e)) => bail!("WebSocket error: {e}"),
            None => bail!("WebSocket stream closed unexpectedly"),
        };

        if let Some(value) = response
            .get("params")
            .cloned()
            .and_then(|p| serde_json::from_value::<T>(p).ok())
        {
            Ok(value)
        } else if let Ok(err) = serde_json::from_value::<WsResponseError>(response.clone()) {
            bail!(HoneyIdError::new(
                err.code
                    .try_into()
                    .map(ErrorCode::new)
                    .unwrap_or(ErrorCode::INTERNAL_ERROR),
                err.params.to_string()
            ));
        } else {
            bail!("Invalid response received from server: {response}");
        }
    }
}
