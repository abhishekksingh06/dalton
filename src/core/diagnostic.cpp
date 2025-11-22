#include "diagnostics.hpp"

void dalton::core::DiagnosticEngine::error(SourceLocation location,
                                           std::string message,
                                           std::optional<std::string> help) {
  this->diagnostic.push_back(
      Diagnostic{Severity::Error, message, location, help});
}

void dalton::core::DiagnosticEngine::warn(SourceLocation location,
                                          std::string message,
                                          std::optional<std::string> help) {
  this->diagnostic.push_back(
      Diagnostic{Severity::Warning, message, location, help});
}
