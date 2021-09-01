use rocket::request::FromRequest;
use rocket::Request;

pub struct User {
    pub id: i32,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> rocket::request::Outcome<Self, ()> {
        match request.cookies().get_private("user_id") {
            Some(user_id) => rocket::request::Outcome::Success(User {
                id: user_id.value().parse::<i32>().unwrap(),
            }),
            None => rocket::request::Outcome::Forward(()),
        }
    }
}
