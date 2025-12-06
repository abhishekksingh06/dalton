#pragma once

#include "dalton/core/diagnostics.hpp"
#include "dalton/lexer/token.hpp"
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

  core::SourceLocation currentLocation() const;
  bool matchNext(char target);

  std::optional<Token> makeToken(TokenType type,
                                 const std::string &lexeme) const;
  std::optional<Token> lexIdent(const char first);

  std::optional<Token> lexSymbol(char first);

public:
  Lexer(std::string filename, std::string source, core::DiagnosticEngine &diag);

  std::optional<Token> nextToken();
};
} // namespace dalton::lexer
