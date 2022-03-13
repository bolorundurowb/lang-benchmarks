#[macro_use]
extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    return "Hello world!";
}

#[rocket::main]
async fn main() {
    let figment = rocket::Config::figment().merge(("port", 7654));

    rocket::custom(figment)
        .mount("/", routes![hello])
        .launch()
        .await;
}
