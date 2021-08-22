import { Bonus } from "./types";


export async function fetchBonuses(): Promise<Bonus[]> {
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
