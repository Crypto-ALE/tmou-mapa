# TMOU 22 - mapa

Aplikace pro internetovou kvalifikaci/hru. Umožňuje procházet stanoviště po Brně, vyzvedávat na nich šifry a možná i něco dalšího. Má dvě části.

## Backend
Napsáno v jazyku Rust, použitý framework Rocket. Databáze bude. Více v [README](./tmou-mapa-backend/README.md). 

Pro standardní použití frontendu není třeba dělat nic, v repozitáři je kopie šablony i skriptů.

## Frontend
Využívá mapové knihovny leaflet, do které vykresluje vlastní body a cesty. Kvůli agresivnímu přístupu Leafletu k DOMu nevyužívá žádného frameworku (React, Vue, Svelte), ale nechává co nejvíce věcí na Leafletu a jen připojuje handlery k částem UI. Je napsána v TypeScriptu a transpiluje se přes Rollup. Ideálně bude používat BEM a SASS pro css (až to bude potřeba). Pro lokální vývoje je potřeba Node.js.

