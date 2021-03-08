import {getMap, switchToMapyCzBase, switchToMapyCzOutdoor, switchToOSM} from './map';
import {circleFactory} from "./circle";
import {
  Circle,
  LatLng,
  LatLngLiteral,
  LeafletMouseEvent,
  Polyline
} from "leaflet";
import {Item, Node, way, MessageWithTimestamp, Bonus, BadgeClass} from './types';
import {discover, getTeamState, moveTeam, fetchMessages, fetchBonuses, skip, checkSkip, skipStartPuzzle} from './api';
import {translations} from './translation';

const mapInstance = getMap('map', [49.195, 16.609], 15);

async function run() {
  // Data, init
  const secretPhrase = document.querySelector("body").dataset.secretphrase || null;
  const renderedNodes = new Map<string, Circle>();
  const renderedWays = new Set();
  const localContainer = [];

  // Check after page load, init
  drawTranslations(translations);
  messagesHandler();
  // Set periodic checks
  setInterval(messagesHandler, 10000);

  // Find team position, init
  let {nodes, ways, state, items} = await getTeamState(secretPhrase);
  const lines: Polyline[] = [];
  const latLng: LatLngLiteral = nodes.get(state.position)!.latLng;
  let currentNodeCoords: LatLng = new LatLng(latLng.lat, latLng.lng);
  let currentNode: Circle;

  const lastNodesAndWays = window.localStorage.getItem('nodesAndWays');
  if (lastNodesAndWays) {
    const nodesAndWays = JSON.parse(lastNodesAndWays);
    for (const nw of nodesAndWays) {
      const nodes: Map<string, Node> = new Map(JSON.parse(nw.nodes));
      const ways: Map<string, way> = new Map(JSON.parse(nw.ways));
      drawNodesAndWays(nodes, ways);
    }
  }

  // Set positions and items, init
  mapInstance.setView(currentNodeCoords, 17);
  drawInventory(items);
  drawNodesAndWays(nodes, ways);
  document.getElementById('teamName').innerText = state.name;
  
  // Controls Handlers
  document.getElementById("mapSelectorMO").onclick = switchToMapyCzOutdoor;
  document.getElementById("mapSelectorMB").onclick = switchToMapyCzBase;
  document.getElementById("mapSelectorOSM").onclick = switchToOSM;

  document.getElementById('discover').onclick = async () => {
    try {
    const {event, newItems} = await discover(secretPhrase);
    switch (event) {
      case "nothing": {
        showTextPopup(translations.popup_failed_heading, translations.popup_failed_search_text, 'shrug');
        break;
      }
      case "puzzles-found": {
        if (newItems.length) {
          showTextPopup(translations.popup_success_heading, translations.popup_success_search_text, 'get_puzzle');
        } else {
          showTextPopup(translations.popup_neutral_heading, translations.popup_neutral_search_text, 'shrug');
        }
        break;
      }
    }
    let {items} = await getTeamState(secretPhrase);
    drawInventory(items);
    } catch (e) {
        alert(translations.error);
        console.error(e);
    }
  }

  async function messagesHandler() {
    try {
      const messages = await fetchMessages(secretPhrase);
      drawMessages(messages);
    } catch (e) {
        alert(translations.error_messages);
        console.error(e);
    }
  }

  function showTextPopup(heading: string, text: string, badgeClass: BadgeClass) {
    showPopup(heading, `<p>${text}</p>`, badgeClass);
  }

  function showPopup(heading: string, content: string, badgeClass: BadgeClass, clickHandler?: (e: Event) => void) {
    document.querySelector('.popup_text>h2').textContent = heading;
    document.querySelector('.popup_text>div').innerHTML = content;
    document.querySelector('#popup .large_badge').classList.add(badgeClass);
    document.getElementById('popup').classList.remove('popup__hidden');
    document.getElementById('overlay').classList.remove('overlay__hidden');
    document.getElementById('popup').classList.add('popup__visible');
    document.getElementById('overlay').classList.add('overlay__visible');

    if (clickHandler) {
      document.querySelector('.popup #continue').addEventListener('click', clickHandler);
    }
    document.querySelector('.popup #continue').addEventListener('click', hidePopup);
    document.querySelector('.popup #close').addEventListener('click', hidePopup);
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
      if (clickHandler) {
        e.target.removeEventListener('click', clickHandler);
      }
      document.querySelector('.popup #continue').removeEventListener('click', hidePopup);
      document.querySelector('.popup #close').removeEventListener('click', hidePopup);
      document.removeEventListener('keyup', hidePopup);
    }
  }


  function drawNodesAndWays(nodes: Map<string, Node>, ways: Map<string, way>) {
    storeNodesAndWays(nodes, ways);
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

  function storeNodesAndWays(nodes: Map<string, Node>, ways: Map<string, way>) {
    if (localContainer.length === 10) {
      localContainer.shift();
    }

    localContainer.push({nodes: JSON.stringify(Array.from(nodes)), ways: JSON.stringify(Array.from(ways))});
    window.localStorage.setItem('nodesAndWays', JSON.stringify(localContainer));
  }

  function drawMessages(messages: MessageWithTimestamp[]) {
    const typesToClasses = {
      success: 'ok',
      fail: 'wrong',
      info: 'info',
    }

    let messagesElements = '';

    for (const m of messages) {
      messagesElements += `
        <div class="message ${typesToClasses[m.type]}">
        <p>${m.content}</p>
        <p>${formatTimestamp(m.timestamp)}</p>
        </div>
      `
    }

    if (!messagesElements) {
      messagesElements =`
        <div>
          <p>${translations.no_messages_yet}</p>
        </div>
      `
    }

    const messagesEl = document.getElementById('messages');
    messagesEl.innerHTML = messagesElements;
  }

  async function handleNodeClick(node: Circle, nodeId: string) {
    //mapInstance.setView(node.getLatLng(), mapInstance.getZoom());
    currentNodeCoords = node.getLatLng();
    const {nodes, ways, items} = await moveTeam(nodeId, secretPhrase);
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
    const puzzles = items
      .filter((item) => item.type === "puzzles" || item.type === "puzzles-fake" || item.type === "dead")
      .sort((a, b) => a.level - b.level)
      .map(({url, description}) => `<li><a href="${url}" target="_blank">${description}</a>`);

    if (puzzles.length) {
      document.querySelector('#puzzles>#puzzles-list').innerHTML = `<ul>${puzzles.join('')}</ul>`;
    }

  }

  function drawTranslations(translations: { [key: string]: string }) {
    for (const [id, val] of Object.entries(translations)) {
      const el = document.querySelector(`[data-translation='${id}']`);
      if (el) {
        el.textContent = val;
      }
    }
  }

  // Attach debugging current node function for teams in troubles
  window['debug_node'] = () => {
    console.info("Actual node ID", currentNode['id']);
    console.info("Actual node coord", currentNodeCoords);
  }
}



run().then(_ => console.log('Running'))
