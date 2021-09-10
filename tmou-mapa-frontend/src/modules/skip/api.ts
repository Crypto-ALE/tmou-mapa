import { Skip,SkipResult } from "./types";

export async function skip(verified: boolean, secretPhrase?: string): Promise<SkipResult> {
  const url = secretPhrase ? `/game/${secretPhrase}/skip` : '/game/skip';
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

export async function checkSkip(secretPhrase?: string): Promise<Skip> {
  const url = secretPhrase ? `/game/${secretPhrase}/skip` : '/game/skip';
  const res = await fetch(url);

  if (!res.ok) {
    throw new Error("Skip check is not working, is game running?");
  }

  return (await res.json());
}
