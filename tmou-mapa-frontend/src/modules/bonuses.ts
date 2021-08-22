interface Bonus {
  url: string,
  label: string,
  description: string,
}

function drawBonuses(items: Bonus[]) {
	const bonuses = items
	  .map(({url, description}) => `<li><a href="${url}" target="_blank">${description}</a>`);

	if (bonuses.length > 0) {
	  document.querySelector('#bonuses>#bonuses-list').innerHTML = `<ul>${bonuses.join('')}</ul>`;
	}
}

export async function bonusesHandler() {
	const bonuses = await fetchBonuses();
	drawBonuses(bonuses);
}

async function fetchBonuses(): Promise<Bonus[]> {
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
