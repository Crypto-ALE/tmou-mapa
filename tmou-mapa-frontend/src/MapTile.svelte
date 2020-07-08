<script>
    export let x,y, url;
</script>
<style>
    .tile {
        width: 100%;
        height: 100%;
    }
</style>
<div class="tile" style="grid-column: {x+1}; grid-row: {y+1}">
    {#if url === undefined}
        <center><h3>None</h3></center>
    {:else}
        <img alt="tile{x}{y}" src="sample_5x5/{url}">
    {/if}
</div>
