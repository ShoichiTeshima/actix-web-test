table! {
    users (id) {
        id -> Bigint,
        family_name -> Varchar,
        first_name -> Varchar,
        mail_address -> Varchar,
        password -> Varchar,
        phone_number -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}
