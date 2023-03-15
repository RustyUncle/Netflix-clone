use actix_session::Session;
use actix_web::{web, post, get, HttpResponse, error::{Error, self}};
use log::info;
use sqlx::{postgres::PgPool, Row};
use tera::{Tera, Context};

use crate::{redirect, includes::models::{Entity, Videoprogress}};
use super::models::{Search, Progress};

#[get("/ajax/search")]
async fn get_search(tmpl: web::Data<Tera>, session: Session) -> Result<HttpResponse, Error> {

    let mut ctx = Context::new();

    if let None = session.get::<String>("username")?{
        return Ok(redirect("/login"));
    }
    if let Some(user) = session.get::<String>("username")? {
        ctx.insert("username", &user);
    }
    
    let body = tmpl.render("search.html", &ctx).map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(body))
}

#[post("/ajax/search")]
async fn post_search(tmpl: web::Data<Tera>, path: web::Form<Search>, session: Session, db: web::Data<PgPool>) -> Result<HttpResponse, Error> {

    let mut ctx = Context::new();

    if let None = session.get::<String>("username")?{
        return Ok(redirect("/login"));
    }
    if let Some(user) = session.get::<String>("username")?{
        ctx.insert("username", &user);
    }

    let term = path.term.clone();
    let un = path.username.clone();

    if !term.is_empty() && !un.is_empty(){

        let query = sqlx::query_as::<_, Entity>("SELECT * FROM entities WHERE name LIKE CONCAT('%', $1, '%') LIMIT 30 ")
        .bind(term)
        .fetch_all(&**db)
        .await;

        match query {
            Ok(search) => {
                ctx.insert("searches", &search);
            },
            Err(_) => {}    
        }
    }

    let body = tmpl.render("search.html", &ctx).map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(body))
}



#[post("/ajax/adduration")]
async fn add_duration(path: web::Form<Progress>, db: web::Data<PgPool>) -> Result<HttpResponse, Error> {

    let un= path.username.clone();
    let v_id= path.videoid; 
    
    if !v_id.is_negative() && !un.is_empty(){

        let query = sqlx::query_as::<_,Progress>("SELECT * FROM videoprogress WHERE username = $1 AND videoid = $2")
            .bind(&un).bind(v_id)
            .fetch_all(&**db)
            .await;

        match query {
            Ok(progress) => {

                if progress.is_empty(){

                    let query = sqlx::query("INSERT INTO videoprogress (username, videoid) VALUES ($1, $2)")
                        .bind(&un)
                        .bind(v_id)
                        .execute(&**db)
                        .await
                        .unwrap();

                    info!("{:?}", query); 
                    return Ok(HttpResponse::Ok().body(""));
                }
            },
            Err(_) => {}
        }
    }
        Ok(HttpResponse::Ok().body(""))
}

#[post("/ajax/updateduration")]
async fn set_finished(path: web::Form<Videoprogress>, db: web::Data<PgPool>) -> Result<HttpResponse, Error> {

    let v_id= path.videoid.clone(); 
    let un= path.username.clone();
    let progress= path.progress; 
    
    if !v_id.is_negative() && !un.is_empty() && !progress.is_nan(){

        let query = sqlx::query("UPDATE videoprogress SET progress = $3, datemodified = NOW() WHERE username = $1 AND videoid = $2")
            .bind(&un)
            .bind(v_id)
            .bind(progress)
            .execute(&**db)
            .await
            .unwrap();

        info!("{:?}", query); 
        return Ok(HttpResponse::Ok().body(""));
    }
    Ok(HttpResponse::Ok().body(""))
}

#[post("/ajax/setfinished")]
async fn update_duration(path: web::Form<Progress>, db: web::Data<PgPool>) -> Result<HttpResponse, Error> {

    let v_id= path.videoid; 
    let un= path.username.clone();
    
    if !v_id.is_negative() && !un.is_empty(){

        let query = sqlx::query("UPDATE videoprogress SET finished = 1, progress =0 WHERE username = $1 AND videoid = $2")
            .bind(&un)
            .bind(v_id)
            .execute(&**db)
            .await
            .unwrap(); 

        info!("{:?}", query); 
        return Ok(HttpResponse::Ok().body(""));
    }  
    Ok(HttpResponse::Ok().body(""))
}

#[post("/ajax/progress")]
async fn set_start_time(path: web::Form<Progress>, db: web::Data<PgPool>) -> Result<HttpResponse, Error> {

    let un= path.username.clone();
    let vid= path.videoid; 
    
    if !vid.is_negative() && !un.is_empty(){

        let query = sqlx::query("SELECT progress FROM videoprogress WHERE username = $1 AND videoid = $2")
            .bind(un)
            .bind(vid)
            .fetch_one(&**db)
            .await;

        match query {

            Ok(progress) => {
                return Ok(HttpResponse::Ok().json(progress.get::<i32,_>("progress")));
            },
            Err(_) => {},
        }
    }
    Ok(HttpResponse::Ok().body(""))
}

pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(set_start_time)
    .service(add_duration)
    .service(update_duration)
    .service(set_finished)
    .service(get_search)
    .service(post_search);
}