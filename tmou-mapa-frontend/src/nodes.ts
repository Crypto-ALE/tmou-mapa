import {LatLngLiteral} from "leaflet";

interface Info {
  position: string;
  ranking: number;
}

interface Item {
  type: "puzzles" | "badge" | "message",
  url: String,
  level: number,
  name: String,
  description: String,
}

interface Items {
  items: Item[],
}

interface Node {
  latLng: LatLngLiteral,
  type: "ordinary" | "junction",
  data?: String,
}

interface TeamState {
  nodes: Map<string, Node>;
  ways: LatLngLiteral[][];
  state: Info;
  items: Items;
}

export async function getTeamState(secretPhrase: string): Promise<TeamState> {
  const res = await fetch(`/game/${secretPhrase}`);

  return parseJson(await res.json());
}

export async function moveTeam(secretPhrase: string, nodeId: string): Promise<TeamState> {
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

function parseJson(res): TeamState {
  const {pois, state, items} = res;
  const nodes: Map<string, Node> = new Map(
      pois.nodes
          // .filter((node) => node.type === 'junction')
          .map((node) => [node.id, {latLng:{lat: node.y, lng: node.x}, type: node.type, data: node.data}])
  );
  const ways = pois.ways.map((way) => way.nodes.map(nodeId => nodes.get(nodeId)!.latLng));

  return {nodes, ways, state, items};
}
