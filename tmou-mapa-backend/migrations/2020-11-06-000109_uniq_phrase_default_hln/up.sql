-- default node - Hlavni Nadrazi
ALTER TABLE teams ALTER COLUMN position SET DEFAULT 3325029225;

-- uniq phrase and external team id
ALTER TABLE teams ADD CONSTRAINT unique_team_phrase UNIQUE (phrase);
ALTER TABLE teams ADD CONSTRAINT unique_ext_team_id UNIQUE (team_id);
