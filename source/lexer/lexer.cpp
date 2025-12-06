#include "dalton/lexer/lexer.hpp"
#include "dalton/lexer/token.hpp"
#include <cctype>
#include <format>
#include <optional>
#include <string>
#include <unordered_map>

namespace dalton::lexer {

static const std::unordered_map<std::string, TokenType> KEYWORDS = {
    {"let", TokenType::KwLet},
    {"mut", TokenType::KwMut},
    {"const", TokenType::KwConst},
    {"fn", TokenType::KwFn},
    {"struct", TokenType::KwStruct},
    {"trait", TokenType::KwTrait},
    {"enum", TokenType::KwEnum},
    {"type", TokenType::KwType},
    {"scope", TokenType::KwScope},
    {"external", TokenType::KwExternal},
    {"pub", TokenType::KwPub},
    {"import", TokenType::KwImport},
    {"where", TokenType::KwWhere},
    {"if", TokenType::KwIf},
    {"then", TokenType::KwThen},
    {"else", TokenType::KwElse},
    {"switch", TokenType::KwSwitch},
    {"case", TokenType::KwCase},
    {"default", TokenType::KwDefault},
    {"for", TokenType::KwFor},
    {"while", TokenType::KwWhile},
    {"loop", TokenType::KwLoop},
    {"break", TokenType::KwBreak},
    {"continue", TokenType::KwContinue},
    {"return", TokenType::KwReturn},
    {"defer", TokenType::KwDefer},
    {"in", TokenType::KwIn},
    {"as", TokenType::KwAs},
    {"nil", TokenType::KwNil},
    {"true", TokenType::KwTrue},
    {"false", TokenType::KwFalse},
    {"try", TokenType::KwTry},
    {"catch", TokenType::KwCatch},
    {"compiletime", TokenType::KwCompiletime},
    {"i8", TokenType::KwI8},
    {"i16", TokenType::KwI16},
    {"i32", TokenType::KwI32},
    {"i64", TokenType::KwI64},
    {"i128", TokenType::KwI128},
    {"i256", TokenType::KwI256},
    {"isize", TokenType::KwIsize},
    {"u8", TokenType::KwU8},
    {"u16", TokenType::KwU16},
    {"u32", TokenType::KwU32},
    {"u64", TokenType::KwU64},
    {"u128", TokenType::KwU128},
    {"u256", TokenType::KwU256},
    {"usize", TokenType::KwUsize},
    {"f32", TokenType::KwF32},
    {"f64", TokenType::KwF64},
    {"char", TokenType::KwChar},
    {"str", TokenType::KwStr},
    {"bool", TokenType::KwBool},
    {"map", TokenType::KwMap},
};

Lexer::Lexer(std::string filename, std::string source,
             core::DiagnosticEngine &diag)
    : filename(std::move(filename)), source(std::move(source)), diag(diag) {
  line = 1;
  column = 1;
}

bool Lexer::isAtEnd() const { return index >= source.length(); }

std::optional<char> Lexer::peek() const {
  if (isAtEnd())
    return std::nullopt;
  return source[index];
}

std::optional<char> Lexer::advance() {
  if (isAtEnd())
    return std::nullopt;

  char c = source[index++];

  if (c == '\n') {
    line++;
    column = 1;
  } else {
    column++;
  }

  return c;
}

core::SourceLocation Lexer::currentLocation() const {
  return {filename, line, column};
}

bool Lexer::matchNext(const char target) {
  if (isAtEnd())
    return false;
  if (peek() == target) {
    advance();
    return true;
  }
  return false;
}

std::optional<Token> Lexer::makeToken(const TokenType type,
                                      const std::string &lexeme) const {
  return Token{type, currentLocation(), lexeme};
}

std::optional<Token> Lexer::lexIdent(const char first) {
  if (!(std::isalpha(static_cast<unsigned char>(first)) || first == '_'))
    return std::nullopt;
  std::string text{first};
  while (true) {
    auto c = peek();
    if (!c.has_value())
      break;
    auto ch = static_cast<unsigned char>(*c);
    if (!(std::isalnum(ch) || *c == '_'))
      break;
    text.push_back(*c);
    advance();
  }
  auto it = KEYWORDS.find(text);
  if (it != KEYWORDS.end()) {
    return makeToken(it->second, text);
  }
  return makeToken(TokenType::Identifier, text);
}

std::optional<Token> Lexer::lexSymbol(const char first) {
  switch (first) {
  case '+':
    if (matchNext('='))
      return makeToken(TokenType::PlusEqual, "+=");
    return makeToken(TokenType::Plus, "+");
  case '-':
    if (matchNext('='))
      return makeToken(TokenType::MinusEqual, "-=");
    if (matchNext('>'))
      return makeToken(TokenType::Arrow, "->");
    return makeToken(TokenType::Minus, "-");
  case '*':
    if (matchNext('='))
      return makeToken(TokenType::StarEqual, "*=");
    return makeToken(TokenType::Star, "*");
  case '/':
    if (matchNext('='))
      return makeToken(TokenType::SlashEqual, "/=");
    return makeToken(TokenType::Slash, "/");
  case '%':
    if (matchNext('='))
      return makeToken(TokenType::PercentEqual, "%=");
    return makeToken(TokenType::Percent, "%");
  case '^':
    if (matchNext('='))
      return makeToken(TokenType::CaretEqual, "^=");
    return makeToken(TokenType::Caret, "^");
  case '!':
    if (matchNext('='))
      return makeToken(TokenType::BangEqual, "!=");
    return makeToken(TokenType::Bang, "!");
  case ',':
    return makeToken(TokenType::Comma, ",");
  case ':':
    return makeToken(TokenType::Colon, ":");
  case ';':
    return makeToken(TokenType::Semicolon, ";");
  case '=':
    if (matchNext('='))
      return makeToken(TokenType::EqualEqual, "==");
    return makeToken(TokenType::Equal, "=");
  case '.':
    if (matchNext('.')) {
      if (matchNext('.'))
        return makeToken(TokenType::DotDotDot, "...");
      return makeToken(TokenType::DotDot, "..");
    }
    return makeToken(TokenType::Dot, ".");
  case '?':
    return makeToken(TokenType::Question, "?");
  case '|':
    if (matchNext('|'))
      return makeToken(TokenType::OrOr, "||");
    return makeToken(TokenType::Pipe, "|");
  case '&':
    if (matchNext('&'))
      return makeToken(TokenType::AndAnd, "&&");
    return makeToken(TokenType::Amp, "&");
  case '~':
    return makeToken(TokenType::Tilde, "~");
  case '>':
    if (matchNext('='))
      return makeToken(TokenType::GreaterEqual, ">=");
    if (matchNext('>'))
      return makeToken(TokenType::ShiftRight, ">>");
    return makeToken(TokenType::Greater, ">");
  case '<':
    if (matchNext('='))
      return makeToken(TokenType::LessEqual, "<=");
    if (matchNext('<'))
      return makeToken(TokenType::ShiftLeft, "<<");
    return makeToken(TokenType::Less, "<");
  case '(':
    return makeToken(TokenType::LeftParen, "(");
  case ')':
    return makeToken(TokenType::RightParen, ")");
  case '[':
    return makeToken(TokenType::LeftBracket, "[");
  case ']':
    return makeToken(TokenType::RightBracket, "]");
  case '{':
    return makeToken(TokenType::LeftBrace, "{");
  case '}':
    return makeToken(TokenType::RightBrace, "}");
  default:
    return std::nullopt;
  }
}

std::optional<Token> Lexer::nextToken() {
  while (!isAtEnd()) {
    char c = *peek();
    if (std::isspace(c)) {
      advance();
      continue;
    }
    advance();
    if (auto token = lexSymbol(c))
      return token;
    if (auto token = lexIdent(c))
      return token;
    diag.error(currentLocation(), std::format("unknown token: '{}'", c),
               "check for typos or unsupported characters");
  }
  return makeToken(TokenType::Eof, "");
}
} // namespace dalton::lexer
