#include "calendar.hpp"
#include <ctime>
#include <optional>
#include <sstream>

const std::vector<std::string> date_formats = {"%F"};

std::optional<struct tm> parse_date(std::string text) {
  struct tm timestamp;
  for (auto format : date_formats) {
    if (strptime(text.c_str(), format.c_str(), &timestamp) != NULL) {
      return timestamp;
    }
  }
  return std::nullopt;
}

std::string calendar::handler(std::string title, std::string start_date) {
  std::optional<struct tm> parsed_date = parse_date(start_date);
  if (!parsed_date.has_value()) {
    return "Unable to parse start date";
  }

  char formatted_date[100];
  std::strftime(formatted_date, 100, "%Y%m%d", &parsed_date.value());

  std::stringstream output;

  output << "BEGIN:VCALENDAR\n";
  output << "VERSION:2.0\n";
  output << "PRODID:-//Ninoan//Calendar 1.0//EN\n";
  output << "METHOD:PUBLISH\n";

  output << "BEGIN:VEVENT\n";
  output << "SUMMARY:" << title << '\n';
  output << "DTSTART;VALUE=DATE:" << formatted_date << "\n";
  output << "DTEND;VALUE=DATE:" << formatted_date << "\n";
  output << "LOCATION:United Kingdom\n";
  output << "DESCRIPTION:This is the day, ";
  output << title << '\n';
  output << "UID:stsulhstulhstrulhts@ninoan.com\n";
  output << "DTSTAMP:20231015T092026Z\n";
  output << "STATUS:CONFIRMED\n";
  output << "TRANSP:TRANSPARENT\n";
  output << "SEQUENCE:0\n";
  output << "END:VEVENT\n";

  output << "END:VCALENDAR";
  return output.str();
}
