table! {
    items (id) {
        id -> Int4,
        #[sql_name = "type"]
        type_ -> Item_type,
        url -> Varchar,
        level -> Int2,
        label -> Nullable<Varchar>,
        description -> Nullable<Text>,
    }
}

table! {
    nodes (id) {
        id -> Int8,
        lat -> Varchar,
        lon -> Varchar,
        #[sql_name = "type"]
        type_ -> Node_type,
    }
}

table! {
    nodes_items (node_id, item_id) {
        node_id -> Int8,
        item_id -> Int4,
    }
}

table! {
    teams (id) {
        id -> Int4,
        team_id -> Int4,
        name -> Varchar,
        phrase -> Varchar,
        position -> Int8,
    }
}

table! {
    teams_items (team_id, item_id) {
        team_id -> Int4,
        item_id -> Int4,
        timestamp -> Nullable<Timestamptz>,
    }
}

table! {
    ways_nodes (way_id, node_id) {
        way_id -> Int8,
        node_id -> Int8,
    }
}

joinable!(teams -> nodes (position));
joinable!(teams_items -> teams (team_id));

allow_tables_to_appear_in_same_query!(
    items,
    nodes,
    nodes_items,
    teams,
    teams_items,
    ways_nodes,
);
