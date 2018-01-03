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

use hyper::status::StatusCode;

#[cfg(feature = "tiberius")]
use futures::Future;
#[cfg(feature = "tiberius")]
use tokio_core::reactor::Core;

#[cfg(feature = "tiberius")]
use tiberius::SqlConnection;
#[cfg(feature = "tiberius")]
use tiberius::stmt::ResultStreamExt;

use std::io::prelude::*;

use hyper::server::{Request, Response};
use reroute::Captures;

use crypto::sha2::Sha256;

use jwt::{Header, Registered, Token};

use super::*;

pub fn new_token(user_id: &str, _: &str) -> Option<String> {
    let header: jwt::Header = Default::default();
    let claims = jwt::Registered {
        iss: Some("mikkyang.com".into()),
        sub: Some(user_id.into()),
        ..Default::default()
    };
    let token = Token::new(header, claims);

    token.signed(b"secret_key", Sha256::new()).ok()
}

pub fn login(token: &str) -> Option<i32> {
    let token = Token::<Header, Registered>::parse(token).unwrap();

    if token.verify(b"secret_key", Sha256::new()) {
        match token.claims.sub {
            Some(token) => {
                match token.parse::<i32>() {
                    Ok(result) => Some(result),
                    Err(_) => None,
                }
            }
            _ => None,
        }


    } else {
        None
    }
}

#[cfg(feature = "tiberius")]
fn get_user_from_row(row: tiberius::query::QueryRow) -> (i32, String, Option<UserResult>) {
    let email: &str = row.get(0);
    let token: &str = row.get(1);
    let user_name: &str = row.get(2);
    let bio: Option<&str> = row.get(3);
    let image: Option<&str> = row.get(4);
    let user_id: i32 = row.get(5);
    let result = Some(UserResult {
        user: User {
            email: email.to_string(),
            token: token.to_string(),
            bio: bio.map(|s| s.to_string()),
            image: image.map(|s| s.to_string()),
            username: user_name.to_string(),
        },
    });
    (user_id, token.to_string(), result)
}

#[cfg(feature = "tiberius")]
fn get_user_from_row_simple(row: tiberius::query::QueryRow) -> Option<UserResult> {
    let (_, _, result) = get_user_from_row(row);
    result
}

#[cfg(feature = "tiberius")]
fn get_profile_from_row(row: tiberius::query::QueryRow) -> Option<ProfileResult> {
    let _: &str = row.get(0);
    let _: &str = row.get(1);
    let user_name: &str = row.get(2);
    let bio: Option<&str> = row.get(3);
    let image: Option<&str> = row.get(4);
    let f: i32 = row.get(5);
    let following: bool = f == 1;
    let result = Some(ProfileResult {
        profile: Profile {
            following: following,
            bio: bio.map(|s| s.to_string()),
            image: image.map(|s| s.to_string()),
            username: user_name.to_string(),
        },
    });
    result
}

#[cfg(feature = "tiberius")]
static USER_SELECT: &'static str =
    r#"SELECT [Email],[Token],[UserName],[Bio],[Image], Id FROM [dbo].[Users] WHERE [Id] = @id"#;
#[cfg(feature = "tiberius")]
static PROFILE_SELECT : &'static str = r#"SELECT [Email],[Token],[UserName],[Bio],[Image] ,
( SELECT COUNT(*) FROM dbo.Followings F WHERE F.[FollowingId] = Id AND F.FollowerId = @logged ) as Following
FROM [dbo].[Users]  WHERE [UserName] = @username"#;

#[cfg(feature = "diesel")]
pub fn create_user<'a>(new_user: NewUser) -> Option<UserResult> {
    use schema::users;

    let connection = establish_connection();
    let user: User = diesel::insert(&new_user)
        .into(users::table)
        .get_result(&connection)
        .expect("Error saving new user");
    Some(UserResult { user: user })
}

pub fn registration_handler(req: Request, res: Response, _: Captures) {
    let (body, _) = prepare_parameters(req);

    let registration: Registration = serde_json::from_str(&body).unwrap();
    let user = registration.user;
    let email: &str = &user.email;
    let token: &str = &crypto::pbkdf2::pbkdf2_simple(&user.password, 10000).unwrap();
    let user_name: &str = &user.username;

    #[cfg(feature = "tiberius")]
    {
        process(
            res,
            r#"INSERT INTO [dbo].[Users]
                    ([Email]
                    ,[Token]
                    ,[UserName])
                VALUES
                    (@P1
                    ,@P2
                    ,@P3); DECLARE @id int = SCOPE_IDENTITY();"#,
            USER_SELECT,
            get_user_from_row_simple,
            &[&email, &token, &user_name],
        );
    }
    #[cfg(feature = "diesel")]
    {
        let new_user = NewUser {
            email: email,
            token: token,
            username: user_name,
        };
        process(res, create_user, new_user);
    }
}

#[cfg(feature = "diesel")] 
fn update_user(updated: UpdatedUser) -> Option<UserResult> {
    let conn = establish_connection();

    let result = updated.save_changes::<User>(&conn).unwrap();

    Some(UserResult { user:result })
}

pub fn update_user_handler(req: Request, res: Response, _: Captures) {
    let (body, logged_in_user_id) = prepare_parameters(req);

    let updated_user: UpdateUser = serde_json::from_str(&body).unwrap();
    let original_user : User = get_user_by_id(logged_in_user_id).unwrap().user;
    let original_bio = original_user.bio.unwrap_or_default();
    let original_image = original_user.image.unwrap_or_default();

    let user_name: &str = &updated_user.user.username.as_ref().map(|x| &**x).unwrap_or(
        &original_user.username,
    );
    let new_bio: &str = updated_user.user.bio.as_ref().map(|x| &**x).unwrap_or(&original_bio);
    let new_image: &str = updated_user.user.image.as_ref().map(|x| &**x).unwrap_or(&original_image);
    let new_email: &str = &updated_user.user.email.as_ref().map(|x| &**x).unwrap_or(&original_user.email);
    let new_password: &str = &updated_user.user.password.as_ref().map(|x| &**x).unwrap_or("");

    let new_token: &str = &crypto::pbkdf2::pbkdf2_simple(new_password, 10000).unwrap();
         

    #[cfg(feature = "diesel")] {
        let updated = UpdatedUser  {
            id : logged_in_user_id,
            email : new_email,
            bio : new_bio,
            image : new_image,
            token : new_token,
            username : user_name,
        };

        process(res, update_user, updated)
    }

    #[cfg(feature = "tiberius")]
    process(
        res,
        r#"  UPDATE [dbo].[Users] SET 
                                [UserName]=CASE WHEN(LEN(@P2)=0) THEN UserName ELSE @P2 END,
                                [Bio]=CASE WHEN(LEN(@P3)=0) THEN Bio ELSE @P3 END,
                                [Image]=CASE WHEN(LEN(@P4)=0) THEN Image ELSE @P4 END,
                                [Email]=CASE WHEN(LEN(@P5)=0) THEN Email ELSE @P5 END,
                                [Token]=CASE WHEN(LEN(@P7)=0) THEN Token ELSE @P6 END
                                WHERE [Id] = @P1; DECLARE @id int = @P1;
                            "#,
        USER_SELECT,
        get_user_from_row_simple,
        &[
            &logged_in_user_id,
            &user_name,
            &bio,
            &image,
            &email,
            &token,
            &password,
        ],
    );
}

#[cfg(feature = "diesel")]
fn get_user_by_name(user_name: &str) -> Option<User> {
    use schema::users::dsl::*;

    let connection = establish_connection();
    let result: User = 
        users
        .filter(username.eq(user_name))
        .first(&connection)
        .unwrap();
    Some(result)
}

#[cfg(feature = "diesel")]
pub fn get_user_by_id(user_id: i32) -> Option<UserResult> {
    use schema::users::dsl::*;

    let connection = establish_connection();
    let user: User = users
        .filter(id.eq(user_id))
        .first(&connection)
        .unwrap();
    Some(UserResult { user: user })
}

pub fn get_current_user_handler(req: Request, res: Response, _: Captures) {
    let (_, logged_in_user_id) = prepare_parameters(req);

    #[cfg(feature = "tiberius")]
    process(
        res,
        r#"DECLARE @id int = @P1;"#,
        USER_SELECT,
        get_user_from_row_simple,
        &[&logged_in_user_id],
    );
    #[cfg(feature = "diesel")]
    process(res, get_user_by_id, logged_in_user_id);
}

fn get_profile_result(user: User) -> Option<ProfileResult> {
    let followed = is_followed(user.id);
    let result = Profile {
        username : user.username,
        bio : user.bio,
        image : user.image,
        following : followed,
    };

     Some(ProfileResult { profile: result,})
}

pub fn get_profile_handler(req: Request, res: Response, c: Captures) {
    let (_, _) = prepare_parameters(req);

    let caps = c.unwrap();
    let profile = &caps[0].replace("/api/profiles/", "");
    println!("profile: {}", profile);

    #[cfg(feature = "diesel")] {
        let user = get_user_by_name(profile).unwrap();

        process(res, get_profile_result, user)
    }

    #[cfg(feature = "tiberius")]
    process(
        res,
        r#"DECLARE @username nvarchar(max) = @P1;DECLARE @logged int = @P2;"#,
        PROFILE_SELECT,
        get_profile_from_row,
        &[&(profile.as_str()), &logged_in_user_id],
    );
}

pub fn unfollow_handler(req: Request, res: Response, c: Captures) {
    let (_, logged_in_user_id) = prepare_parameters(req);

    let caps = c.unwrap();
    let profile = &caps[0].replace("/api/profiles/", "").replace("/follow", "");
    println!("profile: {}", profile);

    #[cfg(feature = "diesel")] {
        let following : User = get_user_by_name(profile).unwrap();

        unfollow_user(logged_in_user_id, following.id);

        let updated_user = get_user_by_name(profile).unwrap();

        process(res, get_profile_result, updated_user)
    }

    #[cfg(feature = "tiberius")]
    process(
        res,
        r#"DECLARE @username nvarchar(max) = @P1;DECLARE @logged int = @P2;DELETE TOP (1) from [dbo].[Followings] WHERE [FollowerId] = @P2;"#, PROFILE_SELECT,
        get_profile_from_row,
        &[&(profile.as_str()), &logged_in_user_id]
    );
}

#[cfg(feature = "diesel")]
fn is_followed(user_id: i32) -> bool {
    use schema::followings::dsl::*;

    let connection = establish_connection();

    let followers_count: i64 = followings
        .filter(followingid.eq(user_id))
        .count()
        .get_result(&connection)
        .unwrap();
    followers_count > 0    
}

#[cfg(feature = "diesel")]
fn follow_user<'a>(follow: NewFollowing) {  
    let connection = establish_connection();

    use schema::followings;

    let _relationship: Following = diesel::insert(&follow)
    .into(followings::table)
    .get_result(&connection)
    .expect("Error saving new favorited article relationship");    
}

#[cfg(feature = "diesel")]
fn unfollow_user<'a>(follower_id: i32, following_id: i32) {  
    let connection = establish_connection();

    use schema::followings::dsl::*;

    let relationship: Following = followings
        .filter(followerid.eq(follower_id).and(followingid.eq(following_id)))
        .first(&connection)
        .unwrap();

    diesel::delete(followings.filter(id.eq(relationship.id))).execute(&connection).expect("Failed to unfollow user");   
}

pub fn follow_handler(req: Request, res: Response, c: Captures) {

    let (_, logged_in_user_id) = prepare_parameters(req);

    let caps = c.unwrap();
    let profile = &caps[0].replace("/api/profiles/", "").replace("/follow", "");
    println!("profile: {}", profile);

    #[cfg(feature = "diesel")] {
        let followed_user : User = get_user_by_name(profile).unwrap();

        let follow = NewFollowing {
            followerid : logged_in_user_id,
            followingid : followed_user.id,
        };

        follow_user(follow);

        let updated = get_user_by_name(profile).unwrap();
        process(res, get_profile_result, updated)
    }

    #[cfg(feature = "tiberius")]
    process(
        res,
        r#"DECLARE @username nvarchar(max) = @P1;DECLARE @logged int = @P2;INSERT INTO [dbo].[Followings] ([FollowingId] ,[FollowerId])
     SELECT @P2,(SELECT TOP (1) [Id]  FROM [Users] where UserName = @P1) EXCEPT SELECT [FollowingId] ,[FollowerId] from Followings;"#, PROFILE_SELECT,
        get_profile_from_row,
        &[&(profile.as_str()), &logged_in_user_id]
    );
}

pub fn authentication_handler(mut req: Request, mut res: Response, _: Captures) {
    let mut body = String::new();
    let _ = req.read_to_string(&mut body);
    let login: Login = serde_json::from_str(&body).unwrap();
    let user_email: &str = &login.user.email;

    let mut result: Option<UserResult> = None;
    #[cfg(feature = "diesel")]
    {
        use schema::users::dsl::*;

        let connection = establish_connection();
        let user: User = users
            .filter(email.eq(user_email))
            .first(&connection)
            .unwrap();
        let stored_hash: &str = &user.token.to_owned();
        let user_id = user.id;
        let authenticated_user = crypto::pbkdf2::pbkdf2_check(&login.user.password, &stored_hash);
        result = Some(UserResult { user: user });

        match authenticated_user {
            Ok(valid) => {
                if valid {
                    let token2 = new_token(user_id.to_string().as_ref(), &login.user.password)
                        .unwrap();

                    res.headers_mut().set(Authorization(
                        Bearer { token: token2.to_owned() },
                    ));
                    res.headers_mut().set(AccessControlAllowOrigin::Any);
                    res.headers_mut().set(AccessControlAllowHeaders(vec![
                        UniCase("content-type".to_owned()),
                        UniCase("authorization".to_owned()),
                    ]));
                    res.headers_mut().set(ContentType(Mime(
                        TopLevel::Application,
                        SubLevel::Json,
                        vec![(Attr::Charset, Value::Utf8)],
                    )));

                    *res.status_mut() = StatusCode::Ok;
                }
            }
            _ => {
                result = None;
            }
        }
    }
    #[cfg(feature = "tiberius")]
    {
        let mut sql = Core::new().unwrap();
        let get_user_cmd = SqlConnection::connect(sql.handle(), CONNECTION_STRING.as_str() )
            .and_then(|conn| conn.query( "SELECT TOP 1 [Email],[Token],[UserName],[Bio],[Image], Id FROM [dbo].[Users] WHERE [Email] = @P1", &[&user_email] )
            .for_each_row(|row| {
                let (user_id,stored_hash,result2) = get_user_from_row(row);
                let authenticated_user = crypto::pbkdf2::pbkdf2_check( &login.user.password, &stored_hash);
                *res.status_mut() = StatusCode::Unauthorized;

                match authenticated_user {
                    Ok(valid) => {
                        if valid {                     
                            let token = new_token(user_id.to_string().as_ref(), &login.user.password).unwrap();

                            res.headers_mut().set(
                                Authorization(
                                    Bearer {
                                        token: token.to_owned()
                                    }
                                )
                            );
                            res.headers_mut().set(
                                AccessControlAllowOrigin::Any
                            );
                            res.headers_mut().set(
                                AccessControlAllowHeaders(vec![UniCase("content-type".to_owned()), UniCase("authorization".to_owned())])
                            );                            
                            res.headers_mut().set(
                                ContentType(Mime(TopLevel::Application, SubLevel::Json,
                                            vec![(Attr::Charset, Value::Utf8)]))
                            );  

                            *res.status_mut() = StatusCode::Ok;
                            result = result2;
                        }
                    }
                    _ => { result = None; }
                }            
                Ok(())
            })
        );
        sql.run(get_user_cmd).unwrap();
    }

    if result.is_some() {
        let result = result.unwrap();
        let result = serde_json::to_string(&result).unwrap();
        let result: &[u8] = result.as_bytes();
        res.send(&result).unwrap();
    }
}


#[cfg(test)]
use hyper::Client;
#[cfg(test)]
use user::rand::Rng;

#[cfg(test)]
pub static JACOB_PASSWORD: &'static str = r#"jakejake"#;

#[cfg(test)]
pub fn register_jacob() -> (std::string::String, std::string::String) {
    let client = Client::new();
    let since = since_the_epoch();

    let num = rand::thread_rng().gen_range(0, 1000);
    let user_name = format!("Jacob-{}-{}", since, num);
    let email = format!("jake-{}-{}@jake.jake", since, num);
    let body = format!(
        r#"{{"user":{{"username": "{}","email": "{}","password": "{}"}}}}"#,
        user_name,
        email,
        JACOB_PASSWORD
    );

    let mut res = client
        .post("http://localhost:6767/api/users")
        .body(&body)
        .send()
        .unwrap();

    let mut buffer = String::new();
    res.read_to_string(&mut buffer).unwrap();

    let registration: UserResult = serde_json::from_str(&buffer).unwrap();
    let registered_user = registration.user;
    assert_eq!(registered_user.email, email);
    assert_eq!(registered_user.username, user_name);

    assert_eq!(res.status, hyper::Ok);
    (user_name, email)
}

#[cfg(test)]
pub fn login_jacob(email: std::string::String, password: String) -> std::string::String {
    let client = Client::new();

    let body = format!(
        r#"{{"user":{{"email": "{}","password": "{}"}}}}"#,
        email,
        password
    );

    let mut res = client
        .post("http://localhost:6767/api/users/login")
        .body(&body)
        .send()
        .unwrap();
    let mut buffer = String::new();
    res.read_to_string(&mut buffer).unwrap();

    let login: UserResult = serde_json::from_str(&buffer).unwrap();
    let logged_user = login.user;
    assert_eq!(logged_user.email, email);

    assert_eq!(res.status, hyper::Ok);
    let token = res.headers.get::<Authorization<Bearer>>().unwrap();
    let jwt = &token.0.token;
    jwt.to_owned()
}

#[cfg(test)]
pub fn follow_jacob() -> (std::string::String, std::string::String, std::string::String) {
    let client = Client::new();
    let (user_name, email) = register_jacob();
    let jwt = login_jacob(email.to_owned(), JACOB_PASSWORD.to_string());
    let url = format!("http://localhost:6767/api/profiles/{}/follow", user_name);
    println!("url:{}", url);

    let mut res = client
        .post(&url)
        .header(Authorization(Bearer { token: jwt.to_owned() }))
        .send()
        .unwrap();

    let mut buffer = String::new();
    res.read_to_string(&mut buffer).unwrap();

    let profile_result: ProfileResult = serde_json::from_str(&buffer).unwrap();
    let profile = profile_result.profile;
    assert_eq!(profile.username, user_name);
    assert_eq!(profile.following, true);

    assert_eq!(res.status, hyper::Ok);

    (user_name, email, jwt)
}

#[cfg(test)]
#[test]
fn registration_test() {
    register_jacob();
}

#[cfg(test)]
//#[test]
fn login_test() {
    let (_, email) = register_jacob();
    login_jacob(email, JACOB_PASSWORD.to_string());
}

#[cfg(test)]
#[test]
fn get_current_user_test() {
    let client = Client::new();
    let (user_name, email) = register_jacob();
    let jwt = login_jacob(email.to_owned(), JACOB_PASSWORD.to_string());

    let url = format!("http://localhost:6767/api/user");

    let mut res = client
        .get(&url)
        .header(Authorization(Bearer { token: jwt }))
        .send()
        .unwrap();
    let mut buffer = String::new();
    res.read_to_string(&mut buffer).unwrap();

    let registration: UserResult = serde_json::from_str(&buffer).unwrap();
    let registered_user = registration.user;
    assert_eq!(registered_user.email, email);
    assert_eq!(registered_user.username, user_name);

    assert_eq!(res.status, hyper::Ok);
}

#[cfg(test)]
#[test]
fn update_user_test() {
    let client = Client::new();
    let (user_name, email) = register_jacob();
    let jwt = login_jacob(email.to_owned(), JACOB_PASSWORD.to_string());

    let url = format!("http://localhost:6767/api/user");
    let new_user_name = user_name.to_owned() + "_CH";
    let body = format!(
        r#"{{"user": {{ "username":"{}"}}}}"#,
        new_user_name.to_owned()
    );

    let mut res = client
        .put(&url)
        .header(Authorization(Bearer { token: jwt }))
        .body(&body)
        .send()
        .unwrap();
    let mut buffer = String::new();
    res.read_to_string(&mut buffer).unwrap();

    let registration: UserResult = serde_json::from_str(&buffer).unwrap();
    let registered_user = registration.user;
    assert_eq!(registered_user.email, email);
    assert_eq!(registered_user.username, new_user_name);

    assert_eq!(res.status, hyper::Ok);
}

#[cfg(test)]
#[test]
#[should_panic]
fn get_current_user_fail_test() {
    let client = Client::new();

    let url = format!("http://localhost:6767/api/user");

    let mut res = client.get(&url).send().unwrap();
    let mut buffer = String::new();
    res.read_to_string(&mut buffer).unwrap();

    let registration: UserResult = serde_json::from_str(&buffer).unwrap();
    let _ = registration.user;
    assert_eq!(res.status, hyper::Ok);
}


#[cfg(test)]
//#[test]
#[should_panic]
fn login_fail_test() {
    let (_, email) = register_jacob();
    login_jacob(email, JACOB_PASSWORD.to_string() + "!");
}

#[cfg(test)]
//#[test]
fn profile_unlogged_test() {
    let client = Client::new();
    let (user_name, _) = register_jacob();
    let url = format!("http://localhost:6767/api/profiles/{}", user_name);

    let mut res = client.get(&url).send().unwrap();
    let mut buffer = String::new();
    res.read_to_string(&mut buffer).unwrap();

    let profile_result: ProfileResult = serde_json::from_str(&buffer).unwrap();
    let profile = profile_result.profile;
    assert_eq!(profile.username, user_name);
    assert_eq!(profile.following, false);

    assert_eq!(res.status, hyper::Ok);
}

#[cfg(test)]
#[test]
fn follow_test() {
    follow_jacob();
}


#[cfg(test)]
//#[test]
fn profile_logged_test() {
    let client = Client::new();

    let (user_name, email) = register_jacob();
    let jwt = login_jacob(email, JACOB_PASSWORD.to_string());
    let url = format!("http://localhost:6767/api/profiles/{}", user_name);

    let mut res = client
        .get(&url)
        .header(Authorization(Bearer { token: jwt }))
        .send()
        .unwrap();
    let mut buffer = String::new();
    res.read_to_string(&mut buffer).unwrap();

    let profile_result: ProfileResult = serde_json::from_str(&buffer).unwrap();
    let profile = profile_result.profile;
    assert_eq!(profile.username, user_name);
    assert_eq!(profile.following, false);

    assert_eq!(res.status, hyper::Ok);
}

#[cfg(test)]
#[test]
fn unfollow_test() {
    let client = Client::new();

    let (user_name, _, jwt) = follow_jacob();
    let url = format!("http://localhost:6767/api/profiles/{}/follow", user_name);

    let mut res = client
        .delete(&url)
        .header(Authorization(Bearer { token: jwt }))
        .body("")
        .send()
        .unwrap();
    let mut buffer = String::new();
    res.read_to_string(&mut buffer).unwrap();

    let profile_result: ProfileResult = serde_json::from_str(&buffer).unwrap();
    let profile = profile_result.profile;
    assert_eq!(profile.username, user_name);
    assert_eq!(profile.following, false);

    assert_eq!(res.status, hyper::Ok);
}
