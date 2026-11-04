use std::{
    net::{SocketAddr, UdpSocket},
    sync::{
        Arc, RwLock,
        mpsc::{self, Sender},
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use artnet_protocol::*;
use serde::{Deserialize, Serialize};

use crate::core::{graph::GraphId, graph_manager::GraphManager, node::NodeId};

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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ArtnetOutputPinSubscriptionDTO {
    pub graph_id: GraphId,
    pub node_id: NodeId,
    pub pin_index: usize,
    pub channel: usize,
}

#[derive(Debug, PartialEq)]
pub struct ArtnetOutputPinSubscription {
    pub graph_id: GraphId,
    pub pin: usize,
    pub channel: usize,
}

pub struct ArtnetManager {
    tx: Sender<ArtCommand>,
    pub input_subscriptions: Arc<RwLock<Vec<ArtnetInputFieldSubscription>>>,
    pub output_subscriptions: Arc<RwLock<Vec<ArtnetOutputPinSubscription>>>,
    shutdown_tx: Option<Sender<()>>,
    worker: Option<JoinHandle<()>>,
}

impl ArtnetManager {
    pub fn new(graph_tx: Sender<WovmohArtCommand>) -> ArtnetManager {
        let (tx, rx) = mpsc::channel::<ArtCommand>();
        let (shutdown_tx, shutdown_rx) = mpsc::channel::<()>();

        let input_subscriptions: Arc<RwLock<Vec<ArtnetInputFieldSubscription>>> =
            Arc::new(RwLock::new(Vec::new()));

        let input_subscriptions_copy = Arc::clone(&input_subscriptions);
        let worker = thread::spawn(move || {
            #[cfg(test)]
            let socket = UdpSocket::bind(("0.0.0.0", 0)).unwrap();
            #[cfg(not(test))]
            let socket = UdpSocket::bind(("0.0.0.0", 6454)).unwrap();
            socket.set_broadcast(true).unwrap();

            let broadcast_addr: SocketAddr = "255.255.255.255:6454".parse().unwrap();

            socket
                .set_read_timeout(Some(Duration::from_millis(100)))
                .unwrap();

            let mut buffer = [0u8; 1024];

            loop {
                if shutdown_rx.try_recv().is_ok() {
                    return;
                }

                // Send mpsc recieved commands to socket
                match rx.try_recv() {
                    Ok(command) => {
                        let buff = command.write_to_buffer().unwrap();
                        socket.send_to(&buff, &broadcast_addr).unwrap();
                    }
                    Err(mpsc::TryRecvError::Empty) => {}
                    Err(mpsc::TryRecvError::Disconnected) => {
                        println!("ArtnetManage Sender dropped, shutting down");
                        return;
                    }
                }

                // Process socket recieved commands
                let maybe = socket.recv_from(&mut buffer);
                if maybe.is_err() {
                    continue;
                }

                let (length, _addr) = maybe.unwrap();
                let command = ArtCommand::from_buffer(&buffer[..length]).unwrap();

                // println!("ARTNET: Recieved {:?}", command);
                match command {
                    ArtCommand::Poll(_poll) => {}
                    ArtCommand::PollReply(_reply) => {
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
            shutdown_tx: Some(shutdown_tx),
            worker: Some(worker),
        }
    }

    pub fn subscribe_input(
        &mut self,
        subscription: ArtnetInputFieldSubscription,
    ) -> std::result::Result<(), String> {
        self.input_subscriptions
            .write()
            .map_err(|_| "Could not write to input subscriptions (mutex poisoned?)".to_string())?
            .push(subscription);
        Ok(())
    }

    pub fn unsubscribe_input(
        &mut self,
        subscription: ArtnetInputFieldSubscription,
    ) -> std::result::Result<(), String> {
        let index = match self
            .input_subscriptions
            .read()
            .map_err(|_| "Coult not read input subscriptions (mutex poisoned?)".to_string())?
            .iter()
            .position(|x| *x == subscription)
        {
            Some(pos) => pos,
            None => return Err(format!("Could not find subscription {:?}", subscription)),
        };

        self.input_subscriptions
            .write()
            .map_err(|_| "Could not write to input subscriptions (mutex poisoned?)".to_string())?
            .swap_remove(index);
        Ok(())
    }

    pub fn subscribe_output(
        &mut self,
        subscription: ArtnetOutputPinSubscription,
    ) -> std::result::Result<(), String> {
        self.output_subscriptions
            .write()
            .map_err(|_| "Could not write to output subscriptions (mutex poisoned?)".to_string())?
            .push(subscription);

        Ok(())
    }

    pub fn unsubscribe_output(
        &mut self,
        subscription: ArtnetOutputPinSubscription,
    ) -> std::result::Result<(), String> {
        let index = match self
            .output_subscriptions
            .read()
            .map_err(|_| "Coult not read output subscriptions (mutex poisoned?)".to_string())?
            .iter()
            .position(|x| *x == subscription)
        {
            Some(pos) => pos,
            None => return Err(format!("Could not find subscription {:?}", subscription)),
        };

        self.output_subscriptions
            .write()
            .map_err(|_| "Could not write to output subscriptions (mutex poisoned?)".to_string())?
            .swap_remove(index);

        Ok(())
    }

    pub fn poll(&self) {
        let _ = self.tx.send(ArtCommand::Poll(Poll::default()));
    }

    pub fn send_output_events(&mut self, graphs: &GraphManager) -> std::result::Result<(), String> {
        let mut data: Vec<u8> = Vec::new();
        data.resize(512, 0);
        for output_subscription in self
            .output_subscriptions
            .read()
            .map_err(|_| "Could not read output subscriptions (mutex poisoned?)".to_string())?
            .iter()
        {
            let graph = graphs.try_get_graph(output_subscription.graph_id)?;
            let pin = graph.get_output(output_subscription.pin);
            let value = pin.value.clone();

            match value {
                crate::types::data_type::DataValue::Number(x) => {
                    data[output_subscription.channel - 1] = x as u8
                }
                crate::types::data_type::DataValue::Boolean(x) => {
                    if x {
                        data[output_subscription.channel - 1] = 255u8;
                    } else {
                        data[output_subscription.channel - 1] = 0u8;
                    }
                }
                crate::types::data_type::DataValue::Color(x) => {
                    data[output_subscription.channel - 1] = (x.r() * 256.0) as u8;
                    data[output_subscription.channel - 0] = (x.g() * 256.0) as u8;
                    data[output_subscription.channel + 1] = (x.b() * 256.0) as u8;
                }
                _ => todo!(),
            };
        }

        let output = Output {
            data: PaddedData::from(data),
            ..Default::default()
        };
        println!("{:?}", output.data);

        self.tx
            .send(ArtCommand::Output(output))
            .map_err(|_| "Could not send command to channel".to_string())?;

        Ok(())
    }
}

impl Drop for ArtnetManager {
    fn drop(&mut self) {
        let _ = self.shutdown_tx.take().map(|tx| tx.send(()));
        if let Some(worker) = self.worker.take() {
            worker.join().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::io::artnet::WovmohArtCommand;

    #[test]
    fn test_artnet_manager_init() {
        use crate::io::artnet::ArtnetManager;
        use std::sync::mpsc;

        let (tx, _rx) = mpsc::channel::<WovmohArtCommand>();
        ArtnetManager::new(tx);
    }
}
