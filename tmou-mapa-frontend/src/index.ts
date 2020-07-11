import {getMap} from './map';
import {circleFactory} from "./circle";
import {Circle, latLng, LatLng, LatLngTuple, LeafletMouseEvent, Polyline} from "leaflet";
import {getNodesAndWays, getCurrentNode} from './nodes';

const mapInstance = getMap('map', [49.195, 16.609], 15);


const {nodes, ways}: {nodes: LatLngTuple[], ways: LatLngTuple[][]} = getNodesAndWays();
const lines: Polyline[] = [];
const currentNodeCoords: LatLngTuple = getCurrentNode();
let currentNode: Circle;
let currentLine: Polyline;
for (const node of nodes) {
  const clickHandler = function (e: LeafletMouseEvent) {
    document.getElementById('pos').textContent = e.latlng.toString();
    currentNode.setStyle({color: "blue"});
    currentNode = e.target;
    mapInstance.setView(currentNode.getLatLng(), 18);
    for (const l of lines) {
      for (const coords of l.getLatLngs()) {
        if ((coords as LatLng).equals(currentNode.getLatLng())) {
          currentLine.setStyle({color: "black", weight: 1});
          l.setStyle({color: 'red', weight: 5});
          currentLine = l;
        }
      }
    }
  }

  const c = circleFactory(node, 'alfa', "blue", 2, clickHandler);
  if (node[0] === currentNodeCoords[0] && node[1] === currentNodeCoords[1]) {
    c.setStyle({color: 'red'});
    currentNode = c;
  }
  c.addTo(mapInstance);
}

for (const way of ways) {
  const l = new Polyline(way, {color: "black", weight: 1, interactive: false});
  lines.push(l);
  l.bringToBack();
  l.addTo(mapInstance);
  for (const n of way) {
    if (latLng(n).equals(currentNode.getLatLng())) {
      l.setStyle({color: 'red', weight: 5});
      currentLine = l;
    }
  }
}

mapInstance.setView(currentNode.getLatLng(), 18);
