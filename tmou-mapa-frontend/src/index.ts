import {getMap} from './map';
import {circleFactory} from "./circle";
import {
  Circle,
  LatLng,
  LatLngLiteral,
  LeafletMouseEvent,
  Polyline
} from "leaflet";
import {Item} from './types';
import {discover, getTeamState, moveTeam} from './nodes';

const mapInstance = getMap('map', [49.195, 16.609], 15);

async function run() {
  const secretPhase = document.querySelector("body").dataset.secretphrase;

  document.getElementById('discover').onclick = async () => {
    const {items} = await discover(secretPhase);
    console.debug('items:', items);
    if (items.length === 0) {
      showPopup('Bohužel...', 'Na toto místo žádná šifra nevede, zkuste to jinde.');
    } else {
      const itemsToShow = items.map((item) => {
        switch (item.type) {
          case 'Puzzle': {
            return `Našli jste šifru! Podívejte se na ni <a href="${items[0].url}">zde</a>.`
          }
          case 'badge': {
            return 'Řešení je správně, získali jste za něj odznáček.';
          }
        }
      });
      showPopup('Hurá!', itemsToShow.join('<br />'), Math.max(items.map(item => item.level)));
    }
    const newTeamState = await getTeamState(secretPhase);
    drawInventory(newTeamState.items.items);
  }

  let {nodes, ways, state, items} = await getTeamState(secretPhase);
  const lines: Polyline[] = [];
  const latLng: LatLngLiteral = nodes.get(state.position)!.latLng;
  let currentNodeCoords: LatLng = new LatLng(latLng.lat, latLng.lng);

  mapInstance.setView(currentNodeCoords, 17);
  document.getElementById('pos').textContent = currentNodeCoords.toString();
  document.getElementById('nodeId').textContent = state.position;
  drawInventory(items.items);
  drawNodesAndWays(nodes, ways);

  function showPopup(heading: string, text: string, lvl=null) {
    document.querySelector('.popup_text>h2').textContent = heading;
    document.querySelector('.popup_text>p').innerHTML = text;
    const badgeClass = lvl ? `lvl${lvl}` : 'shrug';
    document.querySelector('#popup .large_badge').classList.add(badgeClass);
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

  function showBadge(level: number, label: String) {
    document.getElementById('badges').innerHTML += `
          <div class="badge lvl${level}">
            <span class="time">17:08</span>
            <span class="label">${label}</span>
          </div>
          `;
  }

  function showPuzzle(level: number, url: String) {
    document.querySelector('#puzzles>ul').innerHTML += `<li><a href="${url}">Level ${level}</a>`
  }

  function drawNodesAndWays(nodes, ways) {
    for (const [id, nodeInfo] of nodes.entries()) {
      const node = nodeInfo.latLng;
      const clickHandler = async function (e: LeafletMouseEvent) {
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
    const {nodes, ways, items} = await moveTeam(secretPhase, nodeId);
    drawInventory(items.items);
    drawNodesAndWays(nodes, ways);
  }

  function drawInventory(items: Item[]) {
    document.getElementById('badges').innerHTML = '';
    document.querySelector('#puzzles>ul').innerHTML = '';
    for (const item of items) {
      switch (item.type) {
        case "puzzles": {
          showPuzzle(item.level, item.url);
          break;
        }
        case "badge": {
          showBadge(item.level, item.description.slice(-2));
          break;
        }
      }
    }
  }

}



run().then(r => console.log('Running'));
