ALTER TABLE teams_items DROP CONSTRAINT fk_teams_items_item_name;
ALTER TABLE teams_items DROP CONSTRAINT teams_items_pkey;
ALTER TABLE teams_items RENAME COLUMN item_name TO item_id;
ALTER TABLE teams_items ALTER COLUMN item_id TYPE INTEGER USING item_id::INTEGER;
ALTER TABLE teams_items ADD CONSTRAINT teams_items_pkey PRIMARY KEY(team_id, item_id);

ALTER TABLE nodes_items DROP CONSTRAINT fk_nodes_items_item_name;
ALTER TABLE nodes_items DROP CONSTRAINT nodes_items_pkey;
ALTER TABLE nodes_items RENAME COLUMN item_name TO item_id;
ALTER TABLE nodes_items ALTER COLUMN item_id TYPE INTEGER USING item_id::INTEGER;
ALTER TABLE nodes_items ADD CONSTRAINT nodes_items_pkey PRIMARY KEY(node_id, item_id);

ALTER TABLE items DROP CONSTRAINT items_pkey;
ALTER TABLE items ALTER COLUMN name DROP NOT NULL;
ALTER TABLE items RENAME COLUMN name TO label;

ALTER TABLE items ADD COLUMN id SERIAL PRIMARY KEY;
ALTER TABLE nodes_items ADD CONSTRAINT fk_item_id
  FOREIGN KEY(item_id) REFERENCES items(id);
ALTER TABLE teams_items ADD CONSTRAINT fk_item_id
  FOREIGN KEY(item_id) REFERENCES items(id);
