import {getMap} from './map';
import {circleFactory} from "./circle";
import {
  Circle,
  LatLng,
  LatLngLiteral,
  LeafletMouseEvent,
  Polyline
} from "leaflet";
import {Item, Node, way} from './types';
import {discover, getTeamState, moveTeam} from './nodes';

const mapInstance = getMap('map', [49.195, 16.609], 15);

async function run() {
  const secretPhase = document.querySelector("body").dataset.secretphrase;
  const renderedNodes = new Map<string, Circle>();
  const renderedWays = new Set();

  document.getElementById('discover').onclick = async () => {
    const {event, newItems} = await discover(secretPhase);
    switch (event) {
      case "nothing": {
        showPopup('Bohužel...', 'Na toto místo žádná šifra nevede, zkuste to jinde.');
        break;
      }
      case "badge-found": {
        if (newItems.length) {
          const {level, description} = newItems[0];
          showBadgePopup(level.toString(), description.slice(-2));
        } else {
          showPopup('No...', 'Jste tu správně, ale buď jste tu už byli nebo už jste v jiném levelu, takže nic nedostanete.');
        }
        break;
      }
      case "checkpoint-visited": {
        const items = newItems.filter((item) => item.type != "checkpoint");
        if (items.length) {
          showPopup('Hurá!', 'Organizátoři vám dali nové šifry.');
        } else {
          showPopup('Bohužel...', 'Tentokrát jste od organizátorů nic nedostali.');
        }
        break;
      }
    }
    let {items} = await getTeamState(secretPhase);
    drawInventory(items);
  }

  let {nodes, ways, state, items} = await getTeamState(secretPhase);
  const lines: Polyline[] = [];
  const latLng: LatLngLiteral = nodes.get(state.position)!.latLng;
  let currentNodeCoords: LatLng = new LatLng(latLng.lat, latLng.lng);
  let currentNode: Circle;

  document.getElementById('teamName').innerText = state.name;
  mapInstance.setView(currentNodeCoords, 17);
  console.debug(`Aktuální pozice: ${currentNodeCoords.toString()}`, `Aktuální nodeId: ${state.position}`);
  drawInventory(items);
  drawNodesAndWays(nodes, ways);

  function showBadgePopup(lvl: string, label: string) {
    const badgeClass = lvl ? `lvl${lvl}` : 'shrug';
    document.querySelector('#popup .large_badge > .label').innerHTML = label;
    const message = lvl === '5' ? 'Gratulujeme k dokončení kvalifikace a budeme se na vás (nejspíš) těšit na startu TMOU.' : 'Řešení je správně, získali jste za něj odznáček.';
    showPopup('Hurá!', message, badgeClass);
  }

  function showPopup(heading: string, text: string, badgeClass='shrug') {
    document.querySelector('.popup_text>h2').textContent = heading;
    document.querySelector('.popup_text>p').innerHTML = text;
    document.querySelector('#popup .large_badge').classList.add(badgeClass);
    document.getElementById('popup').classList.remove('popup__hidden');
    document.getElementById('overlay').classList.remove('overlay__hidden');
    document.getElementById('popup').classList.add('popup__visible');
    document.getElementById('overlay').classList.add('overlay__visible');

    document.querySelector('.popup #continue').addEventListener('click', hidePopup);
    document.addEventListener('keyup', hidePopup);

    function hidePopup(e: Event) {
      if ((e as KeyboardEvent).keyCode && (e as KeyboardEvent).keyCode !== 27) {
        return;
      }
      document.getElementById('popup').classList.remove('popup__visible');
      document.getElementById('overlay').classList.remove('overlay__visible');
      document.getElementById('popup').classList.add('popup__hidden');
      document.getElementById('overlay').classList.add('overlay__hidden');
      document.querySelector('#popup .large_badge').classList.value = 'large_badge';
      e.target.removeEventListener('click', hidePopup);
      document.removeEventListener('keyup', hidePopup);
    }
  }

  function drawNodesAndWays(nodes: Map<string, Node>, ways: Map<string, way>) {
    if (currentNode) {
      currentNode.setStyle({color: "#000000b5"});
    }
    for (const [id, way] of ways) {
      if (!renderedWays.has(id)) {
        const l = new Polyline(way, {color: "#00000066", weight: 3, interactive: false});
        lines.push(l);
        l.bringToBack();
        l.addTo(mapInstance);
        renderedWays.add(id);
      }
    }
    for (const [id, node] of nodes.entries()) {
      const nodeCoords = node.latLng;
      let c = renderedNodes.get(id);
      if (!c) {
        const clickHandler = async function (e: LeafletMouseEvent) {
          await handleNodeClick(e.target, id);
        }
        const radius = node.type === 'junction' ? 6 : 3;
        c = circleFactory(nodeCoords, id, "#000000b5", radius, clickHandler);
        c.addTo(mapInstance);
        c.bringToFront();
        renderedNodes.set(id, c);
      }

      if (currentNodeCoords.equals(nodeCoords)) {
        c.setStyle({color: "#FFEC01"});
        currentNode = c;
      }
    }
  }

  async function handleNodeClick(node: Circle, nodeId) {
    //mapInstance.setView(node.getLatLng(), mapInstance.getZoom());
    currentNodeCoords = node.getLatLng();
    const {nodes, ways, items, state} = await moveTeam(secretPhase, nodeId);
    console.debug(`Aktuální pozice: ${currentNodeCoords.toString()}`, `Aktuální nodeId: ${state.position}`);
    drawInventory(items);
    drawNodesAndWays(nodes, ways);
  }

  function formatTimestamp(timestamp: number) {
        const date = new Date(timestamp);
        const hours = date.getHours();
        const mins = date.getMinutes();

        return `${hours < 10 ? '0' : ''}${hours}:${mins < 10 ? '0' : ''}${mins}`;
  }

  function drawInventory(items: Item[]) {
    const puzzles = items.filter((item) => item.type === "puzzles").sort((a, b) => a.level - b.level).map(({url, level}) => `<li><a href="${url}">Level ${level}</a>`);
    const badges = items
      .filter((item) => item.type === "badge")
      .sort((a, b) => a.timestamp - b.timestamp)
      .map(({level, description, timestamp}) => {
        return `<div class="badge lvl${level}">
            <span class="time">${formatTimestamp(timestamp)}</span>
            <span class="label">${description.slice(-2)}</span>
          </div>`
      })
      .join('');
    document.getElementById('badges').innerHTML = badges;
    if (puzzles.length) {
      document.querySelector('#puzzles>#puzzles-list').innerHTML = `<ul>${puzzles.join('')}</ul>`;
    }

  }

}



run().then(r => console.log('Running'));
