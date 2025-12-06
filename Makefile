# =============================================================================
# Dalton project Makefile - makes building, testing, formatting and docs easy
# =============================================================================

.PHONY: all standalone test run test-fast format format-fix fix-format docs clean

# Default target
all: standalone

# ----------------------------------------------------------------------
# Standalone executable (what you normally want)
# ----------------------------------------------------------------------
standalone:
	cmake -S standalone -B build/standalone
	cmake --build build/standalone

run: standalone
	./build/standalone/Dalton

# ----------------------------------------------------------------------
# Code formatting (using clang-format target you already have)
# ----------------------------------------------------------------------
format:
	cmake -S test -B build/test  # ensures the format target exists
	cmake --build build/test --target format

format-fix: fix-format
fix-format:
	cmake -S test -B build/test
	cmake --build build/test --target fix-format

# ----------------------------------------------------------------------
# Documentation
# ----------------------------------------------------------------------
docs:
	cmake -S documentation -B build/doc
	cmake --build build/doc --target GenerateDocs
	@echo "=================================================================="
	@echo "Documentation built! Opening in your default browser..."
	@echo "=================================================================="
	# Linux
	([ -f /etc/debian_version ] || [ "`uname`" = "Linux" ]) && xdg-open build/doc/doxygen/html/index.html || true
	# macOS
	[ "`uname`" = "Darwin" ] && open build/doc/doxygen/html/index.html || true
	# Windows (git-bash/msys/cygwin)
	[ -n "$$WSL_DISTRO_NAME" ] && cmd.exe /c start build/doc/doxygen/html/index.html || true

# ----------------------------------------------------------------------
# Cleanup
# ----------------------------------------------------------------------
clean:
	rm -rf build/

# ----------------------------------------------------------------------
# Help
# ----------------------------------------------------------------------
help:
	@echo "Available targets:"
	@echo "  all          - build standalone executable (default)"
	@echo "  standalone   - build the main Dalton executable"
	@echo "  run          - build and run the Dalton executable"
	@echo "  test         - configure + build + run tests (full)"
	@echo "  test-fast    - just rebuild and run tests (quick iteration)"
	@echo "  format       - show what clang-format would change"
	@echo "  format-fix   - apply clang-format fixes"
	@echo "  fix-format   - alias for format-fix"
	@echo "  docs         - generate and open Doxygen documentation"
	@echo "  clean        - remove entire build/ directory"
	@echo "  help         - show this help"
