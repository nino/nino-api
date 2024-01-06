#pragma once

#include <optional>
#include <string>

namespace http {

std::optional<std::string> get_file(const std::string host,
                                    const std::string path) noexcept;

} // namespace http
