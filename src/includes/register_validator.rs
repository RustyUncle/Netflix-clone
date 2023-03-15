use sqlx::postgres::PgPool;
use voca_rs::*;
use actix_web::web;
use argon2::{self, Config};
use super::constants::*;

fn sanitazor_fname_lname(input: &str) -> String {
    
    input._strip_tags().trim().replace(" ", "").to_lowercase()._upper_first()
}
fn sanitazor_uname(input: &str) -> String {
    
    input._strip_tags().replace(" ", "").to_lowercase()._upper_first()
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
fn validate_uname(uname: &str) -> Result<String, String> {

    let uname = sanitazor_uname(&uname);

    if !uname.chars().into_iter().all(|p| p.is_alphabetic()) {
        return Ok(SPECIAL_UNAME.to_string())
    }
    else if !uname.chars().all(|p| SPECIAL.contains(p)) {
        return Ok(SPECIAL_UNAME.to_string());
    }
    else if uname.chars().count() < 2 || uname.chars().count() > 25 {
        return Ok(UNAME_LENGHT.to_string())
    }
    else {
        Err(uname)
    }
}

fn validate_email(email: &str, email2: &str) -> Result<String, String> {

    let email = sanitazor_email(email);
    let email2 = sanitazor_email(email2);

    if !mailchecker::is_valid(email.as_str()) {
        return Ok(EMAIL_INVALID.to_string())
    }
    else if email != email2 {
        return Ok(EMAIL_DONT_MATCH.to_string())
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

async fn un_controller(uname: Result<String, String>, db: &web::Data<PgPool>) -> Result<String, String> {
   
    match uname {
         Ok(o) => Ok(o),
         Err(er) => {
             let result = sqlx::query("SELECT uname FROM users WHERE uname = $1")
                .bind(er)
                .fetch_one(&***db)
                .await;

             match result {
                 Ok(_) => Ok(UNAME_USED.to_string()),
                 Err(_) => Err("Not found any result".to_string())
             }
         }
     }
 }

 async fn em_controller(email: Result<String, String>, db: &web::Data<PgPool>) -> Result<String, String> {
    
     match email {
          Ok(o) => Ok(o),
          Err(er) => {
              let result = sqlx::query("SELECT email FROM users WHERE email = $1")
                .bind(er)
                .fetch_one(&***db)
                .await;

              match result {
                  Ok(_) => Ok(EMAIL_USED.to_string()),
                  Err(_) => Err("Not found any result".to_string())
              }
          }
      }
}


pub async fn validator(fname: &str, lname: &str, uname: &str, email: &str, email2: &str, pass: &str, pass2: &str, db: &web::Data<PgPool>) -> Vec<Result<String, String>> {

    let fname = validate_fname(fname);
    let lname = validate_lname(lname);
    let uname = validate_uname(&uname);
    let email = validate_email(email,email2);
    let pass = validata_pass(pass, pass2);

     let un = un_controller(uname.clone(), &db).await;
     let em = em_controller(email.clone(), &db).await;

    if fname.is_ok() {
        return vec![fname];
    }
    else if lname.is_ok() {
        return vec![lname];
    }
    else if un.is_ok() {
        return vec![un];
    }
    else if em.is_ok() {
        return vec![em];
    }
    else if pass.is_ok() {
        return vec![pass];
    }
    else {
        vec![fname,lname,uname,email,pass]
    }
}