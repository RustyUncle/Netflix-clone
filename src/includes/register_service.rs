use actix_session::Session;
use actix_web::{web::{self}, get, post, HttpResponse, error::{Error, self}};
use tera::{Tera, Context};
use sqlx::postgres::PgPool;
use super::constants::*;
use crate::{redirect, includes::models::Createuser};
use super::register_validator::*;


#[get("/register")]
async fn register_get(tmpl: web::Data<Tera>, session: Session) -> Result<HttpResponse, Error> {

    let ctx = Context::new();

    if let Some(_) = session.get::<String>("username")? {
        return Ok(redirect("/"));
    }

    let body = tmpl.render("register.html", &ctx).map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(body))
}

#[post("/register")]
async fn register_post(tmpl: web::Data<Tera>, form: web::Form<Createuser>, db: web::Data<PgPool>) -> Result<HttpResponse, Error> {

    let mut context = Context::new();

    let validator = validator(form.fname.as_str(), form.lname.as_str(), form.uname.as_str(), form.email.as_str(), form.email2.as_str(), form.pass.as_str(), form.pass2.as_str(), &db).await;
    
    let mut create_user = Vec::new();
    
    for result in validator {
        match result {
            Ok(err) => {
                match err {
                    e if e == FNAME_LENGHT.to_string() => context.insert("fname_lenght", &e),
                    e if e == SPECIAL_FNAME.to_string() => context.insert("fname_special", &e),
                    e if e == LNAME_LENGHT.to_string() => context.insert("lname_lenght", &e),
                    e if e == SPECIAL_LNAME.to_string() => context.insert("lname_special", &e),
                    e if e == UNAME_LENGHT.to_string() => context.insert("uname_lenght", &e),
                    e if e == SPECIAL_UNAME.to_string() => context.insert("uname_special", &e),
                    e if e == EMAIL_INVALID.to_string() => context.insert("email_invalid", &e),
                    e if e == EMAIL_DONT_MATCH.to_string() => context.insert("email_notmatch", &e),
                    e if e == PASS_LENGHT.to_string() => context.insert("pass_lenght", &e),
                    e if e == PASS_DONT_MATCH.to_string() => context.insert("pass_notmatch", &e),
                    e if e == UNAME_USED.to_string() => context.insert("uname_used", &e),
                    e if e == EMAIL_USED.to_string() => context.insert("email_used", &e),
                    _ =>context.insert("", &"")
                 }
            },
            Err(user) => {
                create_user.push(user); // everthing is ok than push user input to vec
            }
        }
    }

    if !create_user.is_empty() {

        let result = sqlx::query("INSERT INTO users (fname, lname, uname, email, pass) VALUES ($1,$2,$3,$4,$5)")
            .bind(create_user[0].to_string()) //fname
            .bind(create_user[1].to_string()) //lname
            .bind(create_user[2].to_string()) //uname
            .bind(create_user[3].to_string()) //email
            .bind(create_user[4].to_string()) //pass
            .execute(&**db)
            .await;

        match result {
            Ok(_) => return Ok(redirect("/login")),
            Err(_) => return Ok(redirect("/register"))
        }
    }

    let body = tmpl.render("register.html", &context).map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(body))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(register_get)
    .service(register_post);
}