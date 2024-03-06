// Sets up and configures the Actix web server
use actix_web::{web, App, HttpServer, Responder};
use crate::use_cases::fetch_room_temperature::FetchRoomTemperature;

pub async fn get_room_temperature(data: web::Data<FetchRoomTemperature>) -> impl Responder {
    let temperature = data.execute().unwrap();
    format!("Room temperature: {}", temperature)
}

pub async fn hello() -> impl Responder {
    format!("Hello, Homey!")
}

pub async fn run_server() -> std::io::Result<()> {
    let use_case = FetchRoomTemperature::new(false);
    let port = std::env::var("PORT").unwrap_or_else(|_| String::from("8080"));
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(use_case.clone()))
            .route("/", web::get().to(hello))
            .route("/room_temperature", web::get().to(get_room_temperature))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
