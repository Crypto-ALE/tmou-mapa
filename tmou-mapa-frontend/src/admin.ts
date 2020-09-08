import {getMap} from './map';
import {
  Circle,
} from "leaflet";
import {getTeamsPositions} from './nodes';

const mapInstance = getMap('map', [49.195, 16.609], 15);

async function run() {
  const teamsPositions = await getTeamsPositions();

  for (const team of teamsPositions) {
    const c = new Circle(team.position.latLng, 18, {color: "salmon", fillOpacity: 1, interactive: true});
    c.bindTooltip(team.teamName);
    c.addTo(mapInstance);
  }
}

run().then(_r => console.log('Running for admin'));
