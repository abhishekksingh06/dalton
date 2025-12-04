#include "dalton/core/diagnostics.hpp"
#include <utility>

namespace dalton::core {

Diagnostic::Diagnostic(SourceLocation location, DiagnosticType type,
                       std::string message, std::optional<std::string> help)
    : location(location), type(type), message(std::move(message)),
      help(std::move(help)) {}

bool DiagnosticEngine::hasDiagnostics() const { return !diagnostics.empty(); };

const std::vector<Diagnostic> &DiagnosticEngine::getDiagnostics() const {
  return diagnostics;
}

void DiagnosticEngine::error(SourceLocation location, std::string message,
                             std::optional<std::string> help) {
  diagnostics.emplace_back(Diagnostic(location, DiagnosticType::Error,
                                      std::move(message), std::move(help)));
}

void DiagnosticEngine::warn(SourceLocation location, std::string message,
                            std::optional<std::string> help) {
  diagnostics.emplace_back(Diagnostic(location, DiagnosticType::Warn,
                                      std::move(message), std::move(help)));
}

bool DiagnosticEngine::hasError() const {
  for (const auto &diag : diagnostics)
    if (diag.type == DiagnosticType::Error)
      return true;
  return false;
}

void DiagnosticEngine::clear() { diagnostics.clear(); }
} // namespace dalton::core
