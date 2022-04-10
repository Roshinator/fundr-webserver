mod founder;
mod dbactions;
mod schema;

use std::{str::FromStr};

#[macro_use]
extern crate diesel;
use diesel::{r2d2::ConnectionManager, PgConnection};
use founder::{Founder, NewFounder};
use actix_web::{get, put, web, App, HttpResponse, HttpServer, Responder, http::header::{ContentType}, post, delete};

use r2d2::{Pool};
use serde::{Deserialize, Serialize};

type DBPool = Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    println!("🍆🍆🍆");
    println!("🍑🍑🍑");




    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .service(home)
            .service(get_founder)
            .service(get_founder_img)
            .service(update_founder)
            .service(create_founder)
            .service(delete_founder)
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(("192.168.1.21", 8080))?
    .run()
    .await
}

#[get("/")]
async fn home() -> impl Responder
{
    HttpResponse::Ok().body("Yay it wowks UwU :3")
}

#[derive(Serialize, Deserialize)]
struct GetFounderQueryParams { count: Option<u8> }
#[get("/founder")]
async fn get_founder(query: web::Query<GetFounderQueryParams>, pool: web::Data<DBPool>) -> Result<impl Responder, actix_web::error::Error>
{
    let num_requested = match query.count
    {
        Some(x) => x as i64,
        None => 1
    };
    let result = web::block(move ||
        {
            let conn = pool.get().expect("Connection retreival failed");
            dbactions::get_random_user(&conn, num_requested)
        }
    ).await
    .map_err(actix_web::error::ErrorInternalServerError)?
    .map_err(actix_web::error::ErrorBadRequest)?;

    let json_result = serde_json::to_string_pretty(&result);
    match json_result
    {
        Ok(json) => Ok(HttpResponse::Ok().content_type(ContentType::json()).body(json)),
        Err(output) => Ok(HttpResponse::NotFound().body(output.to_string()))
    }
}

#[derive(Serialize, Deserialize)]
struct ImgRequest{ file_name: String }
#[get("/founder/image")]
async fn get_founder_img(query: web::Query<ImgRequest>) -> impl Responder
{
    let path = match std::path::PathBuf::from_str(&query.file_name)
    {
        Ok(path) if path.is_file() => path,
        _ => {return HttpResponse::NotFound().finish()}
    };
    let content_type = if path.extension().unwrap() == "jpeg" || path.extension().unwrap() == "jpg"
    {
        ContentType::jpeg()
    }
    else
    {
        ContentType::png()
    };
    match std::fs::read(&query.file_name)
    {
        Ok(bytes) => HttpResponse::Ok().content_type(content_type).body(bytes),
        Err(_) => HttpResponse::NotFound().finish()
    }
}

#[put("/founder")]
async fn update_founder(updated_founder: web::Json<Founder>, pool: web::Data<DBPool>) -> Result<impl Responder, actix_web::error::Error>
{
    let new_founder: Founder = updated_founder.0;
    //Insert into database here
    let _result = web::block(move ||
        {
            let conn = pool.get().expect("Connection retreival failed");
            dbactions::update_user(&conn, new_founder)
        }
    ).await
    .map_err(actix_web::error::ErrorInternalServerError)?
    .map_err(actix_web::error::ErrorBadRequest)?;

    Ok(HttpResponse::Ok().finish())
}

#[post("/founder")]
async fn create_founder(new_founder_json: web::Json<NewFounder>, pool: web::Data<DBPool>) -> Result<impl Responder, actix_web::error::Error>
{
    let new_founder = new_founder_json.0;

    let new_founder = web::block(move ||
        {
            let conn = pool.get().expect("Connection retreival failed");
            dbactions::insert_user(&conn, new_founder)
        }
    ).await
    .map_err(actix_web::error::ErrorInternalServerError)?
    .map_err(actix_web::error::ErrorBadRequest)?;

    let result = serde_json::to_string_pretty(&new_founder);
    match result
    {
        Ok(json) => Ok(HttpResponse::Ok().content_type(ContentType::json()).body(json)),
        Err(output) => Ok(HttpResponse::InternalServerError().body(output.to_string()))
    }
}

#[delete("/founder")]
async fn delete_founder(founder_json: web::Json<Founder>, pool: web::Data<DBPool>) -> Result<impl Responder, actix_web::error::Error>
{
    let new_founder = founder_json.0;

    let new_founder = web::block(move ||
        {
            let conn = pool.get().expect("Connection retreival failed");
            dbactions::delete_user(&conn, new_founder)
        }
    ).await
    .map_err(actix_web::error::ErrorInternalServerError)?
    .map_err(actix_web::error::ErrorBadRequest)?;

    let result = serde_json::to_string_pretty(&new_founder);
    match result
    {
        Ok(json) => Ok(HttpResponse::Ok().content_type(ContentType::json()).body(json)),
        Err(output) => Ok(HttpResponse::InternalServerError().body(output.to_string()))
    }
}
