use serde::{Deserialize, Serialize};

use maelstrom::{
    error::MaelstromError,
    message::{InitializationRequest, Message},
    node::MaelstromNode,
    service::Service,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum EchoRequest {
    Echo { echo: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum EchoResponse {
    EchoOk { echo: String },
}

struct EchoNode;

impl MaelstromNode for EchoNode {
    type InputPayload = EchoRequest;
    type OutputPayload = EchoResponse;
    type PeerPayload = ();

    fn new(_: &Message<InitializationRequest>) -> Self {
        Self
    }

    fn handle(
        &mut self,
        message: &Message<Self::InputPayload>,
        _: &mut Service,
    ) -> Result<Option<Self::OutputPayload>, MaelstromError>
    where
        Self: Sized,
    {
        let EchoRequest::Echo { echo } = message.payload();
        Ok(Some(EchoResponse::EchoOk { echo: echo.clone() }))
    }
}

pub fn main() -> anyhow::Result<()> {
    Service::new().run::<EchoNode>()?;
    Ok(())
}
