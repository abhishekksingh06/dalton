#include "location.hpp"
#include <optional>
#include <string>
#include <vector>
namespace dalton::core {

enum class Severity { Error, Warning };

struct Diagnostic {
  Severity severity;
  std::string message;
  dalton::core::SourceLocation location;
  std::optional<std::string> help = std::nullopt;
};

class DiagnosticEngine {
private:
  std::vector<Diagnostic> diagnostic;

public:
  void error(SourceLocation location, std::string message,
             std::optional<std::string> help = std::nullopt);

  void warn(SourceLocation location, std::string message,
            std::optional<std::string> help = std::nullopt);

  const std::vector<Diagnostic> &all() { return this->diagnostic; };
  bool hasError() { return !this->diagnostic.empty(); };
};
}; // namespace dalton::core
