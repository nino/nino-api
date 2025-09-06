#define CPPHTTPLIB_OPENSSL_SUPPORT
#include "http.hpp"
#include "deps/httplib.h"
#include <expected>
#include <string>

namespace http {

std::expected<std::string, HttpError>
get_file(const std::string& host, const std::string& path) noexcept {
    try {
        httplib::Client client(host);
        httplib::Result res = client.Get(path.c_str());

        if (!res) {
            return std::unexpected(HttpError{
                HttpError::NetworkFailure,
                "Failed to connect to " + host,
            });
        }

        if (res->status != 200) {
            return std::unexpected(HttpError{
                HttpError::InvalidResponse,
                "HTTP status: " + std::to_string(res->status),
            });
        }

        return res->body;
    } catch (const std::exception& e) {
        return std::unexpected(HttpError{
            HttpError::UnknownError,
            e.what(),
        });
    } catch (...) {
        return std::unexpected(HttpError{
            HttpError::UnknownError,
            "Unknown error",
        });
    }
}

} // namespace http
