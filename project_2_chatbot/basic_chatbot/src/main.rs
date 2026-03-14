#[macro_use]
extern crate rocket;

mod solution;
mod stencil;

// This is the main function (kindof)
// The actual main function is generated automatically by the
// `#[rocket::launch]` macro.
#[rocket::launch]
async fn rocket() -> _ {
    // Create webserver
    let webserver = stencil::webserver::create_webserver().await;

    // Give the webserver back so rocket::launch launches it.
    webserver
}