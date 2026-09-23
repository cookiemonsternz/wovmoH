<script lang="ts">
    // invoke (call rust from frontend)
    import { invoke } from "@tauri-apps/api/core";
    import { mount } from 'svelte';
    // Svelteflow
    import {
        Background,
        BackgroundVariant,
        Controls,
        type EdgeTypes,
        SvelteFlow,
    } from "@xyflow/svelte";
    // Custom edge
    import CustomEdge from "./SvelteFlow/CustomEdge.svelte";
    // Custom nodes
    import GraphNode from "$lib/components/core/node.svelte";
    import NodePicker from "$lib/components/node-picker.svelte";
    // Base style
    import "@xyflow/svelte/dist/base.css";
    import "$lib/themes/rose-pine-moon.css";
    import "$lib/flow-style.css";
    // Conversion of json to dto
    import { graph_json_to_dto, NodeDataDto } from "$lib/types/data_types";
    import type { GraphJsonDto, NodeDto } from "$lib/types/data_types";
    import { preventDefault } from "svelte/legacy";
    import RightClickMenu from "$lib/components/right-click-menu.svelte";

    // Svelteflow
    const edgeTypes: EdgeTypes = {
        "custom-edge": CustomEdge,
    };

    const nodeTypes = {
        "graph-node": GraphNode,
    };

    let nodes = $state.raw([]);

    let edges = $state.raw([]);

    function addGraph() {
        console.log("Adding Graph");

        invoke("add_graph").then(() => {
            console.log("Success");
        });

        invoke("get_graph_dto", { graphId: 0 }).then((graphJsonDto) => {
            console.log("Got graph");
            console.log(graphJsonDto);
            const graphDto = graph_json_to_dto(graphJsonDto as GraphJsonDto);
            console.log(graphDto);
            nodes = graphDto.nodes;
            console.log($state.snapshot(nodes));
        });
    }

    let right_click_menu: RightClickMenu;

    function getNodeTypes() {
        console.log("Getting node types");
        invoke("get_node_types").then((node_types) => {
            console.log(node_types);
        });
    }

    function on_ctx(e: MouseEvent) {
      console.log(e.x)
      right_click_menu.show()
      right_click_menu.set_position(e.x, e.y)
      e.preventDefault();
    }

    function hide_menu(e: MouseEvent) {
      right_click_menu.hide()
    }
</script>

`
<svelte:head>
    <link rel="stylesheet" href="/style/svelte-flow.css" />
</svelte:head>

<main class="container" style="width: 100vw; height: 100vh">
    <RightClickMenu bind:this={right_click_menu}></RightClickMenu>
    <button id="butttooon" onclick={addGraph}>Add Graph</button>
    <button id="button" onclick={getNodeTypes}>Get Node Types</button>
    <NodePicker></NodePicker>
    <SvelteFlow
        {edgeTypes}
        defaultEdgeOptions={{ type: "custom-edge" }}
        bind:nodes
        bind:edges
        {nodeTypes}
        id="SvelteFlowGraph"
        oncontextmenu={on_ctx}
        onclick={hide_menu}
    >
        <Background patternColor="#6e6a86" variant={BackgroundVariant.Dots} />
        <Controls />
    </SvelteFlow>
</main>

<style>
    @font-face {
        font-family: jetbrains-mono;
        src: url("$lib/assets/fonts/webfonts/jetbrains-mono-latin-500-normal.woff")
            format("woff");
    }

    @font-face {
        font-family: jetbrains-mono;
        src: url("$lib/assets/fonts/webfonts/jetbrains-mono-latin-500-italic.woff")
            format("woff");
        font-style: italic;
    }

    :global(:root) {
        font-family: jetbrains-mono;
        color: var(--text);
    }

    :global(.svelte-flow__pane.draggable) {
        cursor: default;
    }
</style>
