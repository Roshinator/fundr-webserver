mod founder;

use std::{sync::Mutex, str::FromStr};

use founder::{Founder, FounderUuid};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, http::header::ContentType, error};
use futures::StreamExt;
use rand::Rng;
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
            name: String::from("Jeff Bezos"),
            company_name: String::from("Amazon"),
            bio: String::from("Imagine shopping but internet"),
            image: std::path::PathBuf::from_str("images/test.jpg").unwrap()
        }
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
            .service(get_test)
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

#[get("/get-founder")]
async fn get_test(data: web::Data<FounderState>) -> impl Responder
{
    let list = data.founders.lock().unwrap();
    let founder = &list[rand::thread_rng().gen_range(0..list.len())];
    let result = serde_json::to_string_pretty(founder);
    match result
    {
        Ok(json) => HttpResponse::Ok().content_type(ContentType::json()).body(json),
        Err(output) => HttpResponse::NotFound().body(output.to_string())
    }
}

#[derive(Serialize, Deserialize)]
struct ImgRequest{ file_name: String }
#[get("/get-founder-img")]
async fn get_founder_img(info: web::Query<ImgRequest>) -> impl Responder
{
    match std::fs::read(&info.file_name)
    {
        Ok(bytes) => HttpResponse::Ok().content_type(ContentType::jpeg()).body(bytes),
        Err(_) => HttpResponse::NotFound().finish()
    }
}