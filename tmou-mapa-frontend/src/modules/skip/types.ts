import { Item } from "../../types";

export interface Skip {
  allowed: boolean;
}

export interface SkipResult {
  newItems: Item[];
}
