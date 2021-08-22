import {Item} from '../../types';
import { translations } from './translation';
import {checkSkip, skip} from './api';

export async function checkSkipHandler(secretPhrase?: string) {
  let allowed: boolean;
  try {
    ({allowed} = await checkSkip(secretPhrase));
  } catch (e) {
    alert(translations.error);
    console.error(e);
  }
  updateSkipControl(allowed);
}

function updateSkipControl(enable: boolean) {
  const skipEl = document.getElementById("skip");
  if (enable) {
    skipEl.removeAttribute('disabled');
    skipEl.classList.remove('disabled');
    skipEl.classList.add('enabled');
  } else {
    skipEl.setAttribute('disabled', 'disabled');
    skipEl.classList.remove('enabled');
    skipEl.classList.add('disabled');
  }
}

export async function initSkip(translateFunction: (translations: {[key: string]: string}) => void, skipCallback: (newItems: Item[]) => void, secretPhrase?: string): Promise<void> {
	translateFunction(translations);
    await checkSkipHandler(secretPhrase);
    setInterval(() => {checkSkipHandler(secretPhrase)}, 60000);

    document.getElementById('skip').onclick = async () => {
    const validate = window.confirm(translations.skip_confirmation);
    if (validate) {
      // skip puzzle
      try {
        let {newItems} = await skip(validate, secretPhrase);
        skipCallback(newItems);
        // skip used, disable control
        updateSkipControl(false);
      } catch (e) {
        alert(translations.error);
        console.error(e);
      }
    }
  }

}

