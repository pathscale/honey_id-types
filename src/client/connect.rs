use endpoint_libs::model::EndpointSchema;

use crate::client::HoneyIdClient;
use crate::enums::EndpointMethodCode;
use crate::error::HoneyIdResult;
use crate::ws::HoneyIdConnection;

fn get_auth_method_from_schema(schema: &EndpointSchema) -> String {
    schema.name.to_lowercase()
}

impl HoneyIdClient {
    pub async fn connect_public(&self) -> HoneyIdResult<HoneyIdConnection> {
        let auth_endpoint_name = EndpointMethodCode::PublicConnect.schema().name.to_lowercase();
        let header = format!("0{auth_endpoint_name}");
        
        let connection = HoneyIdConnection::connect(self.config.addr, Some(header)).await?;
        
    }
}