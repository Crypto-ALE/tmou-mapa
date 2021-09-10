import {Bonus} from './types';
import {fetchBonuses} from './api';
import {translations} from './translation';

function drawBonuses(items: Bonus[]) {
	const bonuses = items
	  .map(({url, description}) => `<li><a href="${url}" target="_blank">${description}</a>`);

	if (bonuses.length > 0) {
	  document.querySelector('#bonuses>#bonuses-list').innerHTML = `<ul>${bonuses.join('')}</ul>`;
	}
}

async function bonusesHandler() {
	const bonuses = await fetchBonuses();
	drawBonuses(bonuses);
}

export async function initBonuses(translateFunction: (translations: {[key: string]: string}) => void): Promise<void> {
	translateFunction(translations);
    await bonusesHandler();
    setInterval(bonusesHandler, 60000);
}
