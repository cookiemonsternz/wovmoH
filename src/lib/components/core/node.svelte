<script lang="ts">
    import {
        Handle,
        Position,
        useSvelteFlow,
        type NodeProps,
    } from "@xyflow/svelte";

    import InputField from "./input_field.svelte";
    import OutputPin from "./output_pin.svelte";
    import {
        NodeDataDto,
        InputFieldDto,
        OutputPinDto,
    } from "$lib/types/data_types";

    let { id, data }: NodeProps = $props();
    let inputs: InputFieldDto[] = $derived(data.inputs) as InputFieldDto[];
    let outputs: OutputPinDto[] = $derived(data.outputs) as OutputPinDto[];

    let { updateNodeData } = useSvelteFlow();
    
    $effect: (() => {
        console.log(data)
    }) ()
</script>

<div class="graph-node">
    <div>
        <div class="graph-node-title-container drag-handle custom-drag-handle">
            <p class="graph-node-title">{data.name}</p>
        </div>

        {#each outputs as output}
            <OutputPin outputDto={output} />
        {/each}
        <hr>
        {#each inputs as input}
            <InputField inputDto={input} />
        {/each}
    </div>
</div>

<style>
    .graph-node {
        margin: 0%;

        cursor: default;
    }

    .graph-node-title-container {
        cursor: grab;

        margin: 0%;
        background-color: var(--overlay);

        border-radius: 16px 16px 0px 0px;
    }

    .graph-node-title {
        margin: 0% 8px;
        padding: 4px 8px;

        font-weight: bold;
    }

    hr {
        margin: 0;
        color: var(--highlight-med);
        border-style: solid;
    }
</style>