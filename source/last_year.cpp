#include "last_year.hpp"
#include "http.hpp"
#include <ctime>
#include <pugixml.hpp>
#include <sstream>

std::string last_year::handler() {
  auto maybe_feed =
      http::get_file("https://ninoan.com", "/audio/project-hail-mary.xml");
  if (!maybe_feed.has_value()) {
    return "Failed to fetch feed.";
  }
  std::string feed = maybe_feed.value();

  pugi::xml_document doc;
  doc.load_string(feed.c_str());
  doc.first_element_by_path("rss/channel/title")
      .text()
      .set("Project Last Year");
  auto channel = doc.first_element_by_path("rss/channel");
  auto items = doc.select_nodes("//channel/item");
  for (auto item : items) {
    auto pub_date_node = item.node().first_element_by_path("pubDate");
    std::string pub_date_str = pub_date_node.text().get();
    struct tm timestamp;
    if (strptime(pub_date_str.c_str(), "%a, %d %b %Y %H:%M:%S %z",
                 &timestamp) == NULL) {
      strptime(pub_date_str.c_str(), "%A, %d %b %Y %H:%M:%S %Z", &timestamp);
    }
    timestamp.tm_year += 1;
    time_t updated_pub_time = mktime(&timestamp);
    time_t now = time(NULL);
    char time_str[100];
    strftime(time_str, sizeof(time_str), "%a, %d %b %Y %H:%M:%S %z",
             &timestamp);
    pub_date_node.text().set(time_str);

    if (difftime(now, updated_pub_time) < 0) {
      channel.remove_child(item.node());
    }
  }

  std::stringstream output;
  doc.save(output);
  return output.str();
}
