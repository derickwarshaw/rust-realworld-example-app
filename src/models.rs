extern crate chrono;

use super::schema::users;
use super::schema::articles;

use chrono::prelude::*;

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

#[derive(Insertable)]
#[table_name="users"]
#[derive(Debug)]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub token: &'a str,
    pub username: &'a str,
}

#[derive(Queryable)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Article {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub created: NaiveDateTime,
    pub updated: Option<NaiveDateTime>,
    pub author: i32,
}