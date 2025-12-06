#include <utility>

#include "dalton/core/location.hpp"
#include "dalton/lexer/token.hpp"

namespace dalton::lexer {
Token::Token(const TokenType type, core::SourceLocation location,
             std::string lexeme)
    : type(type), location(std::move(location)), lexeme(std::move(lexeme)) {}
} // namespace dalton::lexer
