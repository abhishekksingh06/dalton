#include "dalton/lexer/lexer.hpp"
#include <optional>

namespace dalton::lexer {

Lexer::Lexer(std::string filename, std::string source,
             core::DiagnosticEngine &diag)
    : filename(std::move(filename)), source(std::move(source)), diag(diag) {}

bool Lexer::isAtEnd() const { return index >= source.length(); }

std::optional<char> Lexer::peek() const {
  if (isAtEnd())
    return std::nullopt;
  return source[index];
}

std::optional<char> Lexer::advance() {
  if (isAtEnd())
    return std::nullopt;
  return source[index++];
}

} // namespace dalton::lexer
