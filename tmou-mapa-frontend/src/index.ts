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
    const {items} = await discover(secretPhase);
    if (items.length === 0) {
      showPopup('Bohužel...', 'Na toto místo žádná šifra nevede, zkuste to jinde.');
    } else {
      const itemsToShow = items.map((item) => {
        if (item.type === 'Puzzle') {
          return `Našli jste šifru! Podívejte se na ni <a href="${items[0].url}">zde</a>.`
        }
      });
      showPopup('Hurá!', itemsToShow.join('<br />'));
    }
  }

  let {nodes, ways, state} = await getNodesAndWays(secretPhase);
  const lines: Polyline[] = [];
  const latLng: LatLngLiteral = nodes.get(state.position);
  let currentNodeCoords: LatLng = new LatLng(latLng.lat, latLng.lng);

  mapInstance.setView(currentNodeCoords, 17);
  document.getElementById('ranking').textContent = state.ranking.toString(10);
  document.getElementById('pos').textContent = currentNodeCoords.toString();
  document.getElementById('nodeId').textContent = state.position;
  drawNodesAndWays(nodes, ways);

  function showPopup(heading: string, text: string) {
    document.querySelector('.popup_text>h2').textContent = heading;
    document.querySelector('.popup_text>p').innerHTML = text;
    document.getElementById('popup').classList.remove('popup__hidden');
    document.getElementById('overlay').classList.remove('overlay__hidden');
    document.getElementById('popup').classList.add('popup__visible');
    document.getElementById('overlay').classList.add('overlay__visible');

    document.querySelector('.popup #continue').addEventListener('click', hidePopup);

    function hidePopup(e: Event) {
      document.getElementById('popup').classList.remove('popup__visible');
      document.getElementById('overlay').classList.remove('overlay__visible');
      document.getElementById('popup').classList.add('popup__hidden');
      document.getElementById('overlay').classList.add('overlay__hidden');
      e.target.removeEventListener('click', hidePopup);
    }
  }

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
    // @ts-ignore
    document.getElementById('nodeId').textContent = node.getId();
    const {nodes, ways} = await updateNodesAndWays(secretPhase, nodeId);
    drawNodesAndWays(nodes, ways);
  }
}

run().then(r => console.log('Running'));
