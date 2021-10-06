import {LatLngLiteral} from "leaflet";

export interface Info {
  name: string;
  position: string;
}

export interface Item {
  type: "puzzles" | "badge" | "message" | "dead" | "puzzles-fake" | "checkpoint-start",
  url: string,
  level: number,
  name: string,
  description: string,
  timestamp: unix_timestamp,
}

type unix_timestamp = number;

export interface Items {
  items: Item[],
}

export interface Node {
  latLng: LatLngLiteral,
  type: "ordinary" | "junction" | "checkpoint",
  data?: string,
  tag?: string,
}

export type way = {latLng: LatLngLiteral[][], tag?: string};

export interface TeamState {
  nodes: Map<string, Node>;
  ways: Map<string, way>;
  state: Info;
  items: Item[];
}

export interface DiscoveryEvent {
  event: "badge-found" | "puzzles-found" | "nothing" | "checkpoint-start-visited";
  newItems: Item[];
}

export interface TeamPosition {
  teamName: string;
  position: Node;
  level: number;
}

export interface MessageWithTimestamp extends Message {
  timestamp: unix_timestamp;
}


export type MessageType = "success" | "fail" | "info";

export interface OutgoingMessage {
  recipient_id?: number;
  message: Message;
}

export interface Standings {
  standings: TeamStanding[],
}

export interface TeamStanding {
  rank: number,
  name: string,
  badges: {
    [key: number]: {
      [key: number]: unix_timestamp,
    }
  },
  badge_count: number,
  puzzles_count: number,
  maxTimestamp: unix_timestamp,
}

interface Message {
  content: string;
  type: MessageType;
}


export type BadgeClass = 'badge' | 'shrug' | 'get_puzzle';
