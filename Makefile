BUILD_DIR := build
TARGET := dalton

# Default target
all: configure build

# Configure CMake
configure:
	@mkdir -p $(BUILD_DIR)
	@cmake -S . -B $(BUILD_DIR)

# Build with CMake
build: configure
	@cmake --build $(BUILD_DIR)

# Run the compiled binary
run: build
	@./$(BUILD_DIR)/$(TARGET)

# Clean
clean:
	@rm -rf $(BUILD_DIR)

# Rebuild all
rebuild: clean all

.PHONY: all configure build run clean rebuild

