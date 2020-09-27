import {TeamState, Node, way, DiscoveryEvent, TeamPosition, Message} from './types';

export async function getTeamState(secretPhrase?: string): Promise<TeamState> {
  const url = secretPhrase ? `/game/${secretPhrase}` : '/game';
  const res = await fetch(url);

  return parseJson(await res.json());
}

export async function getTeamsPositions(): Promise<TeamPosition[]> {
  const res = await fetch(`/admin/positions`);
  const teamPositions = await res.json();

  const tps: TeamPosition[] = teamPositions.map((item: any) => {
    return {
      teamName: item.team_name,
      position: {
        latLng: {lat: item.lat, lng: item.lon},
        type: "ordinary" //HAAAAACK
      },
      level: item.level,
    }
  });

  return tps;
}

export async function moveTeam(nodeId: string, secretPhrase?: string): Promise<TeamState> {
  const url = secretPhrase ? `/game/${secretPhrase}` : '/game';
  const res = await fetch(url, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json;charset=utf-8'
      },
      body: JSON.stringify({nodeId})
    });

    return parseJson(await res.json());
}

export async function discover(secretPhrase?: string): Promise<DiscoveryEvent> {
  const url = secretPhrase ? `/game/${secretPhrase}/discover` : '/game/discover';
  const res = await fetch(url);

  return (await res.json());
}

export async function fetchMessages(secretPhrase?: string, limit?: number): Promise<Message[]> {
  const url = new URL(secretPhrase ? `/messages/${secretPhrase}` : '/messages', document.location.toString());
  if (limit) {
    url.search = new URLSearchParams([['limit', limit.toString()]]).toString();
  }
  const res = await fetch(url.toString());
  const messages = await res.json();

  return timestampMapper(messages);
}

function parseJson(res: any): TeamState {
  const {pois, state, items} = res;
  const nodes: Map<string, Node> = new Map(
      pois.nodes
          // .filter((node) => node.type === 'junction')
          .map((node: any) => [node.id, {latLng:{lat: node.y, lng: node.x}, type: node.type, data: node.data}])
  );
  const ways: Map<string, way> = new Map(
    pois.ways.map((way: any) => [way.id, way.nodes.map(nodeId => nodes.get(nodeId)!.latLng)])
  );
  const parsed_items = timestampMapper(items.items);
  return {nodes, ways, state, items: parsed_items};
}

function timestampMapper(items: any[]) {
  return items.map((item: any) => {
    // FIXME: Time from server comes in UTC without timezone specification
    // currently hardocing for correct parsing
    return {...item, timestamp: Date.parse(item.timestamp+"+00:00")}
  });
}
