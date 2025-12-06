#include "dalton/core/diagnostics.hpp"
#include <utility>

namespace dalton::core {
Diagnostic::Diagnostic(SourceLocation location, const DiagnosticType type,
                       std::string message, std::optional<std::string> help)
    : type(type), location(std::move(location)), message(std::move(message)),
      help(std::move(help)) {}

bool DiagnosticEngine::hasDiagnostics() const { return !diagnostics.empty(); };

const std::vector<Diagnostic> &DiagnosticEngine::getDiagnostics() const {
  return diagnostics;
}

void DiagnosticEngine::error(const SourceLocation &location,
                             std::string message,
                             std::optional<std::string> help) {
  diagnostics.emplace_back(location, DiagnosticType::Error, std::move(message),
                           std::move(help));
}

void DiagnosticEngine::warn(const SourceLocation &location, std::string message,
                            std::optional<std::string> help) {
  diagnostics.emplace_back(location, DiagnosticType::Warn, std::move(message),
                           std::move(help));
}

bool DiagnosticEngine::hasError() const {
  for (const auto &diag : diagnostics) {
    if (diag.type == DiagnosticType::Error)
      return true;
  }
  return false;
}

void DiagnosticEngine::clear() { diagnostics.clear(); }
} // namespace dalton::core
