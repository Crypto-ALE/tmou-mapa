<script>
	import axios from 'axios';
	import Map from './Map.svelte';
	import Sidebar from "./Sidebar.svelte";
	let tiles = [[],[],[]];
	function loadTiles() {
		axios.get('https://api.github.com')
				.then(function (response) {
					// handle success
					console.log(response);
					tiles = [['00.png', '01.png', '02.png'],['10.png', '11.png', '12.png'],['20.png', '21.png', '22.png']];
				})
	}
</script>

<style>
	.container {
		display: grid;
		grid-template-columns: 2;
	}
</style>

<div class="container">
	<Map {tiles}/>
	<Sidebar on:loadTiles={loadTiles}/>
</div>
