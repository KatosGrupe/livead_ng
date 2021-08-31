use rocket::form::Form;
use rocket::response::Redirect;
use rocket::serde::{Deserialize, Serialize};
use rocket_dyn_templates::Template;

#[derive(Deserialize, Serialize)]
struct EmptyContext {}

#[get("/login")]
pub fn index() -> Template {
    let context = EmptyContext {};
    Template::render("login", &context)
}

#[derive(Debug, FromForm)]
pub struct Credentials {
    username: String,
    password: String,
}

#[post("/login", data = "<credentials>")]
pub fn login_action(credentials: Form<Credentials>) -> Redirect {
    println!("{:#?}", credentials);

    Redirect::to(uri!("/login"))
}
