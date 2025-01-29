diesel::table! {
    users (id) {
        id -> Serial,
        username -> Varchar,
        password -> Varchar,
        remember_token -> Nullable<Varchar>,
    }
}
