#include "dalton/core/diagnostics.hpp"
#include "dalton/lexer/lexer.hpp"
#include "dalton/lexer/token.hpp"
#include <iostream>

int main() {
  dalton::core::DiagnosticEngine diag;

  std::string code = "let x = 42; \nfn test() { return x }";

  dalton::lexer::Lexer lexer("main.dt", code, diag);

  while (true) {
    auto tok = lexer.nextToken();
    if (!tok || tok->type == dalton::lexer::TokenType::Eof)
      break;

    std::cout << tok->lexeme << "\n";
  }
}
