diesel::table! {
    event (id) {
        id -> VarChar,
        partner_id -> Smallint,
        timestamp -> Timestamp,
        payload -> Text,
    }
}