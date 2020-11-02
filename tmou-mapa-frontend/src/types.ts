import {LatLngLiteral} from "leaflet";

export interface Info {
  name: string;
  position: string;
}

export interface Item {
  type: "puzzles" | "badge" | "message" | "dead" | "puzzles-fake" | "checkpoint-start",
  url: String,
  level: number,
  name: String,
  description: String,
  timestamp: unix_timestamp,
}

type unix_timestamp = number;

export interface Items {
  items: Item[],
}

export interface Node {
  latLng: LatLngLiteral,
  type: "ordinary" | "junction",
  data?: String,
}

export type way = LatLngLiteral[][];

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

export interface Bonus {
  url: String,
  label: String,
  description: String,
}

export type MessageType = "success" | "fail" | "info";

export interface OutgoingMessage {
  recipient_id?: number;
  message: Message;
}

export interface Standings {
  badge_labels: string[],
  standings: TeamStanding[],
}

export interface TeamStanding {
  rank: number,
  name: string,
  badge_timestamps: {
    [key: string]: unix_timestamp
  }
}

interface Message {
  content: string;
  type: MessageType;
}

export interface Skip {
  allowed: boolean;
}
