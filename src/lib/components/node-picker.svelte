<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    let visible = false;

    let node_types: [String];
    invoke("get_node_types").then((types) => {
        node_types = types as [String];
    });
    let mouseX: Number, mouseY: Number;

    document.addEventListener("mousemove", (ev) => {
        (mouseX = ev.clientX), (mouseY = ev.clientY);
    });

    function open() {
        set_position(mouseX, mouseY);
        visible = true;
    }
    function close() {
        visible = false;
    }
    function set_position(x: Number, y: Number) {
        let div = document.getElementsByClassName(
            "node-picker",
        )[0] as HTMLDivElement;
        div.style.top = x.toString() + "px";
        div.style.left = y.toString() + "px";
    }
</script>

{#if visible}
    <div class="node-picker">
        {#each node_types as node_type}
            <button>{node_type}</button>
        {/each}
    </div>
{/if}

<style>
    .node-picker {
        display: flex;
        flex-direction: column;
        position: fixed;
        top: 50px;
        left: 50px;
        z-index: 16;
        visibility: hidden;
    }
</style>
