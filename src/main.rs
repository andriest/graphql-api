use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use graphql::{
    db::{establish_connection, DatabaseKind},
    handlers::graphql::{graphql, playground},
    schema_graphql::create_schema,
};
use std::env;
use std::sync::Arc;

const BANNER: &str = r#"
.d88b  888b.    db    888b. 8   8 .d88b. 8    
8P www 8  .8   dPYb   8  .8 8www8 8P  Y8 8    
8b  d8 8wwK'  dPwwYb  8wwP' 8   8 8b wd8 8    
`Y88P' 8  Yb dP    Yb 8     8   8 `Y88Pw 8888
┌────────────────────────────────────────────────────────────────┐
│ Server version 1.0.1                                           │
│ Playground by ZMAB (andriebamz@gmail.com)                      │
└────────────────────────────────────────────────────────────────┘
"#;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = env::var("HOST").expect("Missing `HOST` env variable");
    let port = env::var("PORT").expect("Missing `PORT` env variable");

    println!("{}", BANNER);

    // create Juniper schema
    let schema = Arc::new(create_schema());

    // database connection pool
    let db_pool = establish_connection(DatabaseKind::Main);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(schema.clone()))
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .service(
                web::resource("/graphql")
                    .route(web::get().to(graphql))
                    .route(web::post().to(graphql)),
            )
            .service(web::resource("/playground").route(web::get().to(playground)))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
