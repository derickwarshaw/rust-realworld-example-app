#[derive(Queryable)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct User {
    pub email: String,
    pub token: String,
    pub username: String,
    pub bio: String,
    pub image: String,
}

use super::schema::users;

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub token: &'a str,
    pub username: &'a str,
}