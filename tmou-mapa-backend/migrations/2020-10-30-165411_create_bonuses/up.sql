CREATE TABLE bonuses (
  id SERIAL PRIMARY KEY,
  url VARCHAR NOT NULL,
  label VARCHAR NOT NULL,
  display_time TIMESTAMPTZ NOT NULL,
  description TEXT
);

CREATE INDEX idx_bonuses_display_time ON bonuses(display_time);
