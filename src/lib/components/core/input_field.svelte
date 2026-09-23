<script lang="ts">
    import { Icon } from "iconify-static";
    import { Handle, Position } from "@xyflow/svelte";
    import { InputFieldDto, DataType } from "$lib/types/data_types";
    import ColorPicker, { A11yVariant } from 'svelte-awesome-color-picker';

    let { inputDto }: { inputDto: InputFieldDto } = $props();

    let buttonsVisible = $state(false);
    let inputContainer: HTMLDivElement = $state() as HTMLDivElement;

    function onContainerHover() {
        buttonsVisible = true;
    }

    function onContainerUnHover() {
        buttonsVisible = false;
    }

    async function onContainerMouseDown() {
        await inputContainer.requestPointerLock();
    }

    async function onContainerMouseUp() {
        await document.exitPointerLock();
    }
</script>

<div class="graph-input-field-container">
    {#if inputDto.dataType == DataType.Number}
        <Handle
            class="input-handle"
            type="target"
            position={Position.Left}
            id={inputDto.name}
            style="background-color: var(--subtle); outline: 1px solid var(--text); border-radius: 50%;"
        />
        <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
        <div
            class="graph-number-input-container nodrag"
            onmouseenter={onContainerHover}
            onmouseleave={onContainerUnHover}
            onmousedown={onContainerMouseDown}
            onmouseup={onContainerMouseUp}
            role="list"
            bind:this={inputContainer}
        >
            {#if buttonsVisible}
                <button
                    id="left-button"
                    class="graph-number-input-button graph-number-input-button-left"
                >
                    <Icon icon="mdi:chevron-left"></Icon>
                </button>
            {:else}
                <button
                    style="opacity:0"
                    class="graph-number-input-button graph-number-input-button-left"
                >
                    <Icon icon="mdi:chevron-right"></Icon>
                </button>
            {/if}
            <p id="input-name" class="graph-number-input-name">
                {inputDto.name}
            </p>
            <p id="input-value" class="graph-number-input-value">
                {inputDto.value.value.value}
            </p>
            {#if buttonsVisible}
                <button
                    id="right-button"
                    class="graph-number-input-button graph-number-input-button-right"
                >
                    <Icon icon="mdi:chevron-right"></Icon>
                </button>
            {:else}
                <button
                    style="opacity:0"
                    class="graph-number-input-button graph-number-input-button-right"
                >
                    <Icon icon="mdi:chevron-right"></Icon>
                </button>
            {/if}
        </div>
    {:else if inputDto.dataType == DataType.Boolean}
        <Handle
            class="input-handle"
            type="target"
            position={Position.Left}
            id={inputDto.name}
            style="background-color: var(--rose); outline: 1px solid var(--text); border-radius: 50%;"
        />
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
        <div
            class="graph-boolean-input-container nodrag"
            onmouseenter={onContainerHover}
            onmouseleave={onContainerUnHover}
            role="list"
            bind:this={inputContainer}
        >
            <input class="graph-boolean-input-checkbox" type="checkbox" checked={inputDto.value.value.value as boolean}/>
            <p id="input-name" class="graph-boolean-input-name">
                {inputDto.name}
            </p>
        </div>
    {:else if inputDto.dataType == DataType.Color}
        <Handle
            class="input-handle"
            type="target"
            position={Position.Left}
            id={inputDto.name}
            style="background-color: var(--gold); outline: 1px solid var(--text); border-radius: 50%;"
        />
        <div class="graph-color-picker">
            <ColorPicker label="" position="fixed"/>
        </div>
        <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
        <p class="graph-color-name">{inputDto.name}</p>
    {/if}
</div>

<style>
    p {
        margin: 5px 0% 5px 0%;
    }

    .graph-input-field-container {
        color: var(--subtle);

        display: flex;
        flex-direction: row;

        height: fit-content;
        min-width: 256px;
    }

    /************ NUMBER INPUT **************/

    .graph-number-input-container {
        display: flex;
        flex-direction: row;

        cursor: ew-resize;

        flex-grow: 1;

        margin-left: 16px;
        margin-right: 16px;
        margin-top: 8px;
        margin-bottom: 8px;
        /*padding-left: 5%;
        padding-right: 5%;*/
        background-color: var(--overlay);
        border-radius: 8px;
    }

    .graph-number-input-container:hover {
        background-color: var(--highlight-med);
        color: var(--text);
    }

    .graph-number-input-value {
        margin-left: auto;
    }

    .graph-number-input-button {
        background-color: transparent;
        outline-color: transparent;
        box-shadow: none;
        color: var(--subtle);

        padding: 0 3px 0 3px;
        border: 0;
    }

    .graph-number-input-button:hover {
        background-color: var(--highlight-high);
        color: var(--text);
        /* background-color: black !important; */
    }

    .graph-number-input-button-left {
        border-radius: 8px 0px 0px 8px;
        margin: 0 3px 0 0;
    }

    .graph-number-input-button-right {
        border-radius: 0px 8px 8px 0px;
        margin: 0 0 0 3px;
    }

    /************ BOOLEAN INPUT **************/

    .graph-boolean-input-container {
        display: flex;
        flex-direction: row;

        flex-grow: 1;

        margin-left: 16px;
        margin-right: 16px;
        margin-top: 8px;
        margin-bottom: 8px;
        /*padding-left: 5%;
        padding-right: 5%;*/
        background-color: var(--overlay);
        border-radius: 8px;
    }

    .graph-boolean-input-checkbox {
        border: none;

        margin: 0 8px 0 8px;
        align-self: center;
    }

    /************ COLOR INPUT **************/
    .graph-color-picker {
        --cp-bg-color: var(--overlay);
        --cp-border-color: var(--highlight-med);
        --cp-text-color: var(--text);
        --cp-input-color: var(--surface);
        --cp-button-hover-color: var(--highlight-low);
        --focus-color: var(--highlight-high);
    }

    .graph-color-name {
        margin-left: 2.5%;
    }
    /************ VECTOR INPUT **************/
    /************ POINT INPUT **************/
</style>
