import {LatLngLiteral} from "leaflet";

export interface Info {
  name: string;
  position: string;
}

export interface Item {
  type: "puzzles" | "badge" | "message" | "checkpoint",
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
  event: "badge-found" | "checkpoint-visited" | "nothing";
  newItems: Item[];
}
