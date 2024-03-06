use homey::drivers::web::actix_mod;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_mod::run_server().await
}