use actix_session::Session;
use actix_web::{web::{self}, get, HttpResponse, error::{Error, self}};
use tera::{Tera, Context};
use sqlx::{postgres::PgPool, Row};
use crate::{redirect, includes::models::{Entity, Categories, Progress, Videos}};

#[get("/entity/{id}")]
async fn get_entity(tmpl: web::Data<Tera>, path: web::Path<i32>, session: Session, db: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    
    let id = path.into_inner();
    if let None = session.get::<String>("username")? {
        return Ok(redirect("/login"));
    }
    
    let query = sqlx::query_as::<_, Entity>("SELECT * FROM entities WHERE id = $1")
        .bind(id)
        .fetch_one(&**db)
        .await;

    match query {
        Ok(entity) => {
            ctx.insert("entity", &vec![entity]);
        },
        Err(_) => {
            ctx.insert("bad_request", &"No ID passed on!");

            let body = tmpl.render("notfound.html", &ctx).map_err(error::ErrorInternalServerError)?;
            return Ok(HttpResponse::Ok().body(body));
        }
    }

    if let Some(user) = session.get::<String>("username")? {
                
        let query = sqlx::query("SELECT videoid FROM videoprogress INNER JOIN videos ON videoprogress.videoid = videos.id
            WHERE videos.entityid = $1 AND videoprogress.username = $2 ORDER BY videoprogress.datemodified DESC LIMIT 1")
            .bind(id)
            .bind(&user)
            .fetch_all(&**db)
            .await;

        match query{
            Ok(videoid) => {

                if videoid.is_empty(){
                    let query = sqlx::query("SELECT id FROM videos WHERE entityid = $1 ORDER BY season, episode ASC LIMIT 1")
                    .bind(id)
                    .fetch_one(&**db)
                    .await;

                    match query{
                        Ok(videoi) => {

                            let videoid = videoi.get::<i32,_>("id");
                            ctx.insert("videoid", &videoid);

                            let query = sqlx::query_as::<_, Progress>("SELECT * FROM videoprogress WHERE videoid = $1 AND username = $2 ")
                            .bind(videoid)
                            .bind(&user)
                            .fetch_one(&**db)
                            .await;

                            if let Ok(_) = query {
                                ctx.insert("inprogress", &"Continue watching");
                            }
                            else{
                                ctx.insert("inprogress", &"Play");
                            }

                            let query = sqlx::query_as::<_, Progress>("SELECT * FROM videoprogress WHERE username = $1 AND finished = 1 ")
                            .bind(&user)
                            .fetch_all(&**db)
                            .await;

                            if let Ok(hasseen) = query {
                                ctx.insert("hasseen", &hasseen);
                            }
                            else{
                                ctx.insert("hasseen", &"");
                            }
                        },
                        Err(_) => {}
                    }

                }
                else {
                    let videoid = videoid[0].get::<i32,_>("videoid");
                    ctx.insert("videoid", &videoid);

                    let query = sqlx::query_as::<_, Progress>("SELECT * FROM videoprogress WHERE videoid = $1 AND username = $2")
                        .bind(videoid)
                        .bind(&user)
                        .fetch_one(&**db)
                        .await;

                        if let Ok(_) = query {
                            ctx.insert("inprogress", &"Continue watching");
                        }
                        else{
                            ctx.insert("inprogress", &"Play");
                        }

                    let query = sqlx::query_as::<_, Progress>("SELECT * FROM videoprogress WHERE username = $1 AND finished = 1")
                        .bind(&user)
                        .fetch_all(&**db)
                        .await;

                        if let Ok(hasseen) = query {
                            ctx.insert("hasseen", &hasseen);
                        }
                        else{
                            ctx.insert("hasseen", &"");
                        }        
                }
            },
            Err(_) => {}
        }
    }

    let query = sqlx::query_as::<_, Videos>("SELECT * FROM videos WHERE entityid = $1 AND ismovie = 0 ORDER BY season, episode ASC")
    .bind(id)
    .fetch_all(&**db)
    .await;

    let mut seasons = Vec::new();

    match query {
        Ok(season) => {
            ctx.insert("episodes", &season);

            for seasn in &season {
                seasons.push(seasn.season);
            }
        },
        Err(_) => {}
    }

    seasons.dedup();
    ctx.insert("seasons", &seasons);

    let query = sqlx::query_as::<_, Entity>("SELECT * FROM entities")
        .fetch_all(&**db)
        .await;

    match query {
        Ok(might) => ctx.insert("might_like", &might),
        Err(_) => {}
    }

    let query = sqlx::query_as::<_, Categories>("SELECT * FROM categories")
        .fetch_all(&**db)
        .await;

    match query {
        Ok(category) => ctx.insert("categories", &category),
        Err(_) => {}
    }

    let body = tmpl.render("entity.html", &ctx).map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(body))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_entity);
}