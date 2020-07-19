import {getMap} from './map';
import {circleFactory} from "./circle";
import {
  Circle,
  LatLng,
  LatLngLiteral,
  LeafletMouseEvent,
  Polyline
} from "leaflet";
import {discover, getNodesAndWays, updateNodesAndWays} from './nodes';

const mapInstance = getMap('map', [49.195, 16.609], 15);

async function run() {
  const secretPhase = document.querySelector("body").dataset.secretphrase;

  document.getElementById('discover').onclick = async (e) => {
    const nodeContent = await discover(secretPhase);
    if (nodeContent.type === 'Puzzle') {
      window.open(nodeContent.data, 'new');
    } else {
      alert(nodeContent.data);
    }
  }

  let {nodes, ways, state} = await getNodesAndWays(secretPhase);
  const lines: Polyline[] = [];
  const latLng: LatLngLiteral = nodes.get(state.position);
  let currentNodeCoords: LatLng = new LatLng(latLng.lat, latLng.lng);

  mapInstance.setView(currentNodeCoords, 17);
  document.getElementById('ranking').textContent = state.ranking.toString(10);
  drawNodesAndWays(nodes, ways);

  function drawNodesAndWays(nodes, ways) {
    for (const [id, node] of nodes.entries()) {
      const clickHandler = async function (e: LeafletMouseEvent) {
        console.log("clicked on", id);
        await handleNodeClick(e.target, id);
      }
      const c = circleFactory(node, id, "blue", 2, clickHandler);

      if (currentNodeCoords.equals(node)) {
        c.setStyle({color: "salmon"});
      }
      c.addTo(mapInstance);
    }

    for (const way of ways) {
      const l = new Polyline(way, {color: "black", weight: 1, interactive: false});
      lines.push(l);
      l.bringToBack();
      l.addTo(mapInstance);
    }

  }

  async function handleNodeClick(node: Circle, nodeId) {
    mapInstance.setView(node.getLatLng(), mapInstance.getZoom());
    currentNodeCoords = node.getLatLng();
    document.getElementById('pos').textContent = node.getLatLng().toString();
    const {nodes, ways} = await updateNodesAndWays(secretPhase, nodeId);
    drawNodesAndWays(nodes, ways);
  }
}

run().then(r => console.log('Running'));
