import {LatLngLiteral} from "leaflet";

export interface Info {
  position: string;
  ranking: number;
}

export interface Item {
  type: "puzzles" | "badge" | "message" | "checkpoint",
  url: String,
  level: number,
  name: String,
  description: String,
}

export interface Items {
  items: Item[],
}

export interface Node {
  latLng: LatLngLiteral,
  type: "ordinary" | "junction",
  data?: String,
}

export interface TeamState {
  nodes: Map<string, Node>;
  ways: LatLngLiteral[][];
  state: Info;
  items: Items;
}
