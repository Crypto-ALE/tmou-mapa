CREATE TABLE ways_nodes (
  way_id BIGINT,
  node_id BIGINT,
  CONSTRAINT fk_node_id
      FOREIGN KEY(node_id)
	  REFERENCES nodes(id),
  PRIMARY KEY (way_id, node_id)
);

CREATE TABLE nodes_items (
  node_id BIGINT,
  item_id INTEGER,
  CONSTRAINT fk_node_id
      FOREIGN KEY(node_id)
	  REFERENCES nodes(id),
  CONSTRAINT fk_item_id
      FOREIGN KEY(item_id)
	  REFERENCES items(id),
  PRIMARY KEY (node_id, item_id)
);

CREATE TABLE teams_items (
  team_id INTEGER,
  item_id INTEGER,
  "timestamp" TIMESTAMPTZ DEFAULT NOW(),
  CONSTRAINT fk_team_id
      FOREIGN KEY(team_id)
	  REFERENCES teams(id),
  CONSTRAINT fk_item_id
      FOREIGN KEY(item_id)
	  REFERENCES items(id),
  PRIMARY KEY (team_id, item_id)
);

ALTER TABLE teams DROP COLUMN position;
ALTER TABLE teams ADD COLUMN position BIGINT NOT NULL DEFAULT 3750367566;
ALTER TABLE teams ADD CONSTRAINT fk_position FOREIGN KEY(position) REFERENCES nodes(id);
