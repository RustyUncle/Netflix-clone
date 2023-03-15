use actix_session::Session;
use actix_web::{web::{self}, get, HttpResponse, error::{Error, self}};
use tera::{Tera, Context};
use sqlx::postgres::PgPool;
use crate::{redirect, includes::models::{Categories, Entity}};


#[get("/shows")]
async fn shows(tmpl: web::Data<Tera>, session: Session, db: web::Data<PgPool>) -> Result<HttpResponse, Error> {

    let mut ctx = Context::new();

    if let None = session.get::<String>("username")? {
        return Ok(redirect("/login"));
    }
        
    let query = sqlx::query_as::<_, Entity>("SELECT * FROM entities ORDER BY RANDOM() LIMIT 1")
        .fetch_one(&**db)
        .await;

    match query {
        Ok(entity) => {ctx.insert("entity", &vec![&entity]);},
        Err(_) => {}
    }

    let query = sqlx::query_as::<_, Categories>("SELECT * FROM categories")
        .fetch_all(&**db).await;

    match query {
        Ok(category) => {
            ctx.insert("category", &category); 
        },
        Err(_) => {}
    }
    
    let query = sqlx::query_as::<_,Entity>("SELECT DISTINCT(entities.id), entities.name, entities.preview, entities,thumbnail,
        entities.categoryid FROM entities INNER JOIN videos ON entities.id = videos.entityid WHERE videos.isMovie = 0")
        .fetch_all(&**db)
        .await;
                
    match query{
        Ok(entities) => {ctx.insert("entities", &entities); },
        Err(_) => {}
    }
            
    let body = tmpl.render("shows.html", &ctx).map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(body))
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(shows);
}