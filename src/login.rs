use rocket::form::Form;
use rocket::http::Cookie;
use rocket::http::CookieJar;
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
pub fn login_action(credentials: Form<Credentials>, cookies: &CookieJar<'_>) -> Redirect {
    println!("{:#?}", credentials);
    cookies.add_private(Cookie::new("user_id", "-1"));
    Redirect::to(uri!("/"))
}
