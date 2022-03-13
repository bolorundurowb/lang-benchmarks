#[macro_use] extern crate rocket;

#[get("/")]
fn hello() ->  &'static str {
    return "Hello world!";
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![hello])
        .launch()
        .await;
}
