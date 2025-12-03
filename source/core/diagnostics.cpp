#include "dalton/core/diagnostics.hpp"

namespace dalton::core {

Diagnostic::Diagnostic(SourceLocation location, DiagnosticType type,
                       std::string message, std::optional<std::string> help)
    : location(location), type(type), message(message), help(help) {}

bool DiagnosticEngine::hasDiagnostics() const { return !diagnostics.empty(); };

const std::vector<Diagnostic> &DiagnosticEngine::getDiagnostics() const {
  return diagnostics;
}

void DiagnosticEngine::error(SourceLocation location, std::string message,
                             std::optional<std::string> help) {
  diagnostics.emplace_back(
      Diagnostic(location, DiagnosticType::Error, message, help));
}

void DiagnosticEngine::warn(SourceLocation location, std::string message,
                            std::optional<std::string> help) {
  diagnostics.emplace_back(
      Diagnostic(location, DiagnosticType::Warn, message, help));
}

bool DiagnosticEngine::hasError() const {
  for (const auto &diag : diagnostics)
    if (diag.type == DiagnosticType::Error)
      return true;
  return false;
}

void DiagnosticEngine::clear() { diagnostics.clear(); }
} // namespace dalton::core
