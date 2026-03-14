use std::str::FromStr;
use std::sync::Arc;
use rocket::{Build, Rocket};
use rocket::fs::FileServer;
use rocket::tokio::sync::Mutex;
use rocket_cors::{AllowedOrigins, CorsOptions};

use crate::stencil;
use crate::stencil::adapter::Adapter;
use crate::stencil::endpoints;

pub async fn create_webserver() -> Rocket<Build> {
    // Create Llama model.
    let model = stencil::llama_model::create_model().await;

    // Create the chatbot solution adapter
    let adapter = Adapter::new(model);

    // Setup cors.
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            ["Get", "Post", "Put", "Delete", "Options"]
                .iter()
                .map(|s| FromStr::from_str(s).unwrap())
                .collect(),
        )
        .allow_credentials(true)
        .to_cors()
        .expect("Failed to setup cors configuration.");

    // let state = v2::initialize_state(&model);
    // let state = v3::initialize_state();

    let rocket = rocket::build()
        .configure(rocket::Config::figment().merge(("port", 3000)))
        .mount("/", routes![endpoints::endpoint, endpoints::history])
        .mount("/", FileServer::from("html"))
        .mount("/", rocket_cors::catch_all_options_routes())
        .attach(cors.clone())
        .manage(cors)
        .manage(Arc::new(Mutex::new(adapter)));

    return rocket;
}