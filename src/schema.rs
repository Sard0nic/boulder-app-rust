// @generated automatically by Diesel CLI.

diesel::table! {
    app_users (id) {
        id -> Int4,
        api_id -> Nullable<Varchar>,
        username -> Varchar,
        password -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        firstname -> Nullable<Varchar>,
        lastname -> Nullable<Varchar>,
        avatar -> Nullable<Varchar>,
        setter -> Bool,
        admin -> Bool,
        created_at -> Date,
        updated_at -> Date,
    }
}

diesel::table! {
    attempts (id) {
        id -> Int4,
        user_id -> Int4,
        boulder_id -> Int4,
        created_at -> Date,
        updated_at -> Date,
        att_count -> Int4,
        topped -> Bool,
    }
}

diesel::table! {
    boulders (id) {
        id -> Int4,
        gradenum -> Int4,
        api_id -> Nullable<Varchar>,
        grade -> Int4,
        nr -> Int4,
        removed -> Bool,
        section -> Nullable<Varchar>,
        setter_id -> Int4,
        setter_two_id -> Nullable<Int4>,
        created_at -> Date,
        updated_at -> Date,
    }
}

diesel::joinable!(attempts -> app_users (user_id));
diesel::joinable!(attempts -> boulders (boulder_id));

diesel::allow_tables_to_appear_in_same_query!(
    app_users,
    attempts,
    boulders,
);
