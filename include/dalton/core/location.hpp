#pragma once

#include <cstdint>
#include <string>

namespace dalton::core {

/**
 * @brief Represents a source location in the input file.
 *
 * The SourceLocation structure is used throughout the Dalton compiler frontend
 * (particularly in the lexer, parser, and MLIR code generation phases) to
 * track the exact position of tokens, AST nodes, and diagnostics in the
 * original source code.
 *
 * This information is essential for:
 * - Precise error and warning reporting with file/line/column information
 * - Source-to-MLIR value mapping and debugging
 * - IDE integration and source-level tooling
 *
 * @note All fields are public for performance and convenience in the frontend,
 *       as Location objects are created and copied very frequently during
 * lexing and parsing.
 *
 * @see Lexer, DiagnosticEngine, MLIRLocation
 */
struct SourceLocation {
  /// Name/path of the source file (empty string if from a virtual buffer or
  /// stdin)
  std::string filename;

  /// One-based line number in the source file (1 = first line)
  std::int32_t line;

  /// One-based column number within the line (1 = first character)
  std::int32_t column;

  SourceLocation(std::string filename, std::int32_t line, std::int32_t column);
};

} // namespace dalton::core
