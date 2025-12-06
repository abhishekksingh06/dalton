#include "dalton/core/location.hpp"

namespace dalton::core {
SourceLocation::SourceLocation(std::string filename, const std::int32_t line,
                               const std::int32_t column)
    : filename(std::move(filename)), line(line), column(column) {}
} // namespace dalton::core
