#include "calendar.hpp"
#include "last_year.hpp"
#include <crow.h>

int main() {
  crow::SimpleApp app;
  CROW_ROUTE(app, "/last-year")([]() { return last_year::handler(); });
  CROW_ROUTE(app, "/calendar/<string>/<string>")
  ([](std::string title, std::string start_date) {
    return calendar::handler(title, start_date);
  });
  app.port(8080).multithreaded().run();
  return 0;
}
