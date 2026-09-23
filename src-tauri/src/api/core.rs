use strum::IntoEnumIterator;
// Tauri Imports
use tauri::async_runtime::Mutex;
use tauri::State;

use crate::core::node::{NodeKind, NodeUIState};
use crate::dto::graph_dto::GraphDto;
use crate::managers::graph_manager::GraphManager;

#[tauri::command]
pub async fn add_graph(state: State<'_, Mutex<GraphManager>>) -> Result<(), ()> {
    let mut graph_manager = state.lock().await;

    graph_manager.add_graph();
    graph_manager.get_graph_mut(0).add_node(
        NodeKind::ConstantColor,
        NodeUIState {
            position: (0.0, 0.0),
        },
    );
    graph_manager.get_graph_mut(0).add_node(
        NodeKind::ConstantColor,
        NodeUIState {
            position: (0.0, 0.0),
        },
    );
    graph_manager.get_graph_mut(0).connect(0, 1);

    graph_manager.get_graph_mut(0).add_node(
        NodeKind::ConstantNumber,
        NodeUIState {
            position: (0.0, 0.0),
        },
    );

    graph_manager.get_graph_mut(0).add_node(NodeKind::ConstantBoolean, NodeUIState { position: (5.0, 1.0) });

    println!("Added graph to GraphManager");

    Ok(())
}

#[tauri::command]
pub async fn get_graph_dto(
    graph_id: usize,
    state: State<'_, Mutex<GraphManager>>,
) -> Result<GraphDto, ()> {
    let graph_manager = state.lock().await;

    let graph = (*graph_manager).get_graph(graph_id);

    println!("Got graph!");

    Ok(graph.to_dto())
}

#[tauri::command]
pub async fn get_node_types(state: State<'_, Mutex<GraphManager>>) -> Result<Vec<&'static str>, ()> {
    let kinds: Vec<&'static str> = NodeKind::iter().map(|x| x.into()).collect();
    Ok(kinds)
}