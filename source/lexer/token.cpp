#include "dalton/lexer/token.hpp"
#include "dalton/core/location.hpp"

namespace dalton::lexer {

Token::Token(TokenType type, core::SourceLocation location, std::string value)
    : type(type), location(location), value(std::move(value)) {}

} // namespace dalton::lexer
