use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;
use std::io;
use std::sync::Mutex;

use std::fmt;
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
pub enum MyError {
    ParseError,
    IOError,
}

impl std::error::Error for MyError {}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::ParseError => write!(f, "Parse Error"),
            MyError::IOError => write!(f, "IO Error"),
        }
    }
}

#[path = "../iter3/db_access.rs"]
mod db_access;
#[path = "../iter3/errors.rs"]
mod errors;
#[path = "../iter3/handlers.rs"]
mod handlers;
#[path = "../iter3/models.rs"]
mod models;
#[path = "../iter3/routes.rs"]
mod routes;
#[path = "../iter3/state.rs"]
mod state;

use routes::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not in .env file");
    let db_pool = PgPool::connect(&database_url).await.unwrap();

    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good. You've already asked me ".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}

fn square(val: &str) -> Result<i32, MyError> {
    let num = val.parse::<i32>().map_err(|_| MyError::ParseError)?;
    let mut f = file::open("fictionalfile.txt").map_err(
        |_| MyError::IOError)?;
    let string_to_write = format!("Square of {:?} is {:?}", num, i32::pow(num, 2));
    f.write_all(string_to_write.as_bytes())
        .map_err(|_| MyError::IOError)?;
    Ok(i32::pow(num, 2))
}