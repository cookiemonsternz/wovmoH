<script lang="ts">
    import { graph_json_to_dto } from "../types/data_types";
    import GraphNode from "./core/node.svelte";

    const { graph_json } = $props();
    const graphDto = $derived(graph_json_to_dto(graph_json));

    console.log(document.getElementsByClassName("graph-graph")[0]);
</script>

<div class="graph-graph" id="mygraph">
    {#each graphDto.nodes as nodeDto}
        <GraphNode />
    {/each}
</div>
