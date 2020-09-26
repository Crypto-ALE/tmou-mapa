CREATE TABLE messages (
  id SERIAL PRIMARY KEY,
  content TEXT NOT NULL,
  "type" VARCHAR(20) NOT NULL, -- info, fail, success
  "timestamp" TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE messages_teams (
  message_id INTEGER,
  team_id INTEGER, -- special team_id 0 is broadcast
  CONSTRAINT fk_message_id
      FOREIGN KEY(message_id)
	  REFERENCES messages(id),
  PRIMARY KEY (message_id, team_id)
);

CREATE INDEX idx_messages_teams_team_id ON messages_teams(team_id);
CREATE INDEX idx_messages_timestamp ON messages("timestamp" DESC);
