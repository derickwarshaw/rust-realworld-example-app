extern crate chrono;

use super::schema::*;

use chrono::prelude::*;

#[derive(Identifiable, Queryable, Associations)]
#[derive(Serialize, Deserialize)]
#[has_many(favoritedarticles)]
#[derive(Debug)]
#[cfg(feature = "diesel")]
pub struct User {
    pub id: i32,
    pub email: String,
    pub token: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    //pub following: Option<bool>
}

#[derive(Identifiable, Queryable, Associations)]
#[derive(Serialize, Deserialize)]
#[table_name = "followings"]
#[primary_key(id)]
#[belongs_to(User, foreign_key = "followingid")]
//#[belongs_to(User, foreign_key = "followerid")]
pub struct Following {
    pub id: i32,
    pub followingid: i32,
    pub followerid: i32,
}

#[derive(Insertable)]
#[derive(Debug)]
#[table_name="followings"]
pub struct NewFollowing {
    pub followingid: i32,
    pub followerid: i32,
}

#[allow(non_snake_case)]
#[derive(Queryable)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
pub struct AdvancedArticle {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub createdAt: NaiveDateTime,
    pub updatedAt: Option<NaiveDateTime>,
    pub author: i32,
    pub favorited: bool,
    pub favoritesCount: i64,
    pub tagList: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[table_name = "articles"]
#[derive(AsChangeset)]
#[derive(Identifiable)]
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct UpdatedArticle<'a> {
    pub id: i32,
    pub slug: &'a str,
    pub title: &'a str,
    pub description: &'a str,
    pub body: &'a str,
    pub author: i32,
    pub createdat: NaiveDateTime,
    pub updatedat: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "users"]
#[derive(Debug)]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub token: &'a str,
    pub username: &'a str,
}

#[derive(Insertable)]
#[table_name = "users"]
#[derive(AsChangeset)]
#[derive(Identifiable)]
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct UpdatedUser<'a> {
    pub id: i32,
    pub email: &'a str,
    pub token: &'a str,
    pub username: &'a str,
    pub image: &'a str,
    pub bio: &'a str,
}

#[derive(Identifiable, Queryable, Associations)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[table_name = "comments"]
#[allow(non_snake_case)]
#[belongs_to(Article, foreign_key = "articleid")]
pub struct Comment {
    pub id: i32,
    pub createdAt: NaiveDateTime,
    pub updatedAt: Option<NaiveDateTime>,
    pub body: String,
    //author: Profile,
    pub author: i32,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub articleid : i32,
}

#[derive(Insertable)]
#[table_name = "comments"]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct NewComment<'a> {
    pub createdat: NaiveDateTime,
    pub updatedat: Option<NaiveDateTime>,
    pub body:  &'a str,
    pub author: i32,
    pub articleid: i32,
}

#[derive(Insertable)]
#[table_name = "articles"]
#[derive(Debug)]
pub struct NewArticle<'a> {
    pub title: &'a str,
    pub slug: &'a str,
    pub description: &'a str,
    pub body: &'a str,
    pub author: i32,
    pub createdat: NaiveDateTime,
    pub updatedat: Option<NaiveDateTime>,
    //pub tagList: &'a Vec<str>,
}

#[derive(Queryable)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct IncomingArticle {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tagList: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct IncomingArticleResult {
    pub article: IncomingArticle,
}

#[allow(non_snake_case)]
#[derive(Identifiable, Queryable, Associations)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
#[has_many(articletags)]
#[has_many(favoritedarticles)]
#[has_many(comments)]
#[table_name = "articles"]
#[belongs_to(User, foreign_key = "author")]
pub struct Article {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub createdAt: NaiveDateTime,
    pub updatedAt: Option<NaiveDateTime>,
    pub author: i32,
}

#[derive(Identifiable, Queryable, Associations)]
#[derive(Serialize, Deserialize)]
#[table_name = "articletags"]
#[primary_key(id)]
#[belongs_to(Article, foreign_key = "articleid")]
#[belongs_to(Tag, foreign_key = "tagid")]
pub struct ArticleTag {
    pub id: i32,
    pub articleid: i32,
    pub tagid: i32,
}

#[derive(Identifiable, Queryable, Associations)]
#[derive(Serialize, Deserialize)]
#[table_name = "favoritedarticles"]
#[primary_key(id)]
#[belongs_to(Article, foreign_key = "articleid")]
#[belongs_to(User, foreign_key = "userid")]
pub struct ArticleUser {
    pub id: i32,
    pub articleid: i32,
    pub userid: i32,
}

#[derive(Insertable)]
#[derive(Debug)]
#[table_name="articletags"]
pub struct NewArticleTag {
    pub articleid: i32,
    pub tagid: i32,
}

#[derive(Insertable)]
#[derive(Debug)]
#[table_name="favoritedarticles"]
pub struct NewArticleUser {
    pub articleid: i32,
    pub userid: i32,
}

#[derive(Identifiable, Queryable, Associations)]
#[has_many(articletags)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Tag {
    pub id: i32,
    pub tag: String,
}
