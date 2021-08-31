#[macro_use] extern crate rocket;

#[get("/info")]
fn info() -> &'static str {
    "The website is working!"
}

#[launch]
fn launch() -> _ {
    rocket::build().mount("/", routes![info])
}
