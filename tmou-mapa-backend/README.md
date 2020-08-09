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
