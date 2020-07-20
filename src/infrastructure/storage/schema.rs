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
        ceo_id -> Int4,
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
        dob -> Nullable<Date>,
        salary -> Nullable<Int8>,
    }
}
