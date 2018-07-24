table! {
    account (id) {
        id -> Int4,
        name -> Varchar,
        passhash -> Nullable<Varchar>,
    }
}

table! {
    node (id) {
        id -> Int4,
        tree -> Int4,
        text -> Text,
        parent -> Nullable<Int4>,
        author -> Nullable<Int4>,
    }
}

table! {
    tree (id) {
        id -> Int4,
        name -> Text,
        creator -> Nullable<Int4>,
    }
}

joinable!(node -> account (author));
joinable!(node -> tree (tree));
joinable!(tree -> account (creator));

allow_tables_to_appear_in_same_query!(
    account,
    node,
    tree,
);
