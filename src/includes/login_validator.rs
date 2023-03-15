use sqlx::{postgres::PgPool, Row};
use voca_rs::*;
use actix_web::web;
use argon2::{self, Config};
use super::constants::*;

fn sanitazor_email(input: &str) -> String {
    
    input._strip_tags().replace(" ", "")
}
fn sanitazor_pass(input: &str) -> String {
    
    input._strip_tags()
}

pub async fn login_validator(email: &str, pass: &str, db: &web::Data<PgPool>) -> Result<String, String> {

    let email = sanitazor_email(email);
    let pass = sanitazor_pass(pass);

    if !mailchecker::is_valid(email.as_str()) {
        return Err(EMAIL_INVALID.to_string())
    }

    let salt = b"randomsaltplussugar";
    let config = Config::default();
    let hash = argon2::hash_encoded(pass.as_bytes(), salt, &config).unwrap();

    let result = sqlx::query("SELECT uname FROM users WHERE email = $1 AND pass = $2")
        .bind(email)
        .bind(&hash)
        .fetch_one(&***db)
        .await;

    match result {
        Ok(o) => Ok(o.get::<String,_>("uname")),
        Err(_) => Err(EMAILPASS_DONT_MATCH.to_string())
    }
 }