#include "dalton/core/location.hpp"

namespace dalton::core {
SourceLocation::SourceLocation(std::string filename, std::int32_t line,
                               std::int32_t column)
    : filename(filename), column(column), line(line) {}
} // namespace dalton::core
