mod last_year;

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

    last_year::process_feed(&feed)
    // Select rss->channel
    // Process the feed such that every rss->channel->item has its `pubDate` set to the next year,
    // and any items whose `pubDate` are now in the future are removed.

    // Return the new XML as a string.
    // todo!()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, last_year_handler])
}
