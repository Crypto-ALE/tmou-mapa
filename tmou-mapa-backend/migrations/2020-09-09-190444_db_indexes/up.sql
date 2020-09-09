CREATE INDEX idx_ways_nodes_way_id ON ways_nodes(way_id);
CREATE INDEX idx_ways_nodes_node_id ON ways_nodes(node_id);
CREATE INDEX idx_ways_nodes_node_order ON ways_nodes(node_order ASC);
CREATE INDEX idx_teams_items_team_id ON teams_items(team_id);
CREATE INDEX idx_teams_items_item_name ON teams_items(item_name);
CREATE INDEX idx_items_item_level ON items("level" DESC);
CREATE INDEX idx_teams_team_id ON teams(team_id);
CREATE INDEX idx_teams_phrase ON teams(phrase);
