CREATE TYPE node_type AS ENUM (
  'junction',
  'ordinary');

CREATE TYPE item_type AS ENUM (
  'puzzles',
  'badge',
  'message');

CREATE TABLE nodes (
  id BIGINT PRIMARY KEY,
  lat VARCHAR NOT NULL,
  lon VARCHAR NOT NULL,
  "type" node_type NOT NULL
);

CREATE TABLE items (
  id SERIAL PRIMARY KEY,
  "type" item_type NOT NULL,
  url VARCHAR NOT NULL,
  "level" SMALLINT NOT NULL,
  label VARCHAR,
  description TEXT
);
