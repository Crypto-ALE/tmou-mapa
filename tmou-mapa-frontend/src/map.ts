import * as L from 'leaflet';
import {LatLngTuple, LeafletMouseEvent} from "leaflet";

export function getMap(mapId: string, coords: LatLngTuple, zoom: number) {
  const map = L.map(mapId).setView(coords, zoom);

  L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
    attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
  }).addTo(map);


  return map;
}

