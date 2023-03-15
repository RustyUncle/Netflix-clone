use actix_session::Session;
use actix_web::{web::{self}, get, HttpResponse, error::{Error, self}};
use tera::{Tera, Context};
use sqlx::postgres::PgPool;
use crate::{redirect, includes::models::{Categories, Entity}};


#[get("/category/{id}")]
async fn get_category(tmpl: web::Data<Tera>, path: web::Path<i32>, session: Session, db: web::Data<PgPool>) -> Result<HttpResponse, Error> {

    let mut ctx = Context::new();
    let category_id = path.into_inner();

    if let None = session.get::<String>("username")?{
        return Ok(redirect("/login"));
    }
    if let Some(user) = session.get::<String>("username")?{
        ctx.insert("username", &user);
    }
        
    let query = sqlx::query_as::<_, Entity>("SELECT * FROM entities ORDER BY RANDOM() LIMIT 1")
    .fetch_one(&**db)
    .await;

    match query {
        Ok(entity) => {ctx.insert("entity", &vec![&entity]);},
        Err(_) => {}
    }

    let result = sqlx::query_as::<_, Categories>("SELECT * FROM categories WHERE id = $1 ")
        .bind(category_id)
        .fetch_all(&**db)
        .await;

    match result{
        Ok(category) => {

            ctx.insert("category", &category);

            let result = sqlx::query_as::<_, Entity>("SELECT * FROM entities WHERE categoryid = $1 ORDER BY RANDOM() LIMIT 30")
            .bind(category_id)
            .fetch_all(&**db)
            .await;

            match result {
                Ok(entities) => {ctx.insert("entities", &entities);},
                Err(_) => {}
            }
        },
        Err(_) => {}
    }
        
    let body = tmpl.render("category.html", &ctx).map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(body))
}


pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(get_category);
}

