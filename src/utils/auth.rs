use diesel::prelude::*;
use actix_web::cookie::{CookieJar, Cookie, time::Duration};
use chrono::Utc;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header, errors::Result, decode, DecodingKey, Validation};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

use crate::{schemas::{user::User, root::Context}, db::DBPooledConnection};

pub const ACCESS_TOKEN_DURATION: i64 = 600;     // 10 minuits in seconds
pub const REFRESH_TOKEN_DURATION: i64 = 129600; // 90 days in seconds
                                                
#[derive(Debug, Deserialize, Serialize)]
pub struct AccessTokenClaims {
    pub sub: String,
    pub email: String,
    pub exp: usize,
}

pub fn create_access_token(user: &User) -> Result<String> {
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

pub fn create_refresh_token(user: &User) -> Result<String> {
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

pub fn get_authed_user(context: &Context) -> Option<User> {
    use crate::schema::users::dsl::{users, id as user_id};

    let jar = &mut context.cookie_jar.write().unwrap();
    let conn = &mut context.db_pool.get().unwrap();

    let access_token = jar.get("access_token");

    if access_token.is_none() {
        return refresh_tokens(conn, jar);
    }

    let access_token = decode_access_token(access_token.unwrap().value());

    if access_token.is_none() {
        return None;
    }

    let access_token = access_token.unwrap();
    let now = Utc::now().timestamp() as usize;

    if access_token.exp < now {
        return refresh_tokens(conn, jar);
    }

    let user = users
        .filter(user_id.eq(access_token.sub))
        .get_result::<User>(conn);

    match user {
        Ok(user) => Some(user),
        Err(_) => None
    }
}

pub fn update_token_cookies(user: &User, jar: &mut CookieJar) {
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

fn refresh_tokens(conn: &mut DBPooledConnection, jar: &mut CookieJar) -> Option<User> {
    use crate::schema::users::dsl::{users, id as user_id};

    let refresh_token = jar.get("refresh_token");

    if refresh_token.is_none() {
        return None;
    }

    let refresh_token = decode_refresh_token(refresh_token.unwrap().value());

    if refresh_token.is_none() {
        return None;
    }

    let now = Utc::now().timestamp() as usize;

    let refresh_token = refresh_token.unwrap();

    if refresh_token.exp < now {
        return None;
    }

    let user = users
        .filter(user_id.eq(refresh_token.user_id))
        .get_result::<User>(conn);

    match user {
        Ok(user) => {
            update_token_cookies(&user, jar);
            return Some(user);
        },
        Err(_) => None
    }
}
