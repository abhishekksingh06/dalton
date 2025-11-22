#include "../core/location.hpp"
#include <string>
namespace dalton::lexer {
enum TokenType {
  Identifier,
  Number,
  Plus,
  Minus,
  EndOfFile,
};
struct Token {
  TokenType type;
  std::string lexeme;
  dalton::core::SourceLocation location;
};
}; // namespace dalton::lexer
