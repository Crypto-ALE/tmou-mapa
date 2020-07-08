<script>
    import MapTile from "./MapTile.svelte";
    export let tiles = [];

    function load() {
        console.log('hey')
    }

</script>
<style>
   .map {
       grid-column: 1;
       border: solid 3px black;
       width: 768px;
       height: 768px;
       display: grid;
       row-gap: 0;
       grid-template-columns: 256px 256px 256px;
       grid-template-rows: 256px 256px 256px;
   }
</style>
<div class="map">
    {#each [0,1,2] as x }
        {#each [0,1,2] as y }
            <MapTile {x} {y} url={tiles[x][y]}/>
        {/each}
    {/each}
</div>
