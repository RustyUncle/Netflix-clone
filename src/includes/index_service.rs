use actix_session::Session;
use actix_web::{web::{self}, get, HttpResponse, error::{Error, self}};
use tera::{Tera, Context};
use sqlx::postgres::PgPool;
use crate::{includes::models::{Entity, Categories}, redirect};


#[get("/")]
async fn index(tmpl: web::Data<Tera>, session: Session, db: web::Data<PgPool>) -> Result<HttpResponse, Error> {

    let mut ctx = Context::new();

    if let None = session.get::<String>("username")?{return Ok(redirect("/login"));}

    let query = sqlx::query_as::<_, Entity>("SELECT * FROM entities ORDER BY RANDOM() LIMIT 1")
        .fetch_all(&**db)
        .await;

        match query {
            Ok(entity) => {ctx.insert("entity", &entity);},
            Err(_) => {}
        }

        let query = sqlx::query_as::<_, Categories>("SELECT * FROM categories")
        .fetch_all(&**db)
        .await;

        let mut entity = Vec::new();

        match query {
            Ok(category) => {

                ctx.insert("category", &category);

                for id in &category{
                    let result = sqlx::query_as::<_, Entity>("SELECT * FROM entities WHERE categoryid = $1 ORDER BY RANDOM() LIMIT 30")
                        .bind(id.id)
                        .fetch_all(&**db)
                        .await
                        .unwrap();
                    
                    entity.push(result);
                }
            },
            Err(_) => {}
        }
        ctx.insert("entities", &entity);
        
        let body = tmpl.render("index.html", &ctx).map_err(error::ErrorInternalServerError)?;
        Ok(HttpResponse::Ok().body(body))
}

// Logout
#[get("/logout")]  
async fn logout(session: Session) -> Result<HttpResponse, Error> {
    session.remove("username");
    Ok(redirect("/login"))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
    .service(logout);
}

