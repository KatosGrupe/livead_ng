use rocket::serde::json::Json;
use rocket::form::Form;
use crate::user::User;
use rocket::response::Redirect;
use rocket::serde::{Deserialize, Serialize};
use rocket_dyn_templates::Template;

#[get("/", rank = 2)]
pub fn index_unauthorized() -> Redirect {
    Redirect::to(uri!("/login"))
}

#[derive(Clone, Debug, Deserialize, FromForm, Serialize)]
pub struct Entry {
    key: String,
    value: String,
}
#[derive(Deserialize, Serialize)]
struct IndexContext {
    values: Vec<Entry>,
}

#[get("/")]
pub fn index(_user: User) -> Template {
    let context = IndexContext {
        values: [
            Entry {
                key: "test1".to_string(),
                value: "value1".to_string(),
            },
            Entry {
                key: "test2".to_string(),
                value: "value2".to_string(),
            },
        ]
        .iter()
        .cloned()
        .collect(),
    };
    Template::render("index", &context)
}

#[get("/edit/<_key>")]
pub fn edit(_key: &str, _user: User) -> Template {
    let context = Entry {
        key: "test1".to_string(),
        value: "value1".to_string(),
    };
    Template::render("edit", &context)
}

#[post("/edit/<_key>", data = "<entry>")]
pub fn edit_action(entry: Form<Entry>, _key: &str, _user: User) -> Redirect {
    println!("Entry: {:#?}", entry);

    Redirect::to(uri!("/"))
}

#[get("/values/<user>")]
pub fn get_values(user: &str) -> Json<Vec<Entry>> {
    Json(match user {
        "ignas@kata.lt" => [
            Entry {
                key: "test1".to_string(),
                value: "value1".to_string(),
            },
            Entry {
                key: "test2".to_string(),
                value: "value2".to_string(),
            },
        ].to_vec(),
        "livead@user.com" => [
            Entry {
                key: "kadras1".to_string(),
                value: "pirmos kadruotės tekstas".to_string(),
            },
            Entry {
                key: "kadras2".to_string(),
                value: "Šūkis!!".to_string(),
            },
            Entry {
                key: "nuotrauka".to_string(),
                value: "https://www2.shutterstock.com/blog/wp-content/uploads/sites/5/2016/11/EarthAbound_2880x1800_BlogHeader_ImageOnly_2.jpg".to_string(),
            },
        ].to_vec(),
        _ => [].to_vec()
    })
}
