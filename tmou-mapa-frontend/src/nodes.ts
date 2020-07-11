
const nodes = [];
const ways = [[]];

export function getNodesAndWays() {
  if (nodes.length === 0) {
  const Connect = new XMLHttpRequest();
  Connect.open("GET", "map.osm", false);
  Connect.setRequestHeader("Content-Type", "text/xml");
  Connect.send(null);
  const txt = Connect.responseXML;
  // @ts-ignore
  for (const way: Element of txt.firstChild.getElementsByTagName('way')) {
    for (const tag of way.getElementsByTagName('tag')) {
      if (tag.getAttribute('k') === 'highway' && tag.getAttribute('v') === 'footway') {
        for (const nd of way.getElementsByTagName('nd')) {
            const refId = nd.getAttribute('ref');
            const node = txt.getElementById(refId);
            const nodeCoords = [node.getAttribute('lat'), node.getAttribute('lon')];
            nodes.push(nodeCoords);
            ways[ways.length - 1].push(nodeCoords);
        }
      }
    }
    ways.push([]);
  }
}

  return {nodes, ways};
}


export function getCurrentNode() {
  const index = Math.random() * nodes.length;
  return nodes[Math.floor(index)];
}
