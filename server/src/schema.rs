table! {
    node (id) {
        id -> Int4,
        tree -> Nullable<Int4>,
        text -> Text,
    }
}

table! {
    tree (id) {
        id -> Int4,
        name -> Text,
    }
}

joinable!(node -> tree (tree));

allow_tables_to_appear_in_same_query!(
    node,
    tree,
);
