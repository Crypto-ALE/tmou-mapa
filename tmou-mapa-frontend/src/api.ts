import {TeamState, Node, way, DiscoveryEvent, TeamPosition, MessageWithTimestamp, OutgoingMessage, MessageType, Standings, TeamStanding, Bonus, Skip, Item} from './types';

export async function getTeamState(secretPhrase?: string): Promise<TeamState> {
  const url = secretPhrase ? `/game/${secretPhrase}` : '/game';
  const res = await fetch(url);

  if (!res.ok) {
    throw new Error("Team state not working, has game started?");
  }

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

  if (!res.ok) {
    throw new Error("Move team not working, has game started?");
  }

    return parseJson(await res.json());
}

export async function skipStartPuzzle(data: FormData, secretPhrase?: string): Promise<Item[]> {
  const url = secretPhrase ? `/game/${secretPhrase}/discover` : '/game/discover';
  const payload = {
    puzzleName: data.get('puzzleName').toString(),
  }
  const res = await fetch(url, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json;charset=utf-8'
      },
      body: JSON.stringify(payload)
    });

  if (!res.ok) {
    throw new Error("Skip puzzle doesn't work, has game started?");
  }

    return await res.json();
}

export async function checkSkip(secretPhrase?: string): Promise<Skip> {
  const url = secretPhrase ? `/game/${secretPhrase}/skip` : '/game/skip';
  const res = await fetch(url);

  if (!res.ok) {
    throw new Error("Skip check is not working, is game running?");
  }

  return (await res.json());
}

export async function skip(verified: boolean, secretPhrase?: string): Promise<DiscoveryEvent> {
  const url = secretPhrase ? `/game/${secretPhrase}/dead` : '/game/dead';
  const res = await fetch(url, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json;charset=utf-8'
      },
      body: JSON.stringify({verified})
    });

  if (!res.ok) {
    throw new Error("Skip check is not working, is game running?");
  }

  return (await res.json());
}

export async function discover(secretPhrase?: string): Promise<DiscoveryEvent> {
  const url = secretPhrase ? `/game/${secretPhrase}/discover` : '/game/discover';
  const res = await fetch(url);

  if (!res.ok) {
    throw new Error("Discover not working, is game running?");
  }

  return (await res.json());
}

export async function fetchMessages(secretPhrase?: string, limit?: number): Promise<MessageWithTimestamp[]> {
  const url = new URL(secretPhrase ? `/messages/${secretPhrase}` : '/messages', document.location.toString());
  if (limit) {
    url.search = new URLSearchParams([['limit', limit.toString()]]).toString();
  }
  const res = await fetch(url.toString());
  const messages = await res.json();

  if (!res.ok) {
    throw new Error("Discover not working, is game running?");
  }

  return timestampMapper(messages);
}

export async function fetchBonuses(secretPhrase?: string): Promise<Bonus[]> {
  try {
    const url = '/game/bonuses';
    const res = await fetch(url);

    if (!res.ok) {
      return [];
    }
    return (await res.json());
  } catch (e) {
    console.error(e);

    return [];
  }
}

export async function sendMessage(data: FormData, secretPhrase?: string) {
  const payload: OutgoingMessage = {
    recipient_id: parseInt(data.get('recipient').toString(), 10),
    message: {content: data.get('message').toString(), type: data.get('type') as MessageType}
  }

  const url = secretPhrase ? `/messages/${secretPhrase}` : '/messages';
  const res = await fetch(url, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json;charset=utf-8'
      },
      body: JSON.stringify(payload)
    });

    if (!res.ok) {
      throw new Error();
    }
}

export async function getStandings(): Promise<Standings> {
  const res = await fetch(`/admin/standings`);
  const {badge_labels, standings: standings_raw} = await res.json();

  const standings: TeamStanding[] = standings_raw.map((item: any) => {
    return {
      name: item.name,
      rank: item.rank,
      badge_timestamps: Object.entries(item.badge_timestamps).reduce((acc, [k,v]) => {
        acc[k] = Date.parse(v+"+00:00");
        return acc;
      }, {}),
    }
  });

  return {badge_labels, standings};
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
