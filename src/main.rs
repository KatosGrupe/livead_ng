#[macro_use]
extern crate rocket;

use rocket_dyn_templates::Template;

mod login;
mod user;

#[get("/info")]
fn info() -> &'static str {
    "The website is working!"
}

#[launch]
fn launch() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![info, login::index, login::login_action])
}
