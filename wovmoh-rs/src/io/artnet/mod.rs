use std::{
    net::{SocketAddr, ToSocketAddrs, UdpSocket},
    sync::{
        Arc, Mutex, RwLock,
        mpsc::{self, Sender},
    },
    thread,
    time::Duration,
};

use artnet_protocol::*;
use serde::{Deserialize, Serialize};

use crate::core::{graph::GraphId, node::NodeId};

pub struct WovmohArtCommand {
    pub graph: GraphId,
    pub field: usize,
    pub value: u8,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ArtnetInputFieldSubscriptionDTO {
    pub graph_id: GraphId,
    pub node_id: NodeId,
    pub field_index: usize,
    pub channel: usize,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ArtnetInputFieldSubscription {
    pub graph_id: GraphId,
    pub field: usize,
    pub channel: usize,
}

#[derive(Debug, PartialEq)]
pub struct ArtnetOutputPinSubscription {
    pub graph_id: GraphId,
    pub pin: usize,
    pub channel: i16,
}

pub struct ArtnetManager {
    tx: Sender<ArtCommand>,
    pub input_subscriptions: Arc<RwLock<Vec<ArtnetInputFieldSubscription>>>,
    pub output_subscriptions: Arc<RwLock<Vec<ArtnetOutputPinSubscription>>>,
}

impl ArtnetManager {
    pub fn new(graph_tx: Sender<WovmohArtCommand>) -> ArtnetManager {
        let (tx, rx) = mpsc::channel::<ArtCommand>();

        let input_subscriptions: Arc<RwLock<Vec<ArtnetInputFieldSubscription>>> =
            Arc::new(RwLock::new(Vec::new()));

        let input_subscriptions_copy = Arc::clone(&input_subscriptions);
        thread::spawn(move || {
            let socket = UdpSocket::bind(("0.0.0.0", 6454)).unwrap();
            socket.set_broadcast(true);

            let broadcast_addr: SocketAddr = "255.255.255.255:6454".parse().unwrap();

            socket
                .set_read_timeout(Some(Duration::from_millis(100)))
                .unwrap();

            let mut buffer = [0u8; 1024];

            loop {
                // Send mpsc recieved commands to socket
                while let Ok(command) = rx.try_recv() {
                    let buff = command.write_to_buffer().unwrap();
                    socket.send_to(&buff, &broadcast_addr).unwrap();
                }

                // Process socket recieved commands
                let maybe = socket.recv_from(&mut buffer);
                if maybe.is_err() {
                    continue;
                }

                let (length, addr) = maybe.unwrap();
                let command = ArtCommand::from_buffer(&buffer[..length]).unwrap();

                // println!("ARTNET: Recieved {:?}", command);
                match command {
                    ArtCommand::Poll(poll) => {}
                    ArtCommand::PollReply(reply) => {
                        // println!(
                        //     "Recieved poll reply from {:?}",
                        //     String::from_utf8(Vec::from(reply.long_name))
                        // );
                    }
                    ArtCommand::Output(output) => {
                        for input_subscription in input_subscriptions_copy.read().unwrap().iter() {
                            graph_tx
                                .send(WovmohArtCommand {
                                    graph: input_subscription.graph_id,
                                    field: input_subscription.field,
                                    value: output.data.as_ref()
                                        [input_subscription.channel as usize],
                                })
                                .unwrap();
                        }
                    }
                    _ => {}
                }
            }
        });

        ArtnetManager {
            tx,
            input_subscriptions,
            output_subscriptions: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn subscribe_input(&mut self, subscription: ArtnetInputFieldSubscription) {
        self.input_subscriptions.write().unwrap().push(subscription);
    }

    pub fn unsubscribe_input(&mut self, subscription: ArtnetInputFieldSubscription) {
        let index = self
            .input_subscriptions
            .read()
            .unwrap()
            .iter()
            .position(|x| *x == subscription)
            .unwrap();
        self.input_subscriptions.write().unwrap().swap_remove(index);
    }

    pub fn subscribe_output(&mut self, subscription: ArtnetOutputPinSubscription) {
        self.output_subscriptions
            .write()
            .unwrap()
            .push(subscription);
    }

    pub fn unsubscribe_output(&mut self, subscription: ArtnetOutputPinSubscription) {
        let index = self
            .output_subscriptions
            .read()
            .unwrap()
            .iter()
            .position(|x| *x == subscription)
            .unwrap();
        self.output_subscriptions
            .write()
            .unwrap()
            .swap_remove(index);
    }

    pub fn poll(&self) {
        let _ = self.tx.send(ArtCommand::Poll(Poll::default()));
    }
}
