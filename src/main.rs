use actix_session::{storage::CookieSessionStore,SessionMiddleware};
use actix_web::{web, route, App, middleware::Logger, HttpServer, HttpResponse, cookie::Key, error::{Error, self}, http::header};
use sqlx::postgres::PgPoolOptions;
use actix_files::Files;
use tera::{Tera, Context};
use dotenv;

mod includes;
use includes::register_service;
use includes::login_service;
use includes::index_service;
use includes::entity_service;
use includes::watch_service;
use includes::ajax_services;
use includes::category_service;
use includes::movies_service;
use includes::shows_service;
use includes::profile_services;


//Return root page if entry wrong route
#[route("/{path_control:.*}", method = "GET", method = "POST")]
async fn not_found(tmpl: web::Data<Tera>)  -> Result<HttpResponse, Error> {

    let mut ctx = Context::new();

    ctx.insert("bad_request", &"No such path found!");

    let body = tmpl.render("notfound.html", &ctx).map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(body))
}

// Redirection 
pub fn redirect(location: &str) -> HttpResponse {
    HttpResponse::Found().append_header((header::LOCATION, location)).finish()
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug, actix_server=info");

    let db = std::env::var("DATABASE_URL").expect("DB not found!");
    let pool = PgPoolOptions::new().max_connections(1000).connect(&db).await.unwrap();

    env_logger::init();

    HttpServer::new(move|| {

        let mut templates = Tera::new("templates/**/*").expect("Tera not found!");
        templates.autoescape_on(vec![]);

        App::new()
        .wrap(Logger::default())
        .wrap(SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64])).cookie_secure(false).build())
        .app_data(web::Data::new(pool.clone()))
        .app_data(web::Data::new(templates.clone()))
        .service(Files::new("/assets", "./assets").show_files_listing())
        .service(Files::new("/entities", "./entities").show_files_listing())
        .configure(index_service::config)
        .configure(register_service::config)
        .configure(login_service::config)
        .configure(entity_service::config)
        .configure(watch_service::config)
        .configure(ajax_services::config)
        .configure(category_service::config)
        .configure(movies_service::config)
        .configure(shows_service::config)
        .configure(profile_services::config)
        .service(not_found)
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await

}
