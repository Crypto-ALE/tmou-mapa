FROM rust:1.49 as builder
WORKDIR /usr/src/tmou-mapa
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/tmou-mapa
COPY --from=builder usr/src/tmou-mapa/target/release/tmou-mapa-backend .
COPY --from=builder usr/src/tmou-mapa/target/release/import-* /usr/src/tmou-mapa/
COPY --from=builder usr/src/tmou-mapa/templates/* /usr/src/tmou-mapa/templates/
COPY --from=builder usr/src/tmou-mapa/migrations/* /usr/src/tmou-mapa/migrations/
COPY --from=builder usr/src/tmou-mapa/static/ /usr/src/tmou-mapa/static/

CMD ["./tmou-mapa-backend"]
