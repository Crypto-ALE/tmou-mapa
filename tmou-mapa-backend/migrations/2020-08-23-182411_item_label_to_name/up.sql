ALTER TABLE nodes_items DROP CONSTRAINT fk_item_id;
ALTER TABLE teams_items DROP CONSTRAINT fk_item_id;
ALTER TABLE items DROP CONSTRAINT items_pkey;
ALTER TABLE items DROP COLUMN id;

ALTER TABLE items RENAME COLUMN label TO name;
ALTER TABLE items ALTER COLUMN name SET NOT NULL;
ALTER TABLE items ADD CONSTRAINT items_pkey PRIMARY KEY(name);

ALTER TABLE nodes_items DROP CONSTRAINT nodes_items_pkey;
ALTER TABLE nodes_items RENAME COLUMN item_id TO item_name;
ALTER TABLE nodes_items ALTER COLUMN item_name TYPE VARCHAR;
ALTER TABLE nodes_items ADD CONSTRAINT nodes_items_pkey PRIMARY KEY(node_id, item_name);
ALTER TABLE nodes_items ADD CONSTRAINT fk_nodes_items_item_name
  FOREIGN KEY(item_name) REFERENCES items(name);

ALTER TABLE teams_items DROP CONSTRAINT teams_items_pkey;
ALTER TABLE teams_items RENAME COLUMN item_id TO item_name;
ALTER TABLE teams_items ALTER COLUMN item_name TYPE VARCHAR;
ALTER TABLE teams_items ADD CONSTRAINT teams_items_pkey PRIMARY KEY(team_id, item_name);
ALTER TABLE teams_items ADD CONSTRAINT fk_teams_items_item_name
  FOREIGN KEY(item_name) REFERENCES items(name);
