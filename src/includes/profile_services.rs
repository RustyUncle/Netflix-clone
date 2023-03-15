use actix_session::Session;
use actix_web::{web::{self}, get, post, HttpResponse, error::{Error, self}};
use tera::{Tera, Context};
use sqlx::postgres::PgPool;
use crate::{redirect, includes::models::Updatepassword};

use super::constants::*;
use super::models::User;
use super::profile_validator::*;


#[get("/profile")]
async fn profile_details_get(tmpl: web::Data<Tera>, session: Session, db: web::Data<PgPool>) -> Result<HttpResponse, Error> {

    let mut ctx = Context::new();

    if let None = session.get::<String>("username")? {
        return Ok(redirect("/login"));
    }
    if let Some(username) = session.get::<String>("username")? {

        let query = sqlx::query_as::<_,User>("SELECT * FROM users WHERE uname = $1 ")
            .bind(username)
            .fetch_all(&**db)
            .await;
        
        match query {
            Ok(user) => {
                ctx.insert("users", &user)
            },
            Err(_) =>{}
        }
    }  

    let body = tmpl.render("profile.html", &ctx).map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(body))
}

#[post("/profile")]
async fn profile_details_post(tmpl: web::Data<Tera>, form: web::Form<User>, session: Session, db: web::Data<PgPool>) -> Result<HttpResponse, Error> {

    let mut context = Context::new();
    
    if let Some(username) = session.get::<String>("username")? {

        let mut update = Vec::new();

        let validator = validator_up(form.fname.as_str(), form.lname.as_str(), form.email.as_str(), &username, &db).await;

        for valid in validator {
            match valid {
                Ok(err) => {
                    match err {
                        e if e == FNAME_LENGHT.to_string() => context.insert("fname_lenght", &e),
                        e if e == SPECIAL_FNAME.to_string() => context.insert("fname_special", &e),
                        e if e == LNAME_LENGHT.to_string() => context.insert("lname_lenght", &e),
                        e if e == SPECIAL_LNAME.to_string() => context.insert("lname_special", &e),
                        e if e == EMAIL_INVALID.to_string() => context.insert("email_invalid", &e),
                        e if e == EMAIL_USED.to_string() => context.insert("email_used", &e),
                        _ =>context.insert("", &"")
                    }
                },
                Err(user) => {
                update.push(user);
                }
            }
        }

        if !update.is_empty() {

            context.insert("detail", &"Profile updated successfully!");

            let query = sqlx::query("UPDATE users set fname = $1, lname = $2, email = $3 WHERE uname = $4 ")
                .bind(update[0].to_string())
                .bind(update[1].to_string())
                .bind(update[2].to_string())
                .bind(&username)
                .execute(&**db)
                .await;

            match query {
                Ok(_) => {
                    let query = sqlx::query_as::<_,User>("SELECT * FROM users WHERE uname = $1 ")
                    .bind(&username)
                    .fetch_all(&**db)
                    .await;
            
                    match query {
                        Ok(user) => {
                            context.insert("users", &user)
                        },
                        Err(_) =>{}
                    }
                },
                Err(_) => {}
            }
        }
    }   

    let body = tmpl.render("profile.html", &context).map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(body))
}

#[post("/pw_up")]
async fn password_update(tmpl: web::Data<Tera>, form: web::Form<Updatepassword>, db: web::Data<PgPool>, session: Session) -> Result<HttpResponse, Error> {

    let mut context = Context::new();

    if let Some(username) = session.get::<String>("username")? {

        let validator = validator_up_pw(username.as_str(), form.oldpass.as_str(), form.newpass.as_str(), form.newpass2.as_str(), &db).await;

        match validator {
            Ok(e) => {
                match e {

                    e if e == PASS_NOT_MATCH.to_string() => context.insert("empw_notmatch", &e),
                    e if e == PASS_LENGHT.to_string() => context.insert("pass_lenght", &e),
                    e if e == PASS_DONT_MATCH.to_string() => context.insert("pass_notmatch", &e),
                    _ =>context.insert("", &"")
                }
            },
            Err(password) => {
                let query = sqlx::query("UPDATE users SET pass = $1 WHERE uname = $2")
                    .bind(password)
                    .bind(username)
                    .execute(&**db)
                    .await;

                match query {
                    Ok(_) => {context.insert("updated", &"Password updated successfully!")},
                    Err(_) => {}
                }
            },
        }
    }
    
    let body = tmpl.render("profile.html", &context).map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(body))
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(profile_details_get)
    .service(profile_details_post)
    .service(password_update);
}