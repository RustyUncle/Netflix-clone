use actix_session::Session;
use actix_web::{web::{self}, get, post, HttpResponse, error::{Error, self}};
use tera::{Tera, Context};
use sqlx::postgres::PgPool;
use super::constants::*;
use crate::{redirect, includes::models::{Loginuser}};
use super::login_validator::*;


#[get("/login")]
async fn login_get(tmpl: web::Data<Tera>, session: Session) -> Result<HttpResponse, Error> {

    let context = Context::new();

    if let Some(_) = session.get::<String>("username")? {
        return Ok(redirect("/"));
    }

    let body = tmpl.render("login.html", &context).map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(body))
}

#[post("/login")]
async fn login_post(tmpl: web::Data<Tera>, form: web::Form<Loginuser>, db: web::Data<PgPool>, session: Session) -> Result<HttpResponse, Error> {

    let mut context = Context::new();

    let validator = login_validator(form.email.as_str(), form.pass.as_str(), &db).await;

    match validator {
        Ok(o) => {
            session.insert("username", &o);
            return Ok(redirect("/"))
        },
        Err(er) => {
            match er {
                er if er == EMAIL_INVALID.to_string() => context.insert("email_invalid", &er),
                er if er == EMAILPASS_DONT_MATCH.to_string().to_string() => context.insert("empw_notmatch", &er),
                _ => context.insert("", &"")
            }
        }
    }

    let body = tmpl.render("login.html", &context).map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(body))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(login_get)
    .service(login_post);
}