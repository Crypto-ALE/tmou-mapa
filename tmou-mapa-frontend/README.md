# TMOU Mapa Frontend

Aplikace pro renderování mapy

Pro vývoje je potřeba nainstalovat závislosti (`npm install`) a vytvořit bundle, který se zkopíruje do správných míst ve složce backend (`rollup -c rollup.config.js -w`).

## Moduly
Aplikaci zkoušíme převést na systém modulů, které je možné vypínat/zapínat podle potřeby. Jejich zapnutí **řídí BACKEND** podle konfiguračního souboru (zatím ENV vars). Navrhovaná adresářová struktura:
```
* modules
  * název modulu
	* api.ts - vše, co potřebuje komunikovat s backendem
	* types.ts - definice typů
	* index.ts - exportuje ideálně jednu funkci, která se použije v hlavním index.ts
	* index.html - html šablona, která se pomocí direktivy {% include %} vloží v hlavní html šabloně
	* translation.ts - překladové řetězce
```

### Co je potřeba pro vytvoření nového modulu§
- připravit adresářovou strukturu (viz výše)
- do souboru [main.rs](../tmou-mapa-backend/src/main.rs) přidat do contextu proměnnou, podle které se řídí zapnutí modulu
- do [konfigurace rollupu](./rollup-config.js) přidat nastavení pro kopírování html šablony a její přejmenování
- do hlavní [html šablony](./index.html) přidat opodmínkovanou direktivu `include` pro novou šablonu

Ideální je taky někam do html šablony modulu vložit nějaký `data-<modul>Enabled="1"`, aby bylo možné z JS zjistit, jestli je modul zapnutý. (TODO: vyřešit nějak standardněji)
