DROP TABLE ways_nodes;
DROP TABLE nodes_items;
DROP TABLE teams_items;
ALTER TABLE teams DROP CONSTRAINT fk_position;
ALTER TABLE teams DROP COLUMN position;
ALTER TABLE teams ADD COLUMN position VARCHAR NOT NULL DEFAULT '3750367566';
