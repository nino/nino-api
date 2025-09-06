#pragma once

#include <expected>
#include <string>

namespace http {

struct HttpError {
    enum Type {
        NetworkFailure,
        InvalidResponse,
        ConnectionTimeout,
        UnknownError,
    } type;
    std::string message;

    HttpError(Type t, std::string msg = "")
        : type(t), message(std::move(msg)) {}
};

std::expected<std::string, HttpError>
get_file(const std::string& host, const std::string& path) noexcept;

} // namespace http
