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
