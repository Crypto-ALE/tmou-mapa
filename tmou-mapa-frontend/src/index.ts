import {getMap, switchToMapyCzBase, switchToMapyCzOutdoor, switchToOSM} from './map';
import {circleFactory} from "./circle";
import {
  Circle,
  LatLng,
  LatLngLiteral,
  LeafletMouseEvent,
  Polyline
} from "leaflet";
import {Item, Node, way, MessageWithTimestamp, Bonus} from './types';
import {discover, getTeamState, moveTeam, fetchMessages, fetchBonuses, skip, checkSkip} from './api';

const mapInstance = getMap('map', [49.195, 16.609], 15);

async function run() {
  const secretPhrase = document.querySelector("body").dataset.secretphrase || null;
  const renderedNodes = new Map<string, Circle>();
  const renderedWays = new Set();
  const localContainer = [];


  messagesHandler();
  setInterval(messagesHandler, 10000);
  bonusesHandler();
  setInterval(bonusesHandler, 60000);
  checkSkipHandler();
  setInterval(checkSkipHandler, 60000);

  async function checkSkipHandler() {
    let allowed: boolean;
    try {
      ({allowed} = await checkSkip(secretPhrase));
    } catch (e) {
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
    const messages = await fetchMessages(secretPhrase);
    drawMessages(messages);
  }

  async function bonusesHandler() {
    const bonuses = await fetchBonuses(secretPhrase);
    drawBonuses(bonuses);
  }

  document.getElementById("mapSelectorMO").onclick = switchToMapyCzOutdoor;
  document.getElementById("mapSelectorMB").onclick = switchToMapyCzBase;
  document.getElementById("mapSelectorOSM").onclick = switchToOSM;

  document.getElementById('discover').onclick = async () => {
    try {
    const {event, newItems} = await discover(secretPhrase);
    // TODO adjust messages based on the discover updates
    switch (event) {
      case "nothing": {
        showPopup('Bohužel...', 'Na toto místo žádná šifra nevede, zkuste to jinde.', 'shrug');
        break;
      }
      case "badge-found": {
        if (newItems.length) {
          const {level, description} = newItems[0];
          showBadgePopup(level.toString(), description.slice(-2));
        } else {
          showPopup('No...', 'Jste tu správně, ale buď jste tu už byli nebo už jste v jiném levelu, takže nic nedostanete.', 'shrug');
        }
        // badge can trigger lower limit for skip, check it
        checkSkipHandler();
        break;
      }
      case "checkpoint-visited": {
        const items = newItems.filter((item) => item.type != "checkpoint");
        if (items.length) {
          showPopup('Hurá!', 'Organizátoři vám dali nové šifry.', 'get_puzzle');
        } else {
          showPopup('Bohužel...', 'Tentokrát jste od organizátorů nic nedostali.', 'shrug');
        }
        break;
      }
    }
    let {items} = await getTeamState(secretPhrase);
    drawInventory(items);
    } catch (e) {
      console.info(e);
    }
  }
  document.getElementById('skip').onclick = async () => {
    const validate = window.confirm("Opravdu chcete přeskočit šifru?");
    if (validate) {
      // skip puzzle
      try {
        // TODO check server response
        await skip(validate, secretPhrase);
      } catch (e) {
        console.error(e);
      }
      // get new items
      let {items} = await getTeamState(secretPhrase);
      drawInventory(items);
      // skip used, disable control
      updateSkipControl(false);
    }
  }

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

  document.getElementById('teamName').innerText = state.name;
  mapInstance.setView(currentNodeCoords, 17);
  drawInventory(items);
  drawNodesAndWays(nodes, ways);

  function showBadgePopup(lvl: string, label: string) {
    const badgeClass = (lvl ? `lvl${lvl}` : 'shrug') as BadgeClass;
    document.querySelector('#popup .large_badge > .label').innerHTML = label;
    const message = lvl === '5' ? 'Gratulujeme k dokončení kvalifikace a budeme se na vás (nejspíš) těšit na startu TMOU.' : 'Řešení je správně, získali jste za něj odznáček.';
    showPopup('Hurá!', message, badgeClass);
  }

  type BadgeClass = 'lvl1' | 'lvl2' | 'lvl3' | 'lvl4' | 'lvl5' | 'shrug' | 'get_puzzle';

  function showPopup(heading: string, text: string, badgeClass: BadgeClass) {
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
          <p>Tady se můžou objevit zprávy od organizátorů.</p>
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
      .filter((item) => item.type === "puzzles")
      .sort((a, b) => a.level - b.level)
      .map(({url, level}) => `<li><a href="${url}" target="_blank">Level ${level}</a>`);

    const badges = items
      .filter((item) => item.type === "badge")
      .sort((a, b) => a.timestamp - b.timestamp)
      .map(({level, description, timestamp}) => {
        return `<div class="badge lvl${level}">
            <span class="time">${formatTimestamp(timestamp)}</span>
          </div>`
      })
      .join('');

    document.getElementById('badges').innerHTML = badges;
    if (puzzles.length) {
      document.querySelector('#puzzles>#puzzles-list').innerHTML = `<ul>${puzzles.join('')}</ul>`;
    }

  }

  function drawBonuses(items: Bonus[]) {
    const bonuses = items
      .map(({url, label}) => `<li><a href="${url}" target="_blank">${label}</a>`);

    document.querySelector('#bonuses>#bonuses-list').innerHTML = `<ul>${bonuses.join('')}</ul>`;

  }

  // Attach debugging current node function for teams in troubles
  window['debug_node'] = () => {
    console.info("Actual node ID", currentNode['id']);
    console.info("Actual node coord", currentNodeCoords);
  }
}



run().then(r => console.log('Running'))
