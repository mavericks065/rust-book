table! {
    check_points (id) {
        id -> Int4,
        comments -> Nullable<Text>,
        manager_actions -> Nullable<Text>,
        managee_actions -> Nullable<Text>,
        highlights -> Nullable<Text>,
        mood -> Nullable<Int4>,
        goals -> Nullable<Text>,
        previous_actions_status -> Nullable<Text>,
        check_point_date -> Nullable<Timestamp>,
    }
}

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
        created_at -> Nullable<Timestamp>,
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
        created_at -> Nullable<Timestamp>,
        follow_up_ids -> Nullable<Array<Int4>>,
    }
}

table! {
    follow_ups (id) {
        id -> Int4,
        managee_id -> Nullable<Int4>,
        check_points_ids -> Nullable<Array<Int4>>,
    }
}

joinable!(companies -> employees (ceo_id));
joinable!(follow_ups -> employees (managee_id));

allow_tables_to_appear_in_same_query!(check_points, companies, employees, follow_ups,);
