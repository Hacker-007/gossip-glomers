use std::{
    collections::{HashMap, HashSet},
    sync::{self, mpsc::Receiver},
    time::Duration,
};

use rand::prelude::*;
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
}

struct BroadcastNode {
    id: String,
    values: HashSet<usize>,
    network: Vec<String>,
    seen_values: HashMap<String, HashSet<usize>>,
    rx: Receiver<()>,
}

impl BroadcastNode {
    fn gossip_to(&self, service: &mut Service, peer: &str) -> Result<(), MaelstromError> {
        let known_to_peer = &self.seen_values[peer];
        let (already_known, mut must_notify): (HashSet<_>, HashSet<_>) = self
            .values
            .iter()
            .copied()
            .partition(|message| known_to_peer.contains(message));

        let mut rng = rand::thread_rng();
        let additional_cap = (must_notify.len() * 50 / 100) as u32;
        must_notify.extend(already_known.iter().filter(|_| {
            rng.gen_ratio(
                additional_cap.min(already_known.len() as u32),
                already_known.len() as u32,
            )
        }));

        service.peer_rpc(
            self.id.clone(),
            peer.to_string(),
            PeerPayload::Gossip {
                // messages: self.values.clone(),
                messages: must_notify,
            },
        )
    }
}

impl MaelstromNode for BroadcastNode {
    type InputPayload = BroadcastRequest;
    type OutputPayload = BroadcastResponse;
    type PeerPayload = PeerPayload;

    fn new(init_message: &Message<InitializationRequest>) -> Self {
        let InitializationRequest::Init { id, neighbors } = init_message.payload();
        let network = neighbors.clone();
        let seen_values = network
            .iter()
            .map(|peer| (peer.clone(), HashSet::new()))
            .collect();

        let (tx, rx) = sync::mpsc::channel();
        std::thread::spawn(move || loop {
            std::thread::sleep(Duration::from_millis(450));
            if let Err(sync::mpsc::SendError(_)) = tx.send(()) {
                break;
            }
        });

        Self {
            id: id.clone(),
            values: HashSet::new(),
            network,
            seen_values,
            rx,
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
        let response = match message.payload() {
            BroadcastRequest::Broadcast { message: value } => {
                self.values.insert(*value);
                Ok(Some(BroadcastResponse::BroadcastOk))
            }
            BroadcastRequest::Read => Ok(Some(BroadcastResponse::ReadOk {
                messages: self.values.clone(),
            })),
            BroadcastRequest::Topology { .. } => Ok(Some(BroadcastResponse::TopologyOk)),
        };

        if let Ok(_) = self.rx.try_recv() {
            self.network
                .iter()
                .map(|peer| self.gossip_to(service, peer))
                .collect::<Result<Vec<_>, _>>()?;
        }

        response
    }

    fn handle_peer(
        &mut self,
        message: &Message<Self::PeerPayload>,
        _: &mut Service,
    ) -> Result<Option<Self::PeerPayload>, MaelstromError> {
        match message.payload() {
            PeerPayload::Gossip { messages } => {
                self.values.extend(messages);
                self.seen_values
                    .get_mut(message.src())
                    .expect("message came from known neighbor")
                    .extend(messages.iter());

                Ok(None)
            }
        }
    }
}

pub fn main() -> anyhow::Result<()> {
    Service::new().run::<BroadcastNode>()?;
    Ok(())
}
