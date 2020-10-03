import {getMap} from './map';
import {
  Circle,
  layerGroup,
} from "leaflet";
import {TeamPosition, Standings} from './types';
import {getTeamsPositions, sendMessage, getStandings} from './api';

const mapInstance = getMap('map', [49.195, 16.609], 15);
const teamsPositionsLayer = layerGroup();
teamsPositionsLayer.addTo(mapInstance);
const colors = ['#000000aa', '#ffd83c', '#28d428', '#2972ff', '#ff2929', '#9346ba']

async function run() {
  await updateTeamsPositions();
  await updateStandings();
  setInterval(updateTeamsPositions, 5000);
  setInterval(updateStandings, 10000);
}


async function updateTeamsPositions() {
    const teamsPositions = await getTeamsPositions();
    drawTeamsPositions(teamsPositions);
}


async function updateStandings() {
    const standings = await getStandings();
    drawStandings(standings);
}


function drawStandings(standings: Standings) {
  let s = "<table><tr><th>#</th><th>Tým</th>";
  for (const bl of standings.badge_labels) {
    const t = bl.slice(-3)
    s += `<th class="header_badge lvl${t[0]}"><span>${t}</span></th>`;
  }
  s += '</tr>';
  for (const t of standings.standings) {
    s += `<tr><td style="text-align: left">${t.rank}</td><td style="text-align: left">${t.name}</td>`;
    for (const bl of standings.badge_labels) {
      const ts = t.badge_timestamps[bl];
      s += `<td title="${ts ? formatTimestamp(ts) : ''}">${ ts ? '✓' : '✗'}</td>`;
    }
    s += `</tr>`
  }
  document.getElementById('standings').innerHTML = s;
}

function drawTeamsPositions(teamsPositions: TeamPosition[]) {
  teamsPositionsLayer.clearLayers();
  for (const team of teamsPositions) {
    const c = new Circle(team.position.latLng, 18, {color: colors[team.level], fillOpacity: 1, interactive: true});
    c.bindTooltip(team.teamName);
    c.addTo(teamsPositionsLayer);
  }

  document.getElementById('discover').onclick = async (e: Event) => {
    e.preventDefault();
    const formEl = document.getElementById("messageForm") as HTMLFormElement;
    const data = new FormData(formEl);
    try {
      await sendMessage(data);
      //TODO Better flash system
      alert("Zpráva odeslána");
      (document.getElementById("message") as HTMLFormElement).value = null;
    } catch (e) {
      alert("Nepovedlo se odeslat zprávu.");
    }
  }
}

function formatTimestamp(timestamp: number) {
      const date = new Date(timestamp);
      const hours = date.getHours();
      const mins = date.getMinutes();

      return `${hours < 10 ? '0' : ''}${hours}:${mins < 10 ? '0' : ''}${mins}`;
}
run().then(_r => console.log('Running for admin'));
