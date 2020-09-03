import {TeamState, Node, DiscoveryEvent} from './types';

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

export async function discover(secretPhrase: string): Promise<DiscoveryEvent> {
  const res = await fetch(`/game/${secretPhrase}/discover`);

  return (await res.json());
}

function parseJson(res: any): TeamState {
  const {pois, state, items} = res;
  const nodes: Map<string, Node> = new Map(
      pois.nodes
          // .filter((node) => node.type === 'junction')
          .map((node: any) => [node.id, {latLng:{lat: node.y, lng: node.x}, type: node.type, data: node.data}])
  );
  const ways = pois.ways.map((way: any) => way.nodes.map(nodeId => nodes.get(nodeId)!.latLng));

  return {nodes, ways, state, items};
}
