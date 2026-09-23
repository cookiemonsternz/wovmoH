<script lang="ts">
    import { Icon } from "iconify-static";
    import { Handle, Position } from "@xyflow/svelte";
    import { DataType, OutputPinDto } from "$lib/types/data_types";

    let { outputDto }: { outputDto: OutputPinDto } = $props();
</script>

<div class="graph-output-pin-container">
    {#if outputDto.dataType == DataType.Number}
        <p id="output-name" class="graph-number-output-name">
            {outputDto.name}
        </p>
        <Handle
            class="output-handle"
            type="source"
            position={Position.Right}
            id={outputDto.name}
            style="background-color: var(--subtle); outline: 1px solid var(--text); border-radius: 50%;"
        />
    {:else if outputDto.dataType == DataType.Boolean}
        <p id="output-name" class="graph-boolean-output-name">
            {outputDto.name}
        </p>
        <Handle
            class="output-handle"
            type="source"
            position={Position.Right}
            id={outputDto.name}
            style="background-color: var(--rose); outline: 1px solid var(--text); border-radius: 50%;"
        />
    {:else if outputDto.dataType == DataType.Color}
        <p id="output-name" class="graph-boolean-output-name">
            {outputDto.name}
        </p>
        <Handle
            class="output-handle"
            type="source"
            position={Position.Right}
            id={outputDto.name}
            style="background-color: var(--gold); outline: 1px solid var(--text); border-radius: 50%;"
        />
    {/if}
</div>

<style>
    p {
        margin: 5px 10px 5px 0%;
    }

    .graph-output-pin-container {
        color: var(--subtle);

        display: flex;
        flex-direction: row;

        justify-content: flex-end;

        height: fit-content;
        min-width: 256px;
    }
</style>
