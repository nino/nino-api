/// Note that this is the worst code anyone's ever written in the history of the universe. XML is
/// so annoying omg.
extern crate derive_builder;
use chrono::{DateTime, FixedOffset};
use quick_xml::{
    events::{attributes::AttrError, BytesEnd, BytesStart, BytesText, Event},
    Reader, Writer,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error parsing item: {0}")]
    MissingItemAttribute(#[from] ItemBuilderError),

    #[error("Error parsing date: {0}")]
    Date(#[from] chrono::ParseError),

    #[error("XML error: {0}")]
    Xml(#[from] quick_xml::Error),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("Attribute error: {0}")]
    Attribute(#[from] AttrError),

    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
}

impl ItemBuilder {
    fn parse_pub_date(&mut self, date: &str) -> Result<(), Error> {
        let date_attempt1 = DateTime::parse_from_str(&date, "%a, %d %b %Y %H:%M:%S %z");
        let date_attempt2 = DateTime::parse_from_str(&date, "%A, %d %b %Y %H:%M:%S %z");
        self.pub_date(date_attempt1.or(date_attempt2)?);
        Ok(())
    }
}

#[derive(Builder)]
pub struct Item {
    title: String,
    link: String,
    description: String,
    pub_date: DateTime<FixedOffset>,
    enclosure_url: String,
    enclosure_length: String,
    enclosure_type: String,
    guid: String,
    itunes_title: String,
    itunes_author: String,
    itunes_duration: String,
}

impl Item {
    pub fn read(reader: &mut Reader<&[u8]>) -> Result<Self, Error> {
        let mut item_builder = ItemBuilder::default();

        loop {
            match reader.read_event()? {
                Event::Start(ref e) => match e.name().as_ref() {
                    b"title" => {
                        item_builder.title(reader.read_text(e.name())?.into_owned());
                    }
                    b"link" => {
                        item_builder.link(reader.read_text(e.name())?.into_owned());
                    }
                    b"description" => {
                        item_builder.description(reader.read_text(e.name())?.into_owned());
                    }
                    b"pubDate" => {
                        let date = reader.read_text(e.name())?.into_owned();
                        item_builder.parse_pub_date(&date)?;
                    }
                    b"guid" => {
                        item_builder.guid(reader.read_text(e.name())?.into_owned());
                    }
                    b"itunes:title" => {
                        item_builder.itunes_title(reader.read_text(e.name())?.into_owned());
                    }
                    b"itunes:author" => {
                        item_builder.itunes_author(reader.read_text(e.name())?.into_owned());
                    }
                    b"itunes:duration" => {
                        item_builder.itunes_duration(reader.read_text(e.name())?.into_owned());
                    }
                    _ => {}
                },
                Event::Empty(ref e) => match e.name().as_ref() {
                    b"enclosure" => {
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.as_ref() {
                                b"url" => {
                                    item_builder.enclosure_url(attr.unescape_value()?.into_owned());
                                }
                                b"length" => {
                                    item_builder
                                        .enclosure_length(attr.unescape_value()?.into_owned());
                                }
                                b"type" => {
                                    item_builder
                                        .enclosure_type(attr.unescape_value()?.into_owned());
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                },
                Event::Eof => break,
                Event::End(ref e) => match e.name().as_ref() {
                    b"item" => break,
                    _ => (),
                },
                _ => {}
            }
        }

        Ok(item_builder.build()?)
    }

    pub fn advance_one_year(&mut self) {
        self.pub_date += chrono::Duration::days(365);
    }

    pub fn write(&self, writer: &mut Writer<Vec<u8>>) -> Result<(), Error> {
        if self.pub_date > chrono::Utc::now() {
            return Ok(());
        }
        writer.write_event(Event::Start(BytesStart::new("item")))?;

        let mut guid_event = BytesStart::new("guid");
        guid_event.push_attribute(("isPermaLink", "false"));
        writer.write_event(Event::Start(guid_event))?;
        writer.write_event(Event::Text(BytesText::new(&self.guid)))?;
        writer.write_event(Event::End(BytesEnd::new("guid")))?;

        writer.write_event(Event::Start(BytesStart::new("pubDate")))?;
        writer.write_event(Event::Text(BytesText::new(&self.pub_date.to_rfc2822())))?;
        writer.write_event(Event::End(BytesEnd::new("pubDate")))?;

        writer.write_event(Event::Start(BytesStart::new("title")))?;
        writer.write_event(Event::Text(BytesText::new(&self.title)))?;
        writer.write_event(Event::End(BytesEnd::new("title")))?;

        writer.write_event(Event::Start(BytesStart::new("itunes:title")))?;
        writer.write_event(Event::Text(BytesText::new(&self.itunes_title)))?;
        writer.write_event(Event::End(BytesEnd::new("itunes:title")))?;

        writer.write_event(Event::Start(BytesStart::new("itunes:author")))?;
        writer.write_event(Event::Text(BytesText::new(&self.itunes_author)))?;
        writer.write_event(Event::End(BytesEnd::new("itunes:author")))?;

        writer.write_event(Event::Start(BytesStart::new("itunes:duration")))?;
        writer.write_event(Event::Text(BytesText::new(&self.itunes_duration)))?;
        writer.write_event(Event::End(BytesEnd::new("itunes:duration")))?;

        let mut enclosure_event = BytesStart::new("enclosure");
        enclosure_event.push_attribute(("url", self.enclosure_url.as_ref()));
        enclosure_event.push_attribute(("length", self.enclosure_length.as_ref()));
        enclosure_event.push_attribute(("type", self.enclosure_type.as_ref()));
        writer.write_event(Event::Empty(enclosure_event))?;

        writer.write_event(Event::Start(BytesStart::new("description")))?;
        let description = BytesText::new(&self.description);
        let description = description.unescape()?.into_owned();
        writer.write_event(Event::Text(BytesText::new(&description)))?;
        writer.write_event(Event::End(BytesEnd::new("description")))?;

        writer.write_event(Event::Start(BytesStart::new("link")))?;
        writer.write_event(Event::Text(BytesText::new(&self.link)))?;
        writer.write_event(Event::End(BytesEnd::new("link")))?;

        writer.write_event(Event::End(BytesEnd::new("item")))?;
        Ok(())
    }
}

pub fn process_feed(feed: &str) -> Result<String, Error> {
    let mut reader = Reader::from_str(feed);
    let mut writer = Writer::new_with_indent(Vec::new(), b' ', 2);
    loop {
        match reader.read_event()? {
            Event::Start(ref e) => match e.name().as_ref() {
                b"title" => {
                    reader.read_text(e.name())?;
                    writer.write_event(Event::Start(BytesStart::new("title")))?;
                    writer.write_event(Event::Text(BytesText::new("Project Last Year")))?;
                    writer.write_event(Event::End(BytesEnd::new("title")))?;
                }
                b"item" => {
                    let mut item = Item::read(&mut reader)?;
                    item.advance_one_year();
                    item.write(&mut writer)?;
                }
                _ => {
                    writer.write_event(Event::Start(e.clone()))?;
                }
            },
            Event::Eof => {
                writer.write_event(Event::Eof)?;
                break;
            }
            e => {
                writer.write_event(e)?;
            }
        }
    }

    Ok(String::from_utf8(writer.into_inner())?)
}

pub async fn fetch_feed() -> Result<String, Error> {
    Ok(
        reqwest::get("https://ninoan.com/audio/project-hail-mary.xml")
            .await?
            .text()
            .await?,
    )
}
