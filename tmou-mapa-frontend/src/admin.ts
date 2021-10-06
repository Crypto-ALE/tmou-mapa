import {getMap} from './map';
import {saveAs} from 'file-saver';
import {
  Circle,
  layerGroup,
} from "leaflet";
import {TeamPosition, Standings} from './types';
import {getTeamsPositions, sendMessage, getStandings} from './api';

const mapInstance = getMap('map', [49.195, 16.609], 15);
const teamsPositionsLayer = layerGroup();
teamsPositionsLayer.addTo(mapInstance);
const colors = ['#ffffff', '#aaaaaa', '#555555', '#ffff55', '#aa00aa', '#55ff55', '#ff55ff', '#aa0000', '#aa5500', '#aa00aa', '#ff55ff', '#55ffff', '#00aaaa', '#0000aa', '#aaaaff', '#000000'];

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
    console.log("STANDINGS", standings);
    drawStandings(standings);
}

  const levelMap = {
    0: 4,
    1: 3,
    2: 2,
    3: 1,
    4: 1,
  }

function drawStandings(standings: Standings) {
  let s = "<table><tr><th>#</th><th>Tým</th>";
  for (let i=0; i < 5; i++) {
    for (let j=1; j <= levelMap[i]; j++) {
      const puzzleId = (i+1)*10+j;
      s += `<th><span>${puzzleId}</span></th>`;
    }
  }
  s += '</tr>';
  for (const t of standings.standings) {
    s += `<tr><td style="text-align: left">${t.rank}</td><td style="text-align: left">${t.name}</td>`;
    for (let i=0; i < 5; i++) {
      const ts = t.badges[i];
      for (let j=1; j <= levelMap[i]; j++) {
        const puzzleId = (i+1)*10+j;
        if (!ts || !ts[puzzleId]) {
          s += '<td>✗</td>';
        } else {
          s += `<td title="${formatTimestamp(ts[puzzleId])}">✓</td>`;
        }
      }
    }
    s += `</tr>`
  }
  s += '</table>';
  document.getElementById('standings').innerHTML = s;
}

function drawTeamsPositions(teamsPositions: TeamPosition[]) {
  teamsPositionsLayer.clearLayers();
  for (const team of teamsPositions) {
    const c = new Circle(team.position.latLng, 18, {color: colors[team.level], fillOpacity: 1, interactive: true});
    c.bindTooltip(team.teamName);
    c.addTo(teamsPositionsLayer);
  }

  document.getElementById('export').onclick = async (e: Event) => {
    e.preventDefault();
    await export_results();
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

async function export_results() {
  let standings = await getStandings();
  function gen_header() {
    let s = '<thead><tr><th class="bg-yellow-tmou">Pořadí</th><th class="bg-yellow-tmou">Název týmu</th>';
    for (let i=0; i < 5; i++) {
        s += `<th class="bg-yellow-tmou" colspan="${levelMap[i]}">${i+1}. level</th>`;
    }
        s += `<th class="bg-yellow-tmou"> Čas poslední odpovědi</th>`;
    s += '</tr></thead>';

    return s;
  }

  let s = '<div class="table-responsive">';
  s += '<table class="datagrid datagrid-grid w-full" cellspacing="0" cellpadding="0">';
  for (const c of standings.standings) {
    if (c.rank % 50 == 1) {
      s += gen_header();
    }
    s += `<tr><td>${c.rank}</td><td>${c.name}</td>`;
    for (let i=0; i < 5; i++) {
      const ts = c.badges[i];
      for (let j=1; j <= levelMap[i]; j++) {
        const puzzleId = (i+1)*10+j;
        if (!ts || !ts[puzzleId]) {
          s += '<td class="text-center bg-fail-tmou">✗</td>';
        } else {
          s += `<td title="${formatTimestamp(ts[puzzleId])}" class="text-center bg-success-tmou">✓</td>`;
        }
      }
    }
    s += `<td class="text-right">${formatTimestamp(c.maxTimestamp)}</td>`;
    s += `</tr>`
  }
    s += '</table></div>';
  const blob = new Blob([s], {type: "text/plain;charset=utf-8"});

  saveAs(blob, "vysledky.txt");
}

function formatTimestamp(timestamp: number) {
      const date = new Date(timestamp);
      const hours = date.getHours();
      const mins = date.getMinutes();

      return `${hours < 10 ? '0' : ''}${hours}:${mins < 10 ? '0' : ''}${mins}`;
}

run().then(_r => console.log('Running for admin'));
