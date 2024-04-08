use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::twitch_channel)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TwitchChannel {
    pub id: String,
    pub login: String,
    pub name: String,
    pub language: String,
    pub delay: i32,
}


#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::twitch_login_process)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LoginProcess {
    pub state: String,
    pub scope: String,
    pub code: Option<String>,  // If code exists, permission was given to login as user, otherwise no
    pub is_bot: bool,
    pub is_broadcaster: bool,
}