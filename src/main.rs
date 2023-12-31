mod last_year;

#[macro_use]
extern crate derive_builder;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/last-year")]
async fn last_year_handler() -> String {
    // Fetch the feed from https://ninoan.com/audio/project-hail-mary.xml
    let feed = reqwest::get("https://ninoan.com/audio/project-hail-mary.xml")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    match last_year::process_feed(&feed) {
        Ok(feed) => feed,
        Err(e) => format!("Error: {:?}", e),
    }

}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, last_year_handler])
}
