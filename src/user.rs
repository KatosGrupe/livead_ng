use rocket::request::FromRequest;
use rocket::Request;

struct User {}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> rocket::request::Outcome<Self, ()> {
        let result = request.cookies().get_private("user_id");
        println!("result: {:#?}", result);

        rocket::request::Outcome::Success(User {})
    }
}
