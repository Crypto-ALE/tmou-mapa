import {getMap} from './map';
import {
  Circle,
  layerGroup,
} from "leaflet";
import {TeamPosition} from './types';
import {getTeamsPositions, sendMessage} from './api';

const mapInstance = getMap('map', [49.195, 16.609], 15);
const teamsPositionsLayer = layerGroup();
teamsPositionsLayer.addTo(mapInstance);
const colors = ['#000000aa', '#ffd83c', '#28d428', '#2972ff', '#ff2929', '#9346ba']

async function run() {
  await updateTeamsPositions();
  setInterval(updateTeamsPositions, 5000);
}


async function updateTeamsPositions() {
    const teamsPositions = await getTeamsPositions();
    drawTeamsPositions(teamsPositions);
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

run().then(_r => console.log('Running for admin'));
