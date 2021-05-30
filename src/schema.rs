table! {
    appointments (id) {
        id -> Uuid,
        slot -> Uuid,
        location -> Uuid,
        human_id -> Int4,
    }
}

table! {
    locations (id) {
        id -> Uuid,
        name -> Text,
        address -> Text,
        phone -> Text,
    }
}

table! {
    slots (id) {
        id -> Uuid,
        start_time -> Timestamp,
        end_time -> Timestamp,
        location -> Uuid,
        capacity -> Int4,
    }
}

joinable!(appointments -> locations (location));
joinable!(appointments -> slots (slot));
joinable!(slots -> locations (location));

allow_tables_to_appear_in_same_query!(
    appointments,
    locations,
    slots,
);
