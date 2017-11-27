#![feature(custom_attribute)]

extern crate bson;

extern crate iis;
extern crate hyper;

extern crate serde;
extern crate serde_json;

extern crate chrono;

extern crate crypto;

extern crate futures;
extern crate tokio_core;

#[cfg(feature = "tiberius")]
extern crate tiberius;

extern crate toml;

extern crate reroute;

extern crate jwt;

extern crate futures_state_stream;

extern crate slug;

use hyper::server::{Request, Response};
use reroute::Captures;

use slug::slugify;

use super::*;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[allow(non_snake_case)]
struct ArticlesResult {
    articles: Vec<Article>,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct ArticleResult {
    article: AdvancedArticle,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct TagsResult {
    tags: Vec<String>,
}

impl Container<Article> for ArticlesResult {
    fn create_new_with_items(articles: Vec<Article>) -> ArticlesResult {
        ArticlesResult { articles: articles }
    }
}

pub fn get_tag_names<'a>(a: &str) -> Option<TagsResult> {
        use models::Tag;
        use schema::tags;

        let conn = establish_connection();
        
        let tags_result = 
            tags::table
            .load::<Tag>(&conn)
            .expect(
                "Error loading tags",
            );

        let result = tags_result.into_iter().map(|t| t.tag).collect();
        Some(TagsResult { tags: result,})
    }

fn get_tag_ids(tags_vec: Vec<String>) -> Vec<i32> {
        use models::Tag;

        let mut tags_result = Vec::new();
        let connection = establish_connection();

        for tag_str in tags_vec {
            use schema::tags::dsl::*;

            let tag_from_db: Tag = tags
                .filter(tag.eq(tag_str))
                .first(&connection)
                .unwrap();
            &tags_result.push(tag_from_db.id);
        }
        tags_result
    }


#[cfg(feature = "tiberius")]
static ARTICLE_SELECT : &'static str = r#"
  SELECT Slug, Title, [Description], Body, Created, Updated, Users.UserName, Users.Bio, Users.[Image], 
                (SELECT COUNT(*) FROM Followings WHERE FollowerId=@logged AND Author=FollowingId) as [Following],
                (SELECT COUNT(*) FROM FavoritedArticles WHERE ArticleId = @id ) as FavoritesCount,
                (SELECT COUNT(*) FROM FavoritedArticles WHERE UserId = @logged ) as PersonalFavoritesCount,
				(SELECT STRING_AGG(Tag, ',') FROM [Tags] inner join ArticleTags on ArticleTags.TagId = Tags.Id where ArticleId=@id)  as Tags
                FROM Articles INNER JOIN Users on Author=Users.Id  WHERE Articles.Id = @id
"#;

#[cfg(feature = "tiberius")]
fn get_simple_article_from_row(row: tiberius::query::QueryRow) -> Option<Article> {
    let slug: &str = row.get(0);
    let title: &str = row.get(1);
    let description: &str = row.get(2);
    let body: &str = row.get(3);
    let created: chrono::NaiveDateTime = row.get(4);
    let updated: Option<chrono::NaiveDateTime> = row.get(5);
    let user_name: &str = row.get(6);
    let bio: Option<&str> = row.get(7);
    let image: Option<&str> = row.get(8);
    let f: i32 = row.get(9);
    let following: bool = f == 1;
    let favorites_count: i32 = row.get(10);
    let personal_favorite_count: i32 = row.get(11);
    let favorited: bool = personal_favorite_count > 0;
    let tags_combined: &str = row.get(12);

    let profile = Profile {
        username: user_name.to_string(),
        bio: bio.map(|s| s.to_string()),
        image: image.map(|s| s.to_string()),
        following: following,
    };

    let result = Article {
        slug: slug.to_string(),
        title: title.to_string(),
        description: description.to_string(),
        body: body.to_string(),
        tagList: tags_combined.split(",").map(|q| q.to_string()).collect(),
        createdAt: created,
        updatedAt: updated,
        favorited: favorited,
        favoritesCount: favorites_count,
        author: profile,
    };
    Some(result)
}

#[cfg(feature = "tiberius")]
fn get_article_from_row(row: tiberius::query::QueryRow) -> Option<CreateArticleResult> {
    Some(CreateArticleResult {
        article: get_simple_article_from_row(row).unwrap(),
    })
}

#[cfg(feature = "diesel")]
pub fn create_article_tag<'a>(new_article : AdvancedArticle) {
    
    //use diesel::associations::HasTable;
    
    let connection = establish_connection();
    let tag_ids = get_tag_ids(new_article.tagList);
    for tag_id in tag_ids {
        
        let new_relationship = NewArticleTag {
            tagid : tag_id,
            articleid : new_article.id
        };

        use schema::articletags;
        //use diesel::associations::HasTable;

        let relationship: ArticleTag = diesel::insert(&new_relationship)
        .into(articletags::table)
        .get_result(&connection)
        .expect("Error saving new article-tag relationship");
    } 
}

#[cfg(feature = "diesel")]
pub fn create_article<'a>(mut article: AdvancedArticle) -> Option<ArticleResult> {
    use schema::articles;

    //let new_article = new_article.article;
    let connection = establish_connection();

    let cloned_article = article.clone();
    let new_article = NewArticle {
        title: &cloned_article.title,
        slug: &cloned_article.slug,
        description: &cloned_article.description,
        body: &cloned_article.body,
        createdat: cloned_article.createdAt,
        updatedat: cloned_article.updatedAt,
        author: article.author
    };

    let article_result: Article = diesel::insert(&new_article)
        .into(articles::table)
        .get_result(&connection)
        .expect("Error saving new post");    

    article.id = article_result.id;
    
    let result = article.clone();
    create_article_tag(article);
    
    Some(ArticleResult { article: result,} )
}

pub fn create_article_handler(req: Request, res: Response, _: Captures) {
    let (body, logged_in_user_id) = prepare_parameters(req);

    let container: IncomingArticleResult = serde_json::from_str(&body).unwrap();
    let incoming_article = container.article;
    let title: String = incoming_article.title;
    let description: String = incoming_article.description;
    let article_body: String = incoming_article.body;
    let tag_list: Vec<String> = incoming_article.tagList.unwrap_or(Vec::new());
    let slug: String = slugify(&title);
    //let tags: &str = &tag_list.join(",");

    #[cfg(feature = "diesel")]
    {
        use chrono::prelude::*;
        let utc: DateTime<Utc> = Utc::now();

        let article = AdvancedArticle {
            id : -1,
            title: title,
            slug: slug,
            description: description,
            body: article_body,
            createdAt: utc.naive_utc(),
            updatedAt: None,
            author: logged_in_user_id,
            tagList: tag_list,
            favorited: false,
            favoritesCount: 0,
        };
        process(res, create_article, article );
    }

    #[cfg(feature = "tiberius")]
    process(
        res,
        r#"insert into Tags (Tag) SELECT EmployeeID = Item FROM dbo.SplitNVarchars(@P6, ',')  Except select Tag from Tags;                            
        INSERT INTO Articles (Title, [Description], Body, Created, Author, Slug) Values (@P1, @P2, @P3, getdate(), @P4, @P5);
        DECLARE @id int = SCOPE_IDENTITY(); DECLARE @logged int = @P4;
        insert into [ArticleTags] (ArticleId, TagId) SELECT @id, Id From Tags WHERE Tag IN (SELECT EmployeeID = Item FROM dbo.SplitNVarchars(@P6, ','));
        "#, 
        ARTICLE_SELECT,
        get_article_from_row,
        &[&title, &description, &body, &logged_in_user_id, &slug,&tags,]
    );
}

fn process_and_return_article(
    name: &str,
    req: Request,
    res: Response,
    c: Captures,
    sql_command: &'static str,
) {
    let (_, logged_id) = prepare_parameters(req);

    let caps = c.unwrap();
    let slug = &caps[0].replace("/api/articles/", "").replace(
        "/favorite",
        "",
    );
    println!("{} slug: '{}'", name, slug);
    println!("logged_id: {}", logged_id);

    #[cfg(feature = "tiberius")]
    process(
        res,
        sql_command,
        ARTICLE_SELECT,
        get_article_from_row,
        &[&(slug.as_str()), &(logged_id)],
    );
}

#[cfg(feature = "diesel")]
fn favorite_article<'a>(new_relationship: NewArticleUser) {
    
    //use diesel::associations::HasTable;
    
    let connection = establish_connection();

    use schema::favoritedarticles;
    //use diesel::associations::HasTable;

    let relationship: ArticleUser = diesel::insert(&new_relationship)
    .into(favoritedarticles::table)
    .get_result(&connection)
    .expect("Error saving new favorited article relationship");    
}

#[cfg(feature = "diesel")]
fn get_favorites_count(article_id: i32) -> i64 {
    use schema::favoritedarticles::dsl::*;
    use diesel::expression::count;

    let connection = establish_connection();

    let article_count: i64 = favoritedarticles
        //.select(count(articleid))
        .filter(articleid.eq(article_id))
        .count()
        .get_result(&connection)
        .unwrap();
    article_count
}

#[cfg(feature = "diesel")]
fn is_favorited(article_id: i32) -> bool {
    let count = get_favorites_count(article_id);
    count > 0
}

pub fn favorite_article_handler(req: Request, res: Response, c: Captures) {
    #[cfg(feature = "diesel")]
    {
        let (_, logged_in_user_id) = prepare_parameters(req);
        let caps = c.unwrap();
        let url_slug = &caps[0].replace("/api/articles/", "").replace(
            "/favorite","",
        );

        let article = get_article(url_slug.to_owned());
        let new_relationship = NewArticleUser {
            userid : logged_in_user_id,
            articleid : article.unwrap().article.id,
    }   ;
        favorite_article(new_relationship);
        process(res, get_article, url_slug.to_owned() );
    };

    #[cfg(feature = "tiberius")]
    process_and_return_article(
        "favorite_article_handler",
        req,
        res,
        c,
        "declare @id int; select TOP(1) @id = id from Articles where Slug = @P1 ORDER BY 1; DECLARE @logged int = @P2;
                INSERT INTO [dbo].[FavoritedArticles]
	            ([ArticleId],
	            [UserId])
	            VALUES (@id,@P2)",
    );
}

pub fn unfavorite_article_handler(req: Request, res: Response, c: Captures) {
    process_and_return_article(
        "unfavorite_article_handler",
        req,
        res,
        c,
        "declare @id int; DECLARE @logged int = @P2;
                select TOP(1) @id = id from Articles where Slug = @P1 ORDER BY 1;
                DELETE TOP(1) FROM FavoritedArticles WHERE ArticleId = @id AND UserId = @P2;
                ",
    );
}

fn articles_result(_: ArticlesResult) {}

pub fn feed_handler(req: Request, res: Response, c: Captures) {
    let (_, logged_id) = prepare_parameters(req);

    let caps = c.unwrap();
    let url_params = &caps[0].replace("/api/articles/feed?", "");

    println!("feed_handler url_params:'{}'", url_params);

    let parsed_params: Vec<&str> = url_params.split('&').collect();

    let mut limit: i32 = 20;
    let mut offset: i32 = 0;

    for param in &parsed_params {
        let name_value: Vec<&str> = param.split('=').collect();

        if name_value[0] == "offset" {
            offset = name_value[1].parse::<i32>().unwrap();
        } else if name_value[0] == "limit" {
            limit = name_value[1].parse::<i32>().unwrap();
        };
    }

    #[cfg(feature = "tiberius")]
    process_container(
        res,
        r#"declare @logged int = @p1;
        "#,
        r#"SELECT Slug, Title, [Description], Body, Created, Updated, Users.UserName, Users.Bio, Users.[Image], 
                (SELECT COUNT(*) FROM Followings WHERE FollowerId=@logged AND Author=FollowingId) as [Following],
                (SELECT COUNT(*) FROM FavoritedArticles WHERE ArticleId = Articles.Id ) as FavoritesCount,
                (SELECT COUNT(*) FROM FavoritedArticles WHERE UserId = @logged ) as PersonalFavoritesCount,
				(SELECT STRING_AGG(Tag, ',') FROM [Tags] inner join ArticleTags on ArticleTags.TagId = Tags.Id where ArticleId=Articles.Id)  as Tags
                FROM Articles INNER JOIN Users on Author=Users.Id  
				WHERE Author IN ( SELECT FollowingId FROM Followings WHERE FollowerId = @logged ) 
order by Articles.Id DESC OFFSET @p2 ROWS FETCH NEXT @p3 ROWS Only"#,
        get_simple_article_from_row,
        articles_result,
        &[&logged_id, &offset, &limit]
    );
}

#[derive(Debug)]
pub struct FilterParams<'a> {
    pub tag: &'a str,
    pub author: &'a str,
    pub favorited: &'a str,
    pub offset: i32,
    pub limit: i32,
}

fn get_articles_by_filter(filter: FilterParams) -> Vec<Article> {
    use schema::articles;

    let connection = establish_connection();
    articles::table.load(&connection).expect(
        "Error loading articles",
    )
}

pub fn list_article_handler(req: Request, res: Response, c: Captures) {
    let (_, logged_id) = prepare_parameters(req);

    let caps = c.unwrap();
    let url_params = &caps[0].replace("/api/articles?", "");

    println!("list_article_handler url_params:'{}'", url_params);

    let parsed_params: Vec<&str> = url_params.split('&').collect();

    let mut limit: i32 = 20;
    let mut offset: i32 = 0;
    let mut tag = "";
    let mut author = "";
    let mut favorited = "";

    for param in &parsed_params {
        let name_value: Vec<&str> = param.split('=').collect();

        if name_value[0] == "tag" {
            tag = name_value[1];
        } else if name_value[0] == "author" {
            author = name_value[1];
        } else if name_value[0] == "favorited" {
            favorited = name_value[1];
        } else if name_value[0] == "offset" {
            offset = name_value[1].parse::<i32>().unwrap();
        } else if name_value[0] == "limit" {
            limit = name_value[1].parse::<i32>().unwrap();
        };
    }

    #[cfg(feature = "diesel")]
    let filter: FilterParams = FilterParams {
        tag: tag,
        author: author,
        favorited: favorited,
        offset: offset,
        limit: limit,
    };

    #[cfg(feature = "diesel")]
    process_container(res, articles_result, get_articles_by_filter, filter);

    #[cfg(feature = "tiberius")]
    process_container(
        res,
        r#"declare @logged int = @p1;
declare @tag nvarchar(max) = @p4;
declare @username nvarchar(max) = @p5;
declare @favorited nvarchar(max) = @p6;        
        "#,
        r#"SELECT Slug, Title, [Description], Body, Created, Updated, Users.UserName, Users.Bio, Users.[Image], 
        (SELECT COUNT(*) FROM Followings WHERE FollowerId=@logged AND Author=FollowingId) as [Following],
        (SELECT COUNT(*) FROM FavoritedArticles WHERE ArticleId = Articles.Id ) as FavoritesCount,
        (SELECT COUNT(*) FROM FavoritedArticles WHERE UserId = @logged ) as PersonalFavoritesCount,
		(SELECT STRING_AGG(Tag, ',') FROM [Tags] inner join ArticleTags on ArticleTags.TagId = Tags.Id where ArticleId=Articles.Id)  as Tags
        FROM Articles INNER JOIN Users on Author=Users.Id  
		
		WHERE Articles.Id in ( SELECT ArticleId from ArticleTags WHERE TagId IN ( Select Id from Tags where Tag = @tag OR LEN(@tag) = 0 )  ) 
		/*inner join ArticleTags on ArticleTags.ArticleId = Articles.id 
		inner join Tags on Tags.Id = ArticleTags.TagId and Tag = @tag OR LEN(@tag) = 0*/
		
		AND Articles.Author in ( SELECT Id from Users where UserName = @username OR LEN(@username) = 0 ) 

		AND Articles.Id in ( SELECT ArticleId from FavoritedArticles WHERE UserId IN ( SELECT Id from Users where UserName = @favorited OR LEN(@favorited) = 0 )  ) 

order by Articles.Id DESC OFFSET @p2 ROWS FETCH NEXT @p3 ROWS Only"#,
        get_simple_article_from_row,
        articles_result,
        &[&logged_id, &offset, &limit, &tag, &author, &favorited]
    );
}

// #[cfg(feature = "tiberius")]
// static ARTICLE_SELECT : &'static str = r#"
//   SELECT Slug, Title, [Description], Body, Created, Updated, Users.UserName, Users.Bio, Users.[Image], 
//                 (SELECT COUNT(*) FROM Followings WHERE FollowerId=@logged AND Author=FollowingId) as [Following],
//                 (SELECT COUNT(*) FROM FavoritedArticles WHERE ArticleId = @id ) as FavoritesCount,
//                 (SELECT COUNT(*) FROM FavoritedArticles WHERE UserId = @logged ) as PersonalFavoritesCount,
// 				(SELECT STRING_AGG(Tag, ',') FROM [Tags] inner join ArticleTags on ArticleTags.TagId = Tags.Id where ArticleId=@id)  as Tags
//                 FROM Articles INNER JOIN Users on Author=Users.Id  WHERE Articles.Id = @id
// "#;


fn get_tags_for_article(article: &Article, conn: PgConnection) -> Vec<String> {
    use diesel::expression::dsl::any;
    use schema::articletags;
    use schema::tags;

    let tag_ids = ArticleTag::belonging_to(article).select(articletags::tagid);

    let tag_objs =
        tags::table
            .filter((tags::id).eq(any(&tag_ids)))
            .load::<Tag>(&conn)
            .expect("could not load tags");
    tag_objs.into_iter().map(|t| t.tag).collect()
}

fn get_article(url_slug: String) -> Option<ArticleResult> {
    use schema::articles::dsl::*;
    let connection = establish_connection();

    let article: Article = articles
        .filter(slug.eq(url_slug))
        .first(&connection)
        .unwrap();

    let tag_names = get_tags_for_article(&article, connection);
    let favorites_count = get_favorites_count(article.id);

    let result = AdvancedArticle {
        id : article.id,
        slug : article.slug,
        title : article.title,
        description : article.description,
        body : article.body,
        createdAt : article.createdAt,
        updatedAt : article.updatedAt,
        tagList : tag_names,
        author : article.author,
        favoritesCount: favorites_count,
        favorited: favorites_count > 0,
    };

    Some(ArticleResult { article: result,})
}

pub fn get_article_handler(req: Request, res: Response, c: Captures) {
    let (_, logged_in_user_id) = prepare_parameters(req);
    let caps = c.unwrap();
    let url_slug = &caps[0].replace("/api/articles/", "");

    #[cfg(feature = "diesel")] process(res, get_article, (url_slug.to_owned()));

    #[cfg(feature = "tiberius")]
    process_and_return_article(
        "get_article_handler",
        req,
        res,
        c,
        "declare @id int; select TOP(1) @id = id from Articles where Slug = @P1 ORDER BY 1; 
        DECLARE @logged int = @P2;",
    );
}

#[cfg(feature = "diesel")]
pub fn update_article<'a>(new_article: UpdatedArticle) -> Option<ArticleResult> {
    use schema::articles;

    let conn = establish_connection();

    let result = new_article.save_changes::<Article>(&conn).unwrap();

    get_article(result.slug)
}

pub fn update_article_handler(req: Request, res: Response, c: Captures) {
    let (request_body, logged_id) = prepare_parameters(req);

    let caps = c.unwrap();
    let url_slug = &caps[0].replace("/api/articles/", "");
    println!("slug {}", &url_slug);

    #[cfg(feature = "diesel")] {
        use models::UpdatedArticle;

        let incoming_article: UpdateArticle = serde_json::from_str(&request_body).unwrap();
        let new_title: &str = incoming_article
            .article
            .title
            .as_ref()
            .map(|x| &**x)
            .unwrap_or("");
        let new_body: &str = incoming_article
            .article
            .body
            .as_ref()
            .map(|x| &**x)
            .unwrap_or("");
        let new_description: &str = incoming_article
            .article
            .description
            .as_ref()
            .map(|x| &**x)
            .unwrap_or("");
        let new_slug: &str = &slugify(new_title);

        let article_result : ArticleResult = get_article(url_slug.to_owned()).unwrap();
        let original = article_result.article;
        let old_id = original.id;
        let old_author = original.author;
        let old_created = original.createdAt;
        let old_updated = original.updatedAt;

        let new_article = UpdatedArticle {
            id : old_id,
            slug : new_slug,
            title : new_title,
            description : new_description,
            body : new_body,
            author : old_author,
            createdat : old_created,
            updatedat : old_updated,
        };

        process(res, update_article, new_article )
    }

    #[cfg(feature = "tiberius")]
    process(
        res,
        r#"
        declare @id int; select TOP(1) @id = id from Articles where Slug = @P1; 
        DECLARE @logged int = @P5;
        UPDATE TOP(1) [dbo].[Articles] SET 
        [Title]=CASE WHEN(LEN(@P2)=0) THEN Title ELSE @P2 END,
        [Description]=CASE WHEN(LEN(@P3)=0) THEN Description ELSE @P3 END,
        [Body]=CASE WHEN(LEN(@P4)=0) THEN Description ELSE @P4 END,
        [Slug]=CASE WHEN(LEN(@P2)=0) THEN [Slug] ELSE @P6 END
        WHERE [Id] = @id AND Author = @logged; 
        "#,
        ARTICLE_SELECT,
        get_article_from_row,
        &[
            &(slug.as_str()),
            &title,
            &description,
            &body,
            &logged_id,
            &new_slug,
        ],
    );
}

#[cfg(feature = "diesel")]
fn delete_article (url_slug: String) -> Option<bool> {
    use schema::articles::dsl::*;
    let connection = establish_connection();

    // let article: Article = articles
    //     .filter(slug.eq(url_slug))
    //     .first(&connection)
    //     .unwrap();
    // article.del

    diesel::delete(articles.filter(slug.eq(url_slug))).execute(&connection);
    None
}

pub fn delete_article_handler(req: Request, res: Response, c: Captures) {
    let (_, logged_id) = prepare_parameters(req);

    let caps = c.unwrap();
    let slug = &caps[0].replace("/api/articles/", "");
    println!("slug: {}", slug);

    #[cfg(feature = "diesel")] 
    {
        process(res, delete_article, slug.to_owned());
    };

    #[cfg(feature = "tiberius")]
    process(
        res,
        "declare @id int; select TOP(1) @id = id from Articles where Slug = @P1 AND Author = @P2 ORDER BY 1; 
        DELETE FROM Comments WHERE ArticleId = @id;
        DELETE FROM FavoritedArticles WHERE ArticleId = @id;
        DELETE FROM ArticleTags WHERE ArticleId = @id;
        DELETE FROM Articles WHERE id = @id AND Author = @P2;",
        "SELECT 1",
        handle_row_none,
        &[&(slug.as_str()), &(logged_id)],
    );
}

#[cfg(test)]
use rand::Rng;

#[cfg(test)]
pub fn login_create_article(
    follow: bool,
) -> (std::string::String, std::string::String, std::string::String) {
    let client = Client::new();

    let (user_name, _, jwt) = if follow {
        user::follow_jacob()
    } else {
        let (user_name, email) = register_jacob();
        let jwt = login_jacob(email.to_owned(), user::JACOB_PASSWORD.to_string());
        (user_name, email, jwt)
    };

    let since = since_the_epoch();
    let num = rand::thread_rng().gen_range(0, 1000);
    let title = format!("How to train your dragon {}-{}", since, num);
    let slug: &str = &slugify(title.to_owned());

    let body = format!( r#"{{"article": {{"title": "{}","description": "Ever wonder how?","body": "You have to believe","tagList": ["reactjs", "angularjs", "dragons"]
                }}}}"#, title);

    let mut res = client
        .post("http://localhost:6767/api/articles")
        .header(Authorization(Bearer { token: jwt.to_owned() }))
        .body(&body)
        .send()
        .unwrap();

    let mut buffer = String::new();
    res.read_to_string(&mut buffer).unwrap();
    
    let create_result: ArticleResult = serde_json::from_str(&buffer).unwrap();
    let article = create_result.article;
    assert_eq!(article.slug, slug);
    assert_eq!(article.title, title);
    // assert_eq!(article.favorited, false);
    // assert_eq!(article.author.username, user_name);
    assert_eq!(article.tagList.len(), 3);

    assert_eq!(res.status, hyper::Ok);

    (jwt, slug.to_string(), user_name)
}

#[cfg(test)]
//#[test]
fn create_article_test() {
    login_create_article(false);
}

#[cfg(test)]
#[test]
fn favorite_article_test() {
    let client = Client::new();

    let (jwt, slug, user_name) = login_create_article(false);
    let url = format!("http://localhost:6767/api/articles/{}/favorite", slug);

    let mut res = client
        .post(&url)
        .header(Authorization(Bearer { token: jwt }))
        .send()
        .unwrap();
    let mut buffer = String::new();
    res.read_to_string(&mut buffer).unwrap();

    let create_result: ArticleResult = serde_json::from_str(&buffer).unwrap();
    let article = create_result.article;
    assert_eq!(article.slug, slug);
    assert_eq!(article.favorited, true);
    assert_eq!(article.favoritesCount, 1);
    //assert_eq!(article.author.username, user_name);

    assert_eq!(res.status, hyper::Ok);
}

#[cfg(test)]
//#[test]
fn unfavorite_article_test() {
    let client = Client::new();

    let (jwt, slug, user_name) = login_create_article(false);
    let url = format!("http://localhost:6767/api/articles/{}/favorite", slug);

    let mut res = client
        .delete(&url)
        .header(Authorization(Bearer { token: jwt }))
        .body("")
        .send()
        .unwrap();
    let mut buffer = String::new();
    res.read_to_string(&mut buffer).unwrap();

    let create_result: CreateArticleResult = serde_json::from_str(&buffer).unwrap();
    let article = create_result.article;
    assert_eq!(article.slug, slug);
    assert_eq!(article.favorited, false);
    assert_eq!(article.favoritesCount, 0);
    assert_eq!(article.author.username, user_name);

    assert_eq!(res.status, hyper::Ok);
}

#[cfg(test)]
#[test]
fn get_article_test() {
    let client = Client::new();

    let (_, _, user_name) = login_create_article(false);
    let slug = "dragons";
    let url = format!("http://localhost:6767/api/articles/{}", &slug);

    let mut res = client.get(&url).send().unwrap();
    let mut buffer = String::new();
    res.read_to_string(&mut buffer).unwrap();


    let create_result: ArticleResult = serde_json::from_str(&buffer).unwrap();
    let article = create_result.article;
    assert_eq!(article.slug, slug);
    // assert_eq!(article.favorited, false);
    // assert_eq!(article.favoritesCount, 0);
    // assert_eq!(article.author.username, user_name);

    assert_eq!(res.status, hyper::Ok);
}

#[cfg(test)]
//#[test]
fn list_article_test() {
    let client = Client::new();

    let (_, _, _) = login_create_article(false);

    let mut res = client
        .get("http://localhost:6767/api/articles?tag=dragons")
        .body("")
        .send()
        .unwrap();
    assert_eq!(res.status, hyper::Ok);

    let mut buffer = String::new();
    res.read_to_string(&mut buffer).unwrap();

    let articles: ArticlesResult = serde_json::from_str(&buffer).unwrap();
    assert_eq!(articles.articles.len() > 0, true);
}

#[cfg(test)]
//#[test]
fn unfollowed_feed_article_test() {
    let client = Client::new();

    let (jwt, _, _) = login_create_article(false);

    let mut res = client
        .get("http://localhost:6767/api/articles/feed")
        .header(Authorization(Bearer { token: jwt }))
        .send()
        .unwrap();
    assert_eq!(res.status, hyper::Ok);

    let mut buffer = String::new();
    res.read_to_string(&mut buffer).unwrap();

    let articles: ArticlesResult = serde_json::from_str(&buffer).unwrap();
    assert_eq!(articles.articles.len() == 0, true);
}

#[cfg(test)]
//#[test]
fn followed_feed_article_test() {
    let client = Client::new();

    let (jwt, _, _) = login_create_article(true);

    let mut res = client
        .get("http://localhost:6767/api/articles/feed")
        .header(Authorization(Bearer { token: jwt }))
        .send()
        .unwrap();
    assert_eq!(res.status, hyper::Ok);

    let mut buffer = String::new();
    res.read_to_string(&mut buffer).unwrap();

    let articles: ArticlesResult = serde_json::from_str(&buffer).unwrap();
    assert_eq!(articles.articles.len() == 1, true);
}

#[cfg(test)]
#[test]
fn update_article_test() {
    let client = Client::new();

    let (jwt, title, user_name) = login_create_article(false);
    let url = format!("http://localhost:6767/api/articles/{}", title);
    let title2 = title + " NOT";
    let body = format!(
        r#"{{"article": {{"title": "{}","description": "CHANGED1","body": "CHANGED2"}}}}"#,
        title2
    );

    let mut res = client
        .put(&url)
        .header(Authorization(Bearer { token: jwt }))
        .body(&body)
        .send()
        .unwrap();
    let mut buffer = String::new();
    res.read_to_string(&mut buffer).unwrap();
    println!("buffer: '{}'", buffer);

    let create_result: ArticleResult = serde_json::from_str(&buffer).unwrap();
    let article = create_result.article;
    assert_eq!(article.slug, slugify(title2.to_owned()));
    assert_eq!(article.title, title2);
    assert_eq!(article.description, "CHANGED1");
    assert_eq!(article.body, "CHANGED2");
    //assert_eq!(article.favorited, false);
    //assert_eq!(article.favoritesCount, 0);
    //assert_eq!(article.author.username, user_name);
}

#[cfg(test)]
#[test]
fn delete_article_test() {
    let client = Client::new();

    let (jwt, title, _) = login_create_article(false);
    let url = format!("http://localhost:6767/api/articles/{}", title);

    let res = client
        .delete(&url)
        .header(Authorization(Bearer { token: jwt }))
        .body("")
        .send()
        .unwrap();
    assert_eq!(res.status, hyper::Ok);
}
