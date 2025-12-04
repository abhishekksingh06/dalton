#pragma once

#include "dalton/core/diagnostics.hpp"
#include "dalton/lexer/token.hpp"
#include <cstdint>
#include <optional>
#include <string>

namespace dalton::lexer {

class Lexer {
private:
  std::int32_t line = 1;
  std::int32_t column = 0;
  std::int32_t index = 0;
  std::string filename;
  std::string source;
  core::DiagnosticEngine &diag;

  bool isAtEnd() const;
  std::optional<char> peek() const;
  std::optional<char> advance();

public:
  Lexer(std::string filename, std::string source, core::DiagnosticEngine &diag);

  Token nextToken();
};

} // namespace dalton::lexer
