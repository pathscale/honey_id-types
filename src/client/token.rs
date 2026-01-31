use uuid::Uuid;

use crate::HoneyIdClient;
use crate::tables::token::TokenRow;
use crate::types::entity::UserPublicId;

impl HoneyIdClient {
    pub fn store_token(&self, token: Uuid, user_pub_id: UserPublicId) -> eyre::Result<()> {
        let row = TokenRow {
            id: self.token_table.get_next_pk().into(),
            public_id: user_pub_id,
            token,
        };
        self.token_table.insert(row)?;

        Ok(())
    }
}
