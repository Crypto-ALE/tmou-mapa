import {getMap} from './map';
import {
  Circle,
} from "leaflet";
import {getTeamsPositions} from './api';

const mapInstance = getMap('map', [49.195, 16.609], 15);
const colors = ['#000000aa', '#ffd83c', '#28d428', '#2972ff', '#ff2929', '#9346ba']

async function run() {
  const teamsPositions = await getTeamsPositions();

  for (const team of teamsPositions) {
    const c = new Circle(team.position.latLng, 18, {color: colors[team.level], fillOpacity: 1, interactive: true});
    c.bindTooltip(team.teamName);
    c.addTo(mapInstance);
  }
}

run().then(_r => console.log('Running for admin'));
