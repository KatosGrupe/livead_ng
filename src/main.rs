#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

mod login;
mod text;
mod user;

#[get("/info")]
fn info() -> &'static str {
    "The website is working!"
}

#[launch]
fn launch() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount(
            "/",
            routes![
                info,
                login::index,
                login::login_action,
                text::edit,
                text::edit_action,
                text::get_values,
                text::index,
                text::index_unauthorized,
            ],
        )
        .mount("/public", FileServer::from("./static"))
}
