# TMOU Mapa Backend

Aplikace pro servírování mapových dlaždic, uzlů a správy týmů pro online verzi TMOU.

## Požadavky

- rust v nightly verzi 
- další knihovny zapsány v cargo.toml
- běžící instance PostgreSQL

## Spuštění

### Lokální build
1. `git clone https://github.com/Crypto-ALE/tmou-mapa` a přesunout se do adresáře `tmou-mapa/tmou-mapa-backend`
2. `rustup override set nightly` 
3. `ROCKET_DATABASES='{postgres={url="postgres://USER:PASSWORD@SERVER:PORT/DB_NAME"}}' cargo run` 

### Docker
Docker image je publikovaný na docker hub, není třeba ho sestavit lokálně.
```
docker run \
-e TMOU_GAME_EXECUTION_MODE=Auto \
-e TMOU_GAME_START=2020-11-06T20:00:00+01:00 \
-e TMOU_GAME_END=2020-11-07T12:00:00+01:00 \
-e ROCKET_DATABASES='{...}'
--net host \
--name tmou \
cryptoale/itmou
```

## Proměnné prostředí

Název | Formát | příklad | popis
---|---|---|---
TMOU_GAME_EXECUTION_MODE | On/Off/Auto | Auto | On - hra je zapnuta, Off, hra je vypnuta, Auto - hra se řídí proměnnými START a END (viz níže)
TMOU_GAME_START | Čas podle [RFC3339](https://tools.ietf.org/html/rfc3339) | 2020-11-06T20:00:00+01:00 | Začátek hry; Čas, před kterým backend vrací chybu 
TMOU_GAME_END | Čas podle [RFC3339](https://tools.ietf.org/html/rfc3339) | 2020-11-07T12:00:00+01:00 | Konec hry; Čas, po kterém už není možno objevovat uzly
TMOU_GAME_RATE_LIMIT_CHECKING | On/Off | On | Zapínání kontroly minimálního času mezi dvěma požadavky týmu na přesun (viz níže)
TMOU_GAME_RATE_LIMIT_IN_MS | integer | 1000 | Minimalní čas mezi dvěma požadavky týmu na přesun (proměnná neexistuje = 1000)
ROCKET_DATABASES | objekt | {postgres={url="postgres://USER:PASSWORD@SERVER:PORT/DB_NAME"}} | Konfigurační objekt frameworku Rocket se spojením do databází (může jich být víc)
ADMIN_USERNAME | string | admin | Uživatelské jméno pro vstup do admin sekce
ADMIN_PASSWORD | string | admin | Heslo pro vstup do admin sekce
BYPASS_AUTH | 1/Nic | 1 | Preskočení autorizace pro přístup k admin sekci **POUZE PRO TEST**
JWT_TOKEN | string | your-256-bit-secret | Secret pro dekódování [JWT Tokenu](https://jwt.io/) použitého pro sdílení session s webem TMOU _pouze  TMOU_
LOGIN_REDIRECT | url | https://www.tmou.cz | Adresa pro přihlášení týmu při sdílené session _pouze TMOU_
PROJECT_PATH | string | tmou-mapa-backend | Cesta k adresáři s backendem _pouze Heroku_
ROCKET_ENV | dev/stage/prod | dev | Specifikace prostředí pro [Rocket](https://rocket.rs/v0.4/guide/configuration/#environment)
HOST | string | i.tmou.cz | Adresa webu, používaná pro přesměrování z http na https
BONUSES_ENABLED | string | 1/0 | Zapíná modul pro bonusy (např. iTMOU 2020)

## Přihlášení týmu
Tým se může přihlásit dvěma způsoby - buď může využít své přihlášené z webu TMOU nebo může přistupovat na svou url ve tvaru `base-url/{url-tymu}`, kde `url-tymu` odpovídá sloupci `phrase` v databázi. Ve výchozím stavu je toto přihlášení přes url považování za admin přístup a chráněno stejným přihlášením.

_Pro lepší nasazení a použití i mimo TMOU se přihlášení asi brzy změní._

## Migrace

Databáze se vyvíjí postupně pomocí migrací, tedy při každé změně je třeba napsat SQL tak, aby bylo kompatibilní se stávající strukturou. Na migrace se používá ORM - Diesel. Migrace se provedou automaticky při spuštění aplikace.

Postup:
0. Nainstalovat Disel CLI (`cargo install diesel_cli --no-default-features --features postgres`)
1. Příkaz `diesel migration generate nova_tabulka` vytvoří složku s názvem a dvěma soubory - up.qsl a down.sql
2. Do up.sql se píše, co je třeba provést (vytvořit novou tabulku, přidat index apod.) do down.sql postup pro rollback (typicky smazat tabulku)
3. `DATABASE_URL=postgres://USER:PASSWORD@SERVER:PORT/DB diesel migration run` provede migrace nebo skončí s chybou; taky aktualizuje schema.rs
4. `DATABASE_URL=postgres://postgres:tmou_rulez@localhost:15432/tmou diesel migration redo` vyzkouší rollback a znovu aplikuje migraci

## Naplnění databáze

Databáze se plní command-line tooly, které jsou součástí projektu.

Postup:

1. Smazat data v databázi
2. Naimportovat mapová data: `{target}/import-osm-data <osm_file.xml> <default_tag> `
   * uzlům a cestám, které nemají svůj tag (viz níže), se přiřadí <default_tag>
   * pokud není <default_tag> zadaný, použije se prázdný řetězec
3. Naimportovat šifrová data: `{target}/import-game-data sample_game_data.xml`
4. Vytvořit v tabulce teams tým

### Struktura souboru OSM DATA
Používá se standardní Open Street Map XML, detaily [zde](https://wiki.openstreetmap.org/wiki/OSM_XML).
Pokud obsahuje uzel nebo cesta v datech element tag s klíčem "tag", použije se při importu jeho hodnota jako tag, jinak se použije <default_tag> z příkazové řádky. Např. pro soubor:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<osm>
  <node id="1" lat="49.1" lon="16.6">
  	  <tag k="tag" v="Kremilek"/>
  </node>
  <node id="2" lat="49.2" lon="16.7"/>
</osm>
``` 

po zavolání `import-osm-data file.xml Vochomurka` bude mít uzel č. 1 tag `Kremilek` a uzel č. 2 tag `Vochomurka`.



### Struktura souboru GAME DATA
XML popisuje herní prvky, z implementačních důvodů rozdelěné do dvou kategorií (`items`, `bonuses`).

#### Items
Jedná se o předměty, které se dají najít a vyzvednout na mapě.

Atributy:
- name - hlavní identifikátor v databázi
- type - typ předmětu, viz níže
- description - zobrazený popis předmětu v UI
- url - cíl odkazu z inventáře
- level - úroveň předmětu, určená pro ověřování podmínky vyzvednutí (obvykle musí mít tým v invetáři předmět levelu o 1 nižší, než ten, který chce vyzvednout)

Všechny prvky typu item navíc obsahují kolekci prvků `node` s atributem `id`, který popisuje id uzlu ze souboru OSM DATA, na kterém se daný předmět nachází.

Typy předmětů:
- puzzles - zadání šifry
- puzzles-fake - řešení šifry, `name` musí odpovídat příslušné šifře plus připojit `-fake`, např. `puzzles-4-fake`; url je stejné jako url původní šifry
- checkpoint-start - místo, kde je možné vyzvednout řešení šifer, *vyžaduje specifickou implementaci*
- dead - výsledek přeskočení šifry, url odkazuje zadání šifry s level+1
- badge - odznáček za vyluštěný bonus, url je prázdné, podle `name` se hledá příslušený obrázek 

#### Bonuses
Bonusy nejsou na mapě, ale zpřístupňují se v příslušném čase.

Atributy:
- label - hlavní identifikátor v databázi
- description - zobrazený popis bonusu v UI
- url - odkaz na zadání bonusu
- display-time - čas zobrazení bonusu (pro všechny týmy stejný) ve formátu [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)

## Neformální popis API

* `GET /game/<secret_phrase>` vrátí informace o týmu: Název, polohu, viditelné uzly a inventář
* `POST /game/<secret_phrase>` s tělem `{"nodeId":<id>}` přesune tým na daný uzel
* `GET /game/<secret_phrase>/discover` prozkoumá uzel, na kterém tým stojí, provede příslušné změny stavu a vrátí seznam objevených věcí

## Stavový automat objevování uzlů

**OBSOLETE**

legenda:
* Bxy - badge levelu x číslo y
* Px - puzzles levelu x
* C - checkpoint (levelu 0, kde jsou většinou všechny sady šifer)

Discovery vždy vrátí název eventu a nově získané objekty. Sloupec nový inventář v následující tabulce obsahuje starý inventář plus nově získané objekty. Celý nový inventář je pak vrácen při následném volání info.

inventář | obsah uzlu | nový inventář | event | pozn.
---|---|---|---|---
cokoliv | nic | beze změn | nothing | ani smrt nebere...
nic | C, P1, P2,... | P1 | checkpoint-discovered | jsem na kontrole, začínám hru, dostávám šifry sady 1
P1, B11 | B11 | beze změn | badge-found | už mám, ale bylo to tady
P1, B11 | B21 | beze změn | nic | nevidím level 2
P1, B11 | B12 | P1, B11, B12 | badge-found | našel jsem nový
P1, B11, B12, B13 | B14 | beze změn | badge-found | čtvrtý už nepotřebuju, ale je tady
P1, B11, B12 | C, P1, P2,... | beze změn | checkpoint-discovered | jsem na kontrole, ale nemám na nový level
P1, B11, B12, B13 | C, P1 | beze změn | checkpoint-discovered | jsem na kontrole, kde nemají moji sadu, nic nedostávám
P1, B11, B12, B13 | C, P2,... | P1, B11, B12, B13, P2 | checkpoint-discovered | jsem na kontrole, dostal jsem novou sadu

