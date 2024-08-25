// @generated automatically by Diesel CLI.

diesel::table! {
    log (id) {
        id -> Int4,
        ts -> Timestamptz,
        message -> Text,
        span_id -> Int4,
    }
}

diesel::table! {
    span (id) {
        id -> Int4,
        ts_start -> Timestamptz,
        ts_end -> Timestamptz,
        operation_name -> Text,
        parent_span_id -> Nullable<Int4>,
        external_uuid -> Nullable<Uuid>,
    }
}

diesel::table! {
    test_registration (id) {
        id -> Int4,
        blob_url -> Text,
        metadata -> Nullable<Jsonb>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    test_version (id) {
        id -> Int4,
        name -> Text,
        version -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        test_registration_id -> Int4,
    }
}

diesel::joinable!(log -> span (span_id));
diesel::joinable!(test_version -> test_registration (test_registration_id));

diesel::allow_tables_to_appear_in_same_query!(log, span, test_registration, test_version,);
