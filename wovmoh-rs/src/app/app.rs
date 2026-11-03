use std::{
    fs::File,
    io::{BufReader, Write},
    path::PathBuf,
    str::FromStr,
    sync::mpsc::{self, Receiver},
};

use serde::{Deserialize, Serialize};

use crate::{
    core::{
        graph_manager::GraphManager,
        node::{NodeKind, NodeUIState},
    },
    dto::graph_dto::GraphDto,
    io::{
        artnet::{
            ArtnetInputFieldSubscription, ArtnetInputFieldSubscriptionDTO, ArtnetManager,
            ArtnetOutputPinSubscription, ArtnetOutputPinSubscriptionDTO, WovmohArtCommand,
        },
        midi::{
            MidiEvent, MidiInputFieldSubscription, MidiInputFieldSubscriptionDTO, MidiManager,
            MidiOutputPinSubscription, MidiOutputPinSubscriptionDTO,
        },
    },
};

pub struct App {
    pub graphs: GraphManager,
    pub midi: MidiManager,
    pub artnet: ArtnetManager,

    midi_rx: Receiver<MidiEvent>,
    artnet_rx: Receiver<WovmohArtCommand>,
}

impl App {
    pub fn new() -> App {
        let (midi_tx, midi_rx) = mpsc::channel::<MidiEvent>();
        let (artnet_tx, artnet_rx) = mpsc::channel::<WovmohArtCommand>();

        let mut app = App {
            graphs: GraphManager::new(),
            midi: MidiManager::new(midi_tx),
            artnet: ArtnetManager::new(artnet_tx),
            midi_rx,
            artnet_rx,
        };

        app.graphs.add_graph();

        MidiManager::print_available_input_ports();
        MidiManager::print_available_output_ports();

        app.artnet.poll();

        app
    }

    pub fn process_midi_events(&mut self) {
        while let Ok(event) = self.midi_rx.try_recv() {
            self.graphs.handle_midi_event(event);
        }
    }

    pub fn process_artnet_events(&mut self) {
        while let Ok(event) = self.artnet_rx.try_recv() {
            self.graphs.handle_artnet_event(event);
        }
    }

    pub fn event_loop(&mut self) {
        self.process_midi_events();
        self.process_artnet_events();

        for graph in self.graphs.iter_mut() {
            graph.process();
        }

        let _ = self.midi.send_output_events(&self.graphs);
        let _ = self.artnet.send_output_events(&self.graphs);
    }

    pub fn to_dto(&self) -> AppDTO {
        let graphs = self
            .graphs
            .iter()
            .map(|graph| {
                return graph.to_dto();
            })
            .collect();

        let midi_input_subscriptions = self
            .midi
            .input_subscriptions
            .read()
            .unwrap()
            .iter()
            .map(|midi| -> MidiInputFieldSubscriptionDTO {
                MidiInputFieldSubscriptionDTO {
                    graph_id: midi.graph,
                    node_id: self.graphs.get_graph(midi.graph).node_for_input(midi.field),
                    field_index: self
                        .graphs
                        .get_graph(midi.graph)
                        .input_index_for_input(midi.field),
                    message: midi.message.clone(),
                }
            })
            .collect();
        let midi_output_subscriptions = self
            .midi
            .output_subscriptions
            .read()
            .unwrap()
            .iter()
            .map(|midi| -> MidiOutputPinSubscriptionDTO {
                MidiOutputPinSubscriptionDTO {
                    graph_id: midi.graph,
                    node_id: self.graphs.get_graph(midi.graph).node_for_output(midi.pin),
                    pin_index: self
                        .graphs
                        .get_graph(midi.graph)
                        .output_index_for_output(midi.pin),
                    message: midi.message.clone(),
                    midi_output: midi.midi_output.clone(),
                }
            })
            .collect();
        let artnet_input_connections = self
            .artnet
            .input_subscriptions
            .read()
            .unwrap()
            .iter()
            .map(|artnet| -> ArtnetInputFieldSubscriptionDTO {
                ArtnetInputFieldSubscriptionDTO {
                    graph_id: artnet.graph_id,
                    node_id: self
                        .graphs
                        .get_graph(artnet.graph_id)
                        .node_for_input(artnet.field),
                    field_index: self
                        .graphs
                        .get_graph(artnet.graph_id)
                        .input_index_for_input(artnet.field),
                    channel: artnet.channel,
                }
            })
            .collect();
        let artnet_output_connections = self
            .artnet
            .output_subscriptions
            .read()
            .unwrap()
            .iter()
            .map(|artnet| -> ArtnetOutputPinSubscriptionDTO {
                ArtnetOutputPinSubscriptionDTO {
                    graph_id: artnet.graph_id,
                    node_id: self
                        .graphs
                        .get_graph(artnet.graph_id)
                        .node_for_output(artnet.pin),
                    pin_index: self
                        .graphs
                        .get_graph(artnet.graph_id)
                        .output_index_for_output(artnet.pin),
                    channel: artnet.channel,
                }
            })
            .collect();

        AppDTO {
            graphs,
            midi_input_subscriptions,
            midi_output_subscriptions,
            artnet_input_subscriptions: artnet_input_connections,
            artnet_output_subscriptions: artnet_output_connections,
        }
    }

    pub fn save(&self, path: PathBuf) -> Result<(), String> {
        let app_dto = self.to_dto();

        let app_as_json = serde_json::to_string(&app_dto)
            .map_err(|error| format!("Error serializing: {error}"))?;

        let mut file = File::create(&path)
            .map_err(|error| format!("Error creating file {:?}: {error}", path))?;

        file.write_all(app_as_json.as_bytes())
            .map_err(|error| format!("Error writing {:?}: {error}", path))
    }

    pub fn load(path: PathBuf) -> Result<App, String> {
        println!("Loading {:?}", path);

        let file =
            File::open(&path).map_err(|error| format!("Error opening {:?}: {error}", path))?;

        let reader = BufReader::new(file);

        let app_dto = serde_json::from_reader::<BufReader<File>, AppDTO>(reader)
            .map_err(|error| format!("Error deserializing {:?}: {error}", path))?;

        let mut app = App::new();

        for graph_dto in app_dto.graphs {
            app.graphs.add_graph();
            let graph = app.graphs.try_get_graph_mut(graph_dto.id)?;

            for node_dto in graph_dto.nodes {
                let kind = NodeKind::from_str(&node_dto.kind)
                    .map_err(|_| format!("Bad node kind: {}", node_dto.kind))?;
                graph.add_node(
                    kind,
                    NodeUIState {
                        position: node_dto.position,
                        name_override: node_dto.name_override,
                    },
                );
            }

            for connection_dto in graph_dto.connections {
                let output =
                    graph.try_output_id_for(connection_dto.from_node, connection_dto.from_index)?;
                let input =
                    graph.try_input_id_for(connection_dto.to_node, connection_dto.to_index)?;
                graph.try_connect(output, input)?;
            }
        }

        for midi_input_subscription in app_dto.midi_input_subscriptions {
            app.midi.subscribe_input(MidiInputFieldSubscription {
                graph: midi_input_subscription.graph_id,
                field: app
                    .graphs
                    .try_get_graph(midi_input_subscription.graph_id)?
                    .try_input_id_for(
                        midi_input_subscription.node_id,
                        midi_input_subscription.field_index,
                    )?,
                message: midi_input_subscription.message,
            });
        }

        for midi_output_subscription in app_dto.midi_output_subscriptions {
            app.midi.subscribe_output(MidiOutputPinSubscription {
                graph: midi_output_subscription.graph_id,
                pin: app
                    .graphs
                    .try_get_graph(midi_output_subscription.graph_id)?
                    .try_output_id_for(
                        midi_output_subscription.node_id,
                        midi_output_subscription.pin_index,
                    )?,
                message: midi_output_subscription.message,
                midi_output: midi_output_subscription.midi_output,
            });
        }

        for artnet_input_subscription in app_dto.artnet_input_subscriptions {
            app.artnet.subscribe_input(ArtnetInputFieldSubscription {
                graph_id: artnet_input_subscription.graph_id,
                field: app
                    .graphs
                    .try_get_graph(artnet_input_subscription.graph_id)?
                    .try_input_id_for(
                        artnet_input_subscription.node_id,
                        artnet_input_subscription.field_index,
                    )?,
                channel: artnet_input_subscription.channel,
            })?;
        }

        for artnet_output_subscription in app_dto.artnet_output_subscriptions {
            app.artnet.subscribe_output(ArtnetOutputPinSubscription {
                graph_id: artnet_output_subscription.graph_id,
                pin: app
                    .graphs
                    .try_get_graph(artnet_output_subscription.graph_id)?
                    .try_output_id_for(
                        artnet_output_subscription.node_id,
                        artnet_output_subscription.pin_index,
                    )?,
                channel: artnet_output_subscription.channel,
            })?;
        }

        Ok(app)
    }

    pub fn load_override(&mut self, path: PathBuf) -> Result<(), String> {
        let new_app = App::load(path)?;
        *self = new_app;
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct AppDTO {
    graphs: Vec<GraphDto>,
    midi_input_subscriptions: Vec<MidiInputFieldSubscriptionDTO>,
    midi_output_subscriptions: Vec<MidiOutputPinSubscriptionDTO>,
    artnet_input_subscriptions: Vec<ArtnetInputFieldSubscriptionDTO>,
    artnet_output_subscriptions: Vec<ArtnetOutputPinSubscriptionDTO>,
}
