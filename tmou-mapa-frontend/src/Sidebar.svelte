<script>
    import {createEventDispatcher} from "svelte";

    const dispatch = createEventDispatcher();
    function load() {
        console.log('Sidebar')
        dispatch('loadTiles')
    }
</script>

<style>
    .menu {
        grid-column: 2;
    }
</style>

<div class="menu">
<button on:click={load}>
    Load Tiles
</button>
</div>
