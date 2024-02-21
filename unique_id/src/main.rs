use serde::{Deserialize, Serialize};

use maelstrom::{
    error::MaelstromError,
    message::{InitializationRequest, Message},
    node::MaelstromNode,
    service::Service,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum UniqueIdRequest {
    Generate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum UniqueIdResponse {
    GenerateOk { id: String },
}

struct UniqueIdNode {
    id: String,
}

impl MaelstromNode for UniqueIdNode {
    type InputPayload = UniqueIdRequest;
    type OutputPayload = UniqueIdResponse;
    type PeerPayload = ();

    fn new(init_message: &Message<InitializationRequest>) -> Self {
        let InitializationRequest::Init { id, .. } = init_message.payload();
        Self { id: id.clone() }
    }

    fn handle(
        &mut self,
        _: &Message<Self::InputPayload>,
        service: &mut Service,
    ) -> Result<Option<Self::OutputPayload>, MaelstromError>
    where
        Self: Sized,
    {
        let id = format!("id:{}:{}", self.id, service.outbox_id());
        Ok(Some(UniqueIdResponse::GenerateOk { id }))
    }
}

pub fn main() -> anyhow::Result<()> {
    Service::new().run::<UniqueIdNode>()?;
    Ok(())
}
