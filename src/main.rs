mod founder;

use std::{sync::Mutex, str::FromStr};

use founder::{Founder, FounderUuid};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, http::header::ContentType};

use serde::{Deserialize, Serialize};

struct FounderState
{
    founders: Mutex<Vec<Founder>>
}

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    let preinit = vec!
    [
        Founder
        {
            uuid: FounderUuid::new(),
            name: String::from("Tim Apple"),
            company_name: String::from("Apple"),
            bio: String::from("Computer but sexy"),
            image: std::path::PathBuf::from_str("images/test.jpg").unwrap()
        },
        Founder
        {
            uuid: FounderUuid::new(),
            name: String::from("Jeff Bezos"),
            company_name: String::from("Amazon"),
            bio: String::from("Imagine shopping but internet"),
            image: std::path::PathBuf::from_str("images/pngtest.png").unwrap()
        },
    ];
    let founder_list = web::Data::new
    (
        FounderState
        {
            founders: Mutex::new(preinit)
        }
    );

    HttpServer::new(move || {
        App::new()
            .service(home)
            .service(get_founder)
            .service(get_founder_img)
            .app_data(founder_list.clone())
    })
    .bind(("127.0.0.1", 8080))?
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
#[get("/get-founder")]
async fn get_founder(query: web::Query<GetFounderQueryParams>, data: web::Data<FounderState>) -> impl Responder
{
    let list = data.founders.lock().unwrap();
    let num_requested = match query.count
    {
        Some(num) => usize::min(num as usize, list.len()),
        None => 1
    };
    let indices = rand::seq::index::sample(&mut rand::thread_rng(), list.len(), num_requested);
    let mut founders = Vec::new();
    for i in indices
    {
        founders.push(&list[i]);
    }
    let result = serde_json::to_string_pretty(&founders);
    match result
    {
        Ok(json) => HttpResponse::Ok().content_type(ContentType::json()).body(json),
        Err(output) => HttpResponse::NotFound().body(output.to_string())
    }
}

#[derive(Serialize, Deserialize)]
struct ImgRequest{ file_name: String }
#[get("/get-founder-img")]
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