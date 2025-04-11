mod config;
mod domain;
mod adapter;
mod infrastructure;
mod shared;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    dotenv::dotenv().ok();


    Ok(())
}
