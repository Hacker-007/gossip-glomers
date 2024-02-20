use std::io::Write;

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

    fn new(init_message: &Message<InitializationRequest>) -> Self {
        let InitializationRequest::Init { id, .. } = init_message.payload();
        Self { id: id.clone() }
    }

    fn handle(
        &mut self,
        _: &Message<Self::InputPayload>,
        service: &mut Service,
        _: &mut impl Write
    ) -> Result<Option<Self::OutputPayload>, MaelstromError>
    where
        Self: Sized,
    {
        let id = format!("id:{}:{}", self.id, service.outbox_id());
        Ok(Some(UniqueIdResponse::GenerateOk { id }))
    }
}

pub fn main() -> anyhow::Result<()> {
    let mut stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();
    Service::new().run::<UniqueIdNode>(&mut stdin, &mut stdout)?;
    Ok(())
}
