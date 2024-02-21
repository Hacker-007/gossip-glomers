use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use maelstrom::{
    error::MaelstromError,
    message::{InitializationRequest, Message},
    node::MaelstromNode,
    service::Service,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum BroadcastRequest {
    Broadcast {
        message: usize,
    },
    Read,
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum BroadcastResponse {
    BroadcastOk,
    ReadOk { messages: Vec<usize> },
    TopologyOk,
}

struct BroadcastNode {
    values: Vec<usize>,
}

impl MaelstromNode for BroadcastNode {
    type InputPayload = BroadcastRequest;
    type OutputPayload = BroadcastResponse;
    type PeerPayload = ();

    fn new(_: &Message<InitializationRequest>) -> Self {
        Self { values: vec![] }
    }

    fn handle(
        &mut self,
        message: &Message<Self::InputPayload>,
        _: &mut Service,
    ) -> Result<Option<Self::OutputPayload>, MaelstromError>
    where
        Self: Sized,
    {
        match message.payload() {
            BroadcastRequest::Broadcast { message } => {
                self.values.push(*message);
                Ok(Some(BroadcastResponse::BroadcastOk))
            }
            BroadcastRequest::Read => Ok(Some(BroadcastResponse::ReadOk {
                messages: self.values.clone(),
            })),
            BroadcastRequest::Topology { .. } => Ok(Some(BroadcastResponse::TopologyOk)),
        }
    }
}

pub fn main() -> anyhow::Result<()> {
    Service::new().run::<BroadcastNode>()?;
    Ok(())
}
