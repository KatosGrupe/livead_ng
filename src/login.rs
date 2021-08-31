use rocket_dyn_templates::Template;
use rocket::serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
struct EmptyContext{

}

#[get("/login")]
pub fn index() -> Template {
    let context = EmptyContext{};
    Template::render("login", &context)
}
