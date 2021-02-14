import * as L from 'leaflet';
import {LatLngTuple} from "leaflet";

let map: L.Map;
let currentLayer: L.TileLayer;

export function getMap(mapId: string, coords: LatLngTuple, zoom: number) {
  map = L.map(mapId).setView(coords, zoom);

  switchToMapyCzOutdoor();

  return map;
}

export function switchToMapyCzOutdoor() {
  switchCurrentLayer(L.tileLayer("https://mapserver.mapy.cz/turist-m/{z}-{x}-{y}", {
    attribution: '<img src="/static/img/mapy_outline.png" style="width: 10vh"></a>'
  }));

  currentLayer.addTo(map);
}

export function switchToMapyCzBase() {
  switchCurrentLayer(L.tileLayer("https://mapserver.mapy.cz/base-m/{z}-{x}-{y}", {
    attribution: '<img src="/static/img/mapy_outline.png" style="width: 10vh"></a>'
  }));

  currentLayer.addTo(map);
}

export function switchToOSM() {
  switchCurrentLayer(L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
    attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
  }));

}

function switchCurrentLayer(layer: L.TileLayer) {
  if (currentLayer) {
    map.removeLayer(currentLayer);
  }

  currentLayer = layer;
  currentLayer.addTo(map);
}
