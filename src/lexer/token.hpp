#include <cstdint>
#include <string>
namespace dalton::lexer {
enum TokenType {
  Identifier,
  Number,
  Plus,
  Minus,
  EndOfFile,
};

struct SourceLocation {
  uint32_t line;
  uint32_t column;
};

struct Token {
  TokenType type;
  std::string lexeme;
  SourceLocation location;
};
}; // namespace dalton::lexer
