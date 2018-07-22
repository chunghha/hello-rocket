#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod routes {
    use rocket_contrib::Json;

    #[derive(Serialize)]
    pub struct Status {
        status: String,
        hello: String,
    }

    #[get("/status")]
    pub fn status() -> Json<Status> {
        Json(Status {
            status: String::from("Ok"),
            hello: String::from("world"),
        })
    }
}

fn main() {
    rocket::ignite().mount("/", routes![routes::status]).launch();
}