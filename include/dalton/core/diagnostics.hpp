#pragma once
#include "location.hpp"
#include <optional>
#include <string>
#include <vector>

namespace dalton::core {

/**
 * @brief Severity level of a diagnostic message.
 */
enum class DiagnosticType {
  Warn,  ///< Warning — non-fatal issue, compilation may continue
  Error, ///< Error — fatal issue, compilation will fail
};

/**
 * @brief A single diagnostic message emitted during compilation.
 *
 * Represents a warning or error with precise source location, message,
 * and optional help text. Used by the lexer, parser, semantic analysis,
 * and MLIR code generation phases to report issues to the user.
 *
 * @note Intentionally lightweight and copyable — diagnostics are collected
 *       in bulk and often stored/sorted/shown after compilation.
 */
struct Diagnostic {
  /// Type/severity of the diagnostic
  DiagnosticType type;

  /// Source location where the diagnostic originates
  SourceLocation location;

  /// Primary diagnostic message (should be concise and clear)
  std::string message;

  /// Optional additional help text, hint, or suggested fix
  std::optional<std::string> help;

  /**
   * @brief Constructs a diagnostic.
   *
   * @param location Source location of the issue
   * @param type     Severity (Warn or Error)
   * @param message  Main diagnostic text
   * @param help     Optional longer explanation or fix suggestion
   */
  Diagnostic(SourceLocation location, DiagnosticType type, std::string message,
             std::optional<std::string> help = std::nullopt);
};

/**
 * @brief Central facility for collecting and querying diagnostics.
 *
 * The DiagnosticEngine accumulates all warnings and errors produced during
 * lexing, parsing, type checking, and MLIR code generation. It is typically
 * owned by the CompilerInstance or Context and consulted at the end of
 * compilation to determine success/failure and to pretty-print diagnostics.
 *
 * @note Thread-safety is not required — diagnostics are emitted sequentially
 *       during single-threaded compilation passes.
 */
class DiagnosticEngine {
private:
  std::vector<Diagnostic> diagnostics;

public:
  /**
   * @brief Returns true if any diagnostics (warnings or errors) have been
   * emitted.
   */
  [[nodiscard]] bool hasDiagnostics() const;

  /**
   * @brief Returns a const reference to all collected diagnostics.
   *
   * Suitable for diagnostic consumers (CLI printer, IDE integration, etc.).
   */
  [[nodiscard]] const std::vector<Diagnostic> &getDiagnostics() const;

  /**
   * @brief Emit an error diagnostic.
   *
   * @param location Source location of the error
   * @param message  Error message
   * @param help     Optional help text or fixit hint
   */
  void error(const SourceLocation &location, std::string message,
             std::optional<std::string> help = std::nullopt);

  /**
   * @brief Emit a warning diagnostic.
   *
   * @param location Source location of the warning
   * @param message  Warning message
   * @param help     Optional help or suggestion
   */
  void warn(const SourceLocation &location, std::string message,
            std::optional<std::string> help = std::nullopt);

  /**
   * @brief Clear all collected diagnostics.
   *
   * Useful for testing or when reusing the engine across multiple compilations.
   */
  void clear();

  /**
   * @brief Returns true if any error diagnostics were emitted.
   *
   * Convenience function often used to decide compilation success.
   */
  [[nodiscard]] bool hasError() const;
};

} // namespace dalton::core
