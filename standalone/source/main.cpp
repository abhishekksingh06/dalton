#include "dalton/core/diagnostics.hpp"
#include <print>

int main(int argc, char *argv[]) {
  dalton::core::DiagnosticEngine engine;
  engine.error(dalton::core::SourceLocation("main.cpp", 10, 1), "message test");
  std::println("D: {}", engine.hasDiagnostics());
  return 0;
}
