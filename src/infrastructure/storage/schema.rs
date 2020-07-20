table! {
    companies (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        address -> Nullable<Text>,
        post_code -> Nullable<Int4>,
        city -> Nullable<Varchar>,
        country -> Nullable<Varchar>,
        abn -> Nullable<Text>,
        ceo_id -> Nullable<Int4>,
    }
}

table! {
    employees (id) {
        id -> Int4,
        email -> Varchar,
        firsname -> Varchar,
        lastname -> Varchar,
        address -> Nullable<Text>,
        post_code -> Nullable<Int4>,
        city -> Nullable<Varchar>,
        country -> Nullable<Varchar>,
        dob -> Nullable<Timestamp>,
        salary -> Nullable<Int4>,
        company_id -> Int4,
    }
}

joinable!(companies -> employees (ceo_id));

allow_tables_to_appear_in_same_query!(
    companies,
    employees,
);
