import {LatLngLiteral} from "leaflet";

interface Info {
  position: string;
  ranking: number;
}

interface TeamState {
  nodes: Map<string, LatLngLiteral>;
  ways: LatLngLiteral[][];
  state: Info;
}

export async function getNodesAndWays(secretPhrase: string): Promise<TeamState> {
  const res = await fetch(`/game/${secretPhrase}`);

  return parseJson(await res.json());
}

export async function updateNodesAndWays(secretPhrase: string, nodeId: string): Promise<TeamState> {
  const res = await fetch(`/game/${secretPhrase}`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json;charset=utf-8'
      },
      body: JSON.stringify({nodeId})
    });

    return parseJson(await res.json());
}

export async function discover(secretPhrase: string) {
  const res = await fetch(`/game/${secretPhrase}/discover`);

  return (await res.json());
}

function parseJson({pois, state}): TeamState {
  console.log(state);
  const nodes: Map<string, LatLngLiteral> = new Map(
      pois.nodes
          // .filter((node) => node.type === 'junction')
          .map((node) => [node.id, {lat: node.y, lng: node.x}])
  );
  const ways = pois.ways.map((way) => way.nodes.map(nodeId => nodes.get(nodeId)));

  return {nodes, ways, state};
}
