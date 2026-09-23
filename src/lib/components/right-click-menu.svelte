<script lang="ts">
    let context_menu: HTMLDivElement

    export function show() {
      context_menu.style.display = "block";
    }

    export function hide() {
      context_menu.style.display = "none";
    }

    export function set_position(x: number, y: number) {
      context_menu.style.left = x.toString() + "px"
      context_menu.style.top = y.toString() + "px"
    }
</script>

<div id="context-menu" bind:this={context_menu}>
    <ul>
        <li>ITEM 1</li>
        <li>Item 2</li>
        <li>Item 3</li>
    </ul>
</div>

<style>
    #context-menu {
        display: none;
        position: absolute;
        z-index: 1;
    }
</style>
