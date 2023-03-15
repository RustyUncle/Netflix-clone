use actix_session::Session;
use actix_web::{web::{self}, get, HttpResponse, error::{Error, self}};
use log::info;
use sqlx::postgres::PgPool;
use tera::{Tera, Context};

use crate::{redirect, includes::models::Videos};


#[get("/watch/{id}")]
async fn watch(tmpl: web::Data<Tera>, path: web::Path<i32>, session: Session, db: web::Data<PgPool>) -> Result<HttpResponse, Error> {

    let mut ctx = Context::new();
    
    let id = path.into_inner();

    if let None = session.get::<String>("username")? {
        return Ok(redirect("/login"));
    }

    if let Some(user) = session.get::<String>("username")?{
        ctx.insert("username", &user);
    }

    let query = sqlx::query("UPDATE videos SET views=views+1 WHERE id = $1")
        .bind(id)
        .execute(&**db)
        .await
        .unwrap();

    info!("{:?}", query);

    let query = sqlx::query_as::<_, Videos>("SELECT * FROM videos WHERE id = $1")
        .bind(id)
        .fetch_one(&**db)
        .await;

    match query {
        Ok(videos) => {
            let query = sqlx::query_as::<_, Videos>("SELECT * FROM videos WHERE entityid = $1 AND id != $2 AND
                (season = $3 AND episode > $4) ORDER BY season, episode ASC LIMIT 1")
                .bind(videos.entityid)
                .bind(videos.id)
                .bind(videos.season)
                .bind(videos.episode)
                .fetch_all(&**db)
                .await;

            match query {
                Ok(videos) => {
                    if videos.is_empty(){
                        let query = sqlx::query_as::<_, Videos>("SELECT * FROM videos WHERE season <= 1 AND episode <= 1 AND id != $1 ORDER BY views DESC LIMIT 1")
                            .bind(id)
                            .fetch_all(&**db)
                            .await; 

                        match query{
                            Ok(videos) => {ctx.insert("videoss", &videos);},
                            Err(_) => {}
                        }
                    }
                    ctx.insert("videoss", &videos);
                },
                Err(_) => {}
            }
            ctx.insert("videos", &vec![&videos]);
        },
        Err(_) => {
            ctx.insert("bad_request", &"No ID passed on!");

            let body = tmpl.render("nothing.html", &ctx).map_err(error::ErrorInternalServerError)?;
            return Ok(HttpResponse::Ok().body(body));
        }
    }

    let body = tmpl.render("watch.html", &ctx).map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(body))
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(watch);
}