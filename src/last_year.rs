#[macro_use]
extern crate derive_builder;
use quick_xml::Reader;

#[derive(Builder)]
struct Item {
    title: String,
    link: String,
    description: String,
    pub_date: String,
    enclosure: String,
    guid: String,
}

pub fn process_feed(feed: &str) -> String {
    let mut reader = Reader::from_str(feed);
    loop {
        match 
    }
}
