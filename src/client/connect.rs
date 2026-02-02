use endpoint_libs::model::EndpointSchema;
use eyre::eyre;
use url::Url;
use crate::client::HoneyIdClient;
use crate::enums::EndpointMethodCode;
use crate::error::HoneyIdResult;
use crate::ws::HoneyIdConnection;

impl HoneyIdClient {
    pub async fn raw_connect(&self, addr: String, header: String) -> HoneyIdResult<HoneyIdConnection> {
        HoneyIdConnection::connect(Url::parse(&addr).map_err(|e| eyre!(e))?, Some(header)).await
    }
    
    pub async fn connect_public(&self) -> HoneyIdResult<HoneyIdConnection> {
        let auth_endpoint_name = EndpointMethodCode::PublicConnect
            .schema()
            .name
            .to_lowercase();
        let header = format!("0{auth_endpoint_name}");

        let connection = HoneyIdConnection::connect(self.config.addr.clone(), Some(header)).await?;

        todo!()
    }
}
