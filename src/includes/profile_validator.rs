use sqlx::postgres::PgPool;
use voca_rs::*;
use actix_web::web;
use argon2::{self, Config};
use super::constants::*;

fn sanitazor_fname_lname(input: &str) -> String {
    
    input._strip_tags().trim().replace(" ", "").to_lowercase()._upper_first()
}
fn sanitazor_email(input: &str) -> String {
    
    input._strip_tags().replace(" ", "")
}
fn sanitazor_pass(input: &str) -> String {
    
    input._strip_tags()
}

fn validate_fname(fname: &str) -> Result<String, String> {
    
    let fname = sanitazor_fname_lname(fname);

    if !fname.chars().into_iter().all(|p| p.is_alphabetic()) {
        return Ok(SPECIAL_FNAME.to_string());
    }
    else if !fname.chars().all(|p| SPECIAL.contains(p)) {
        return Ok(SPECIAL_FNAME.to_string());
    }
    else if fname.chars().count() < 2 || fname.chars().count() > 25 {
        return Ok(FNAME_LENGHT.to_string());
    }
    else {
        Err(fname)
    }
}
fn validate_lname(lname: &str) -> Result<String, String> {

    let lname = sanitazor_fname_lname(lname);

    if !lname.chars().into_iter().all(|p| p.is_alphabetic()) {
        return Ok(SPECIAL_LNAME.to_string())
    }
    else if !lname.chars().all(|p| SPECIAL.contains(p)) {
        return Ok(SPECIAL_LNAME.to_string());
    }
    else if lname.chars().count() < 2 || lname.chars().count() > 25 {
        return Ok(LNAME_LENGHT.to_string())
    }
    else {
        Err(lname)
    }
}

fn validate_email_up(email: &str) -> Result<String, String> {

    let email = sanitazor_email(email);

    if !mailchecker::is_valid(email.as_str()) {
        return Ok(EMAIL_INVALID.to_string())
    }
    else {
        Err(email)
    }
}

fn validata_pass(pass: &str, pass2: &str) -> Result<String, String> {

    let pass = sanitazor_pass(pass);
    let pass2 = sanitazor_pass(pass2);

    if pass.chars().count() < 6 || pass.chars().count() > 25 {
        return Ok(PASS_LENGHT.to_string())
    }
    else if pass != pass2 {
        return Ok(PASS_DONT_MATCH.to_string())
    }
    else {
        let salt = b"randomsaltplussugar";
        let config = Config::default();
        let hash = argon2::hash_encoded(pass.as_bytes(), salt, &config).unwrap();
        Err(hash)
    }
}

pub async fn validator_up(fname: &str, lname: &str,  email: &str, un: &str,  db: &web::Data<PgPool>) -> Vec<Result<String, String>> {
    
    let fname = validate_fname(fname);
    let lname = validate_lname(lname);
    let email = validate_email_up(email);

    let em = em_controller_up(email.clone(), un.to_string(), &db).await;

    if fname.is_ok() {
        return vec![fname];
    }
    else if lname.is_ok() {
        return vec![lname];
    }
    else if em.is_ok() {
        return vec![em];
    }
    else {
        vec![fname,lname,email]
    }
}

pub async fn validator_up_pw(un: &str, oldpass: &str, newpass: &str, newpass2: &str, db: &web::Data<PgPool>) -> Result<String, String> {

    let oldpass = sanitazor_pass(oldpass);
    let newpass = validata_pass(newpass, newpass2);

    let salt = b"randomsaltplussugar";
    let config = Config::default();
    let hash = argon2::hash_encoded(oldpass.as_bytes(), salt, &config).unwrap();

    let result = sqlx::query("SELECT uname FROM users WHERE uname = $1 AND pass = $2")
        .bind(un)
        .bind(&hash)
        .fetch_one(&***db)
        .await;

    if let Err(_) = result {
        return Ok(PASS_NOT_MATCH.to_string());
    }
    else {
        newpass
    }
 }

async fn em_controller_up(email: Result<String, String>, un: String, db: &web::Data<PgPool>) -> Result<String, String> {
   
    match email {
         Ok(o) => Ok(o),
         Err(er) => {
             let result = sqlx::query("SELECT email FROM users WHERE email = $1 AND uname != $2 ")
                .bind(er)
                .bind(un)
                .fetch_one(&***db)
                .await;

             match result {
                 Ok(_) => Ok(EMAIL_USED.to_string()),
                 Err(_) => Err("Not found any result".to_string())
             }
         }
     }
 }