table! {
    companies (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        address -> Nullable<Text>,
        city -> Nullable<Varchar>,
        abn -> Nullable<Text>,
        country -> Nullable<Varchar>,
    }
}
