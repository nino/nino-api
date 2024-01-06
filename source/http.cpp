#include <optional>
#define CPPHTTPLIB_OPENSSL_SUPPORT
#include "http.hpp"
#include "deps/httplib.h"
#include <string>

std::optional<std::string> http::get_file(const std::string host,
                                          const std::string path) noexcept {
  try {
    httplib::Client client(host);
    auto res = client.Get(path.c_str());
    return res->body;
  } catch (const std::exception &e) {
    return std::nullopt;
  }
}
