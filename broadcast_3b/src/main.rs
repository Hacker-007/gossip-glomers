use std::collections::{HashMap, HashSet};

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
    ReadOk { messages: HashSet<usize> },
    TopologyOk,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum PeerPayload {
    Gossip { messages: HashSet<usize> },
    GossipOk { messages: HashSet<usize> },
}

struct BroadcastNode {
    id: String,
    values: HashSet<usize>,
    network: Vec<String>,
}

impl MaelstromNode for BroadcastNode {
    type InputPayload = BroadcastRequest;
    type OutputPayload = BroadcastResponse;
    type PeerPayload = PeerPayload;

    fn new(init_message: &Message<InitializationRequest>) -> Self {
        let InitializationRequest::Init { id, neighbors } = init_message.payload();
        let network = neighbors
            .iter()
            .filter(|&neighbor| neighbor != id)
            .cloned()
            .collect::<Vec<_>>();

        Self {
            id: id.clone(),
            values: HashSet::new(),
            network,
        }
    }

    fn handle(
        &mut self,
        message: &Message<Self::InputPayload>,
        service: &mut Service,
    ) -> Result<Option<Self::OutputPayload>, MaelstromError>
    where
        Self: Sized,
    {
        match message.payload() {
            BroadcastRequest::Broadcast { message: value } => {
                self.values.insert(*value);
                for neighbor in &self.network {
                    service.rpc(
                        self.id.clone(),
                        neighbor.clone(),
                        PeerPayload::Gossip {
                            messages: self.values.clone(),
                        },
                    )?;
                }

                Ok(Some(BroadcastResponse::BroadcastOk))
            }
            BroadcastRequest::Read => Ok(Some(BroadcastResponse::ReadOk {
                messages: self.values.clone(),
            })),
            BroadcastRequest::Topology { .. } => Ok(Some(BroadcastResponse::TopologyOk)),
        }
    }

    fn handle_peer(
        &mut self,
        message: &Message<Self::PeerPayload>,
        _: &mut Service,
    ) -> Result<Option<Self::PeerPayload>, MaelstromError> {
        match message.payload() {
            PeerPayload::Gossip { messages } => {
                let previous_messages = self.values.clone();
                self.values.extend(messages);
                Ok(Some(PeerPayload::GossipOk { messages: previous_messages }))
            },
            PeerPayload::GossipOk { messages } => {
                self.values.extend(messages);
                Ok(None)
            },
        }
    }
}

pub fn main() -> anyhow::Result<()> {
    Service::new().run::<BroadcastNode>()?;
    Ok(())
}
