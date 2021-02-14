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
    drawStandings(standings);
}


function drawStandings(standings: Standings) {
  let s = "<table><tr><th>#</th><th>Tým</th>";
  for (let i=0; i < 16; i++) {
    s += `<th><span>${i}</span></th>`;
  }
  s += '</tr>';
  for (const t of standings.standings) {
    s += `<tr><td style="text-align: left">${t.rank}</td><td style="text-align: left">${t.name}</td>`;
    for (let j=0; j < 16; j++) {
      const ts = t.puzzles[j];
      if (j == 1) {
    const start_score = `${t.start_puzzles_solved}/10`;
        s += `<td title="${ts?.dead ? start_score : ts ? formatTimestamp(ts.timestamp) : ''}">${ ts?.dead ? '💀' : start_score}</td>`;
      } else {
      s += `<td title="${ts ? formatTimestamp(ts.timestamp) : ''}">${ ts ? (ts.dead ? '💀' : '✓') : '✗'}</td>`;
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
    let s = '<thead><tr><th class="bg-yellow-tmou">Pořadí</th><th class="bg-yellow-tmou">Název týmu</th><th class="bg-yellow-tmou">START</th>';
    for (let i=1; i < 15; i++) {
      s += `<th class="bg-yellow-tmou">${i}</th>`;
    }
    s += `<th class="bg-yellow-tmou">Cíl</th><th class="bg-yellow-tmou">Bonusy</th></tr></thead>`;

    return s;
  }

  let s = '<div class="table-responsive">';
  s += '<table class="stick-2-left-columns datagrid datagrid-grid w-full" cellspacing="0" cellpadding="0">';
  for (const c of standings.standings) {
    if (c.rank % 50 == 1) {
      s += gen_header();
    }
    s += `<tr><td class="text-center">${c.rank}</td><td class="text-center">${c.name}</td>`;
    for (let i=0; i < 16; i++) {
      const ts = c.puzzles[i];
      if (!ts) {
        s += '<td></td>';
        continue;
      }
      const dead_class = ts.dead ? 'bg-fail-tmou' : '';
      const cell_val = i == 0 ? `${c.start_puzzles_solved}/10` : formatTimestamp(ts.timestamp);
      s += `<td class="text-center ${dead_class}">${cell_val}</td>`;
    }
    s += `<td class="text-center">${c.badge_count}</td>`;
    s += '</tr>';
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
