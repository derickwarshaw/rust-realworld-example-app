#[derive(Queryable)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub token: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

use super::schema::users;

#[derive(Insertable)]
#[table_name="users"]
#[derive(Debug)]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub token: &'a str,
    pub username: &'a str,
}