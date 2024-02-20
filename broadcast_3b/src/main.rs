use std::{
    collections::{HashMap, HashSet},
    io::Write,
};

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
    Gossip {
        messages: HashSet<usize>,
    },
    GossipOk {
        messages: HashSet<usize>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum BroadcastResponse {
    BroadcastOk,
    ReadOk { messages: HashSet<usize> },
    TopologyOk,
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
        output: &mut impl Write,
    ) -> Result<Option<Self::OutputPayload>, MaelstromError>
    where
        Self: Sized,
    {
        match message.payload() {
            BroadcastRequest::Broadcast { message: value } => {
                self.values.insert(*value);
                for neighbor in &self.network {
                    service
                        .send_to(
                            self.id.clone(),
                            neighbor.clone(),
                            BroadcastRequest::Gossip {
                                messages: self.values.clone(),
                            },
                        )
                        .write_to(output)?;
                }

                Ok(Some(BroadcastResponse::BroadcastOk))
            }
            BroadcastRequest::Read => Ok(Some(BroadcastResponse::ReadOk {
                messages: self.values.clone(),
            })),
            BroadcastRequest::Topology { .. } => Ok(Some(BroadcastResponse::TopologyOk)),
            BroadcastRequest::Gossip { messages } => {
                let previous_messages = self.values.clone();
                self.values.extend(messages);
                Ok(Some(BroadcastResponse::GossipOk {
                    messages: previous_messages,
                }))
            }
            BroadcastRequest::GossipOk { messages } => {
                self.values.extend(messages);
                Ok(None)
            }
        }
    }
}

pub fn main() -> anyhow::Result<()> {
    let mut stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();
    Service::new().run::<BroadcastNode>(&mut stdin, &mut stdout)?;
    Ok(())
}
