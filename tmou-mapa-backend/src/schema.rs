table! {
    items (name) {
        #[sql_name = "type"]
        type_ -> Varchar,
        url -> Varchar,
        level -> Int2,
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

table! {
    messages (id) {
        id -> Int4,
        content -> Text,
        #[sql_name = "type"]
        type_ -> Varchar,
        timestamp -> Nullable<Timestamptz>,
    }
}

table! {
    messages_teams (message_id, team_id) {
        message_id -> Int4,
        team_id -> Int4,
    }
}

table! {
    nodes (id) {
        id -> Int8,
        #[sql_name = "type"]
        type_ -> Varchar,
        lat -> Float4,
        lon -> Float4,
    }
}

table! {
    nodes_items (node_id, item_name) {
        node_id -> Int8,
        item_name -> Varchar,
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
    teams_items (team_id, item_name) {
        team_id -> Int4,
        item_name -> Varchar,
        timestamp -> Nullable<Timestamptz>,
    }
}

table! {
    ways_nodes (way_id, node_id) {
        way_id -> Int8,
        node_id -> Int8,
        node_order -> Int2,
    }
}

joinable!(messages_teams -> messages (message_id));
joinable!(nodes_items -> items (item_name));
joinable!(teams -> nodes (position));
joinable!(teams_items -> items (item_name));
joinable!(teams_items -> teams (team_id));

allow_tables_to_appear_in_same_query!(
    items,
    messages,
    messages_teams,
    nodes,
    nodes_items,
    teams,
    teams_items,
    ways_nodes,
);
