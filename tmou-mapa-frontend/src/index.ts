import {getMap, switchToMapyCzBase, switchToMapyCzOutdoor, switchToOSM} from './map';
import {circleFactory, squareFactory, MapCircle, MapRectangle} from "./nodes";
import {
  LatLng,
  LatLngLiteral,
  LeafletMouseEvent,
  Polyline
} from "leaflet";
import {Item, Node, way, MessageWithTimestamp, BadgeClass} from './types';
import {discover, getTeamState, moveTeam, fetchMessages, skip, checkSkip, skipStartPuzzle} from './api';
import {translations} from './translation';
import {config} from './config';
import { bonusesHandler } from './modules/bonuses';

const mapInstance = getMap('map', [49.195, 16.609], 15);
type MapNode = MapCircle | MapRectangle;

async function run() {
  // Data, init
  const secretPhrase = document.querySelector("body").dataset.secretphrase || null;
  const renderedNodes = new Map<string, MapNode>();
  const bonusesEnabled = (document.querySelector("#bonuses") as HTMLElement)?.dataset.bonusesenabled || false;
  const renderedWays = new Set();
  const localContainer = [];

  // Check after page load, init
  drawTranslations(translations);
  messagesHandler();
  checkSkipHandler();
  // Set periodic checks
  setInterval(messagesHandler, 10000);
  setInterval(checkSkipHandler, 60000);

  if (bonusesEnabled) {
    await bonusesHandler();
    setInterval(bonusesHandler, 60000);
  }

  // Find team position, init
  let {nodes, ways, state, items} = await getTeamState(secretPhrase);
  const lines: Polyline[] = [];
  const latLng: LatLngLiteral = nodes.get(state.position)!.latLng;
  let currentNodeCoords: LatLng = new LatLng(latLng.lat, latLng.lng);
  let currentNode: MapNode;
  let currentNodeColor: string;

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
      case "badge-found": {
        if (newItems.length) {
          const {name} = newItems[0];
          showBadgePopup(name);
        } else {
          showTextPopup(translations.popup_neutral_heading, translations.popup_neutral_badge_text, 'shrug');
        }
        // badge can trigger lower limit for skip, check it
        checkSkipHandler();
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
      case "checkpoint-start-visited": {
        if (newItems.length) {
          showSelectPopup(translations.popup_checkpoint_select, newItems, 'get_puzzle');
        } else {
          showTextPopup(translations.popup_neutral_heading, translations.popup_neutral_checkpoint_text, 'shrug');
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

  document.getElementById('skip').onclick = async () => {
    const validate = window.confirm(translations.skip_confirmation);
    if (validate) {
      // skip puzzle
      try {
        let {newItems} = await skip(validate, secretPhrase);
        drawInventory(newItems);
        // skip used, disable control
        updateSkipControl(false);
      } catch (e) {
        alert(translations.error);
        console.error(e);
      }
    }
  }

  async function checkSkipHandler() {
    let allowed: boolean;
    try {
      ({allowed} = await checkSkip(secretPhrase));
    } catch (e) {
      alert(translations.error);
      console.error(e);
    }
    updateSkipControl(allowed);
  }

  function updateSkipControl(enable: boolean) {
    const skipEl = document.getElementById("skip");
    if (enable) {
      skipEl.removeAttribute('disabled');
      skipEl.classList.remove('disabled');
      skipEl.classList.add('enabled');
    } else {
      skipEl.setAttribute('disabled', 'disabled');
      skipEl.classList.remove('enabled');
      skipEl.classList.add('disabled');
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


  function showBadgePopup(name: string) {
    showTextPopup(translations.popup_success_heading, translations.popup_success_badge_text, name as BadgeClass);
  }


  function showSelectPopup(heading: string, options: Item[], badgeClass: BadgeClass) {
    const opts = options.map((opt) => `<option value='${opt.name}'>${opt.description}</option>`);
    const form = `<form method='POST' id='skipStartPuzzle'><select name="puzzleName">${opts.join('')}</select></form>`;
    const clickHandler = async (e: Event) => {
      e.preventDefault();
      const formEl = document.getElementById("skipStartPuzzle") as HTMLFormElement;
      const data = new FormData(formEl);
      try {
        const newItems = await skipStartPuzzle(data, secretPhrase);
        drawInventory(newItems);
      } catch (e) {
        alert(translations.error);
        console.error(e);
      }
  }

    showPopup(heading, form, badgeClass, clickHandler);
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
      currentNode.setStyle({color: currentNodeColor});
    }

    for (const [id, way] of ways) {
      if (!renderedWays.has(id)) {
      	const color = config.tagColors[way.tag] || '#0085C766';
        const l = new Polyline(way.latLng, {color, weight: 3, interactive: false});
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
        currentNodeColor = config.tagColors[node.tag] || '#0085C766';
        const clickHandler = async function (e: LeafletMouseEvent) {
          await handleNodeClick(e.target, id);
        }
        if (node.type === 'checkpoint') {
          c = squareFactory(nodeCoords, id, currentNodeColor, clickHandler);
        }
        else {
          const radius = node.type === 'junction' ? 6 : 3;
          c = circleFactory(nodeCoords, id, currentNodeColor, radius, clickHandler);
        }
        c.addTo(mapInstance);
        c.bringToFront();
        renderedNodes.set(id, c);
      }

      if (currentNodeCoords.equals(nodeCoords)) {
        c.setStyle({color: "#ff7b00"});
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

  async function handleNodeClick(node: MapNode, nodeId: string) {
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

    const badges = items
      .filter((item) => item.type === "badge")
      .sort((a, b) => a.timestamp - b.timestamp)
      .map(({name, description}) => {
        return `<div class="badge ${name}" title="${description}"></div>`
      })
      .join('');

    document.getElementById('badges').innerHTML = badges;
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
