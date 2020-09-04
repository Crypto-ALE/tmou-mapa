# TMOU Mapa Backend

Aplikace pro servírování mapových dlaždic, uzlů a správy týmů pro online verzi TMOU.

## Požadavky

- rust v nightly verzi 
- další knihovny zapsány v cargo.toml
- běžící instance PostgreSQL

## Spuštění

1. `git clone https://github.com/miiila/tmou-mapa` a přesunout se do adresáře `tmou-mapa/tmou-mapa-backend`
2. `rustup override set nightly` 
3. `ROCKET_DATABASES='{postgres={url="postgres://USER:PASSWORD@SERVER:PORT/DB_NAME"}}' cargo run` 


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
2. Naimportovat mapová data: `{target}/import-osm-data pubfiles/tiles/osmdata.xml`
3. Naimportovat šifrová data: `{target}/import-game-data sample_game_data.xml`
4. Vytvořit v tabulce teams tým

## Neformální popis API

* `GET /game/<secret_phrase>` vrátí informace o týmu: Název, polohu, viditelné uzly a inventář
* `POST /game/<secret_phrase>` s tělem `{"nodeId":<id>}` přesune tým na daný uzel
* `GET /game/<secret_phrase>/discover` prozkoumá uzel, na kterém tým stojí, provede příslušné změny stavu a vrátí seznam objevených věcí

## Stavový automat objevování uzlů

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

