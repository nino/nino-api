#include "last_year.hpp"
#include <crow.h>

int main() {
  crow::SimpleApp app;
  CROW_ROUTE(app, "/last-year")([]() { return last_year::handler(); });
  app.port(8080).multithreaded().run();
  return 0;
}
