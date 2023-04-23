use actix_web::cookie::{Cookie, time::Duration, CookieJar};
use chrono::Utc;
use jsonwebtoken::{Header, encode, EncodingKey, Validation, Algorithm, decode, DecodingKey};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

use crate::{models::user::{User, UserResult, UserOperation}, db::DBPooledConnection};

pub const ACCESS_TOKEN_DURATION: i64 = 3600 * 12;           // 12 hours 
pub const REFRESH_TOKEN_DURATION: i64 = 3600 * 24 * 90;     // 90 days

#[derive(Debug, Deserialize, Serialize)]
pub struct AccessTokenClaims {
    pub sub: String,
    pub email: String,
    pub exp: usize,
}

pub fn create_access_token(user: &User) -> jsonwebtoken::errors::Result<String> {
    let secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set");

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(ACCESS_TOKEN_DURATION))
        .expect("valid timestamp")
        .timestamp();

    let claims = AccessTokenClaims {
        sub: user.id.clone(),
        email: user.email.clone(),
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);

    encode(&header, &claims, &EncodingKey::from_secret(secret.as_bytes()))
}

pub fn decode_access_token(jwt: &str) -> Option<AccessTokenClaims> {
    let secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set");

    let token = decode::<AccessTokenClaims>(&jwt, &DecodingKey::from_secret(secret.as_bytes()), &Validation::new(Algorithm::HS512));

    match token {
        Ok(token) => Some(token.claims),
        Err(_) => None
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RefreshTokenClaims {
    pub sub: String,
    pub user_id: String,
    pub exp: usize,
}

pub fn create_refresh_token(user: &User) -> jsonwebtoken::errors::Result<String> {
    let secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set");

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(REFRESH_TOKEN_DURATION))
        .expect("valid timestamp")
        .timestamp();

    let claims = RefreshTokenClaims {
        sub: nanoid!(),
        user_id: user.id.clone(),
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);

    encode(&header, &claims, &EncodingKey::from_secret(secret.as_bytes()))
}

pub fn decode_refresh_token(jwt: &str) -> Option<RefreshTokenClaims> {
    let secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set");

    let token = decode::<RefreshTokenClaims>(&jwt, &DecodingKey::from_secret(secret.as_bytes()), &Validation::new(Algorithm::HS512));

    match token {
        Ok(token) => Some(token.claims),
        Err(_) => None
    }
}

pub fn get_authed_user(conn: &mut DBPooledConnection, jar: &mut CookieJar) -> UserResult {
    let access_token = jar.get("access_token");

    if access_token.is_none() {
        return refresh_tokens(conn, jar);
    }

    let access_token_decoded = decode_access_token(access_token.unwrap().value());

    if access_token_decoded.is_none() {
        return UserResult::unauthorized("access token is invalid");
    }

    let access_token_data = access_token_decoded.unwrap();
    let now = Utc::now().timestamp() as usize;

    if access_token_data.exp < now {
        return refresh_tokens(conn, jar);
    }

    UserOperation::find(conn, &access_token_data.sub)
}

pub fn set_authed_user(user: &User, jar: &mut CookieJar) {
    let access_token = create_access_token(user).unwrap();
    let refresh_token = create_refresh_token(user).unwrap();

    jar.add(
        Cookie::build("access_token", access_token.clone())
            .domain("localhost")
            .path("/")
            .max_age(Duration::seconds(ACCESS_TOKEN_DURATION))
            .secure(true)
            .http_only(true)
            .finish()
        );

    jar.add(
        Cookie::build("refresh_token", refresh_token.clone())
            .domain("localhost")
            .path("/")
            .max_age(Duration::seconds(REFRESH_TOKEN_DURATION))
            .secure(true)
            .http_only(true)
            .finish()
        );
}

fn refresh_tokens(conn: &mut DBPooledConnection, jar: &mut CookieJar) -> UserResult {
    let refresh_token = jar.get("refresh_token");
    
    if refresh_token.is_none() {
        return UserResult::unauthorized("no token was provided");
    }

    let refresh_token_decoded = decode_refresh_token(refresh_token.unwrap().value());

    if refresh_token_decoded.is_none() {
        return UserResult::unauthorized("invalid token");
    }

    let refresh_token_data = refresh_token_decoded.unwrap();
    let now = Utc::now().timestamp() as usize;

    if refresh_token_data.exp < now {
        return UserResult::unauthorized("token has expired");
    }

    let user_result = UserOperation::find(conn, &refresh_token_data.user_id);

    if let UserResult::User(user) = &user_result {
        set_authed_user(user, jar);
    }

    user_result
}

// fn refresh_tokens(conn: &mut DBPooledConnection, jar: &mut CookieJar) -> Option<User> {
//     use crate::schema::users::dsl::{users, id as user_id};
// 
//     let refresh_token = jar.get("refresh_token");
// 
//     if refresh_token.is_none() {
//         return None;
//     }
// 
//     let refresh_token = decode_refresh_token(refresh_token.unwrap().value());
// 
//     if refresh_token.is_none() {
//         return None;
//     }
// 
//     let now = Utc::now().timestamp() as usize;
// 
//     let refresh_token = refresh_token.unwrap();
// 
//     if refresh_token.exp < now {
//         return None;
//     }
// 
//     let user = users
//         .filter(user_id.eq(refresh_token.user_id))
//         .get_result::<User>(conn);
// 
//     match user {
//         Ok(user) => {
//             update_token_cookies(&user, jar);
//             return Some(user);
//         },
//         Err(_) => None
//     }
// }
