.PHONY: all build build-fetch build-no-fs release run test test-unit test-all clean format format-check size size-report help

# Default paths for local V4 and V4-front
V4_PATH ?= ../V4
V4FRONT_PATH ?= ../V4-front
V4_USE_V4HAL ?= OFF

# Default target
all: build

# Build with local V4 and V4-front (default)
build:
	@echo "🔨 Building V4-repl with local dependencies..."
	@cmake -B build -DCMAKE_BUILD_TYPE=Debug \
		-DV4_LOCAL_PATH=$(V4_PATH) \
		-DV4FRONT_LOCAL_PATH=$(V4FRONT_PATH) \
		-DV4_USE_V4HAL=$(V4_USE_V4HAL) \
		-DWITH_FILESYSTEM=ON
	@cmake --build build -j

# Build without filesystem support (for embedded)
build-no-fs:
	@echo "🔨 Building V4-repl without filesystem support..."
	@cmake -B build -DCMAKE_BUILD_TYPE=Debug \
		-DV4_LOCAL_PATH=$(V4_PATH) \
		-DV4FRONT_LOCAL_PATH=$(V4FRONT_PATH) \
		-DV4_USE_V4HAL=$(V4_USE_V4HAL) \
		-DWITH_FILESYSTEM=OFF
	@cmake --build build -j

# Build by fetching V4 and V4-front from Git
build-fetch:
	@echo "🔨 Building V4-repl (fetching dependencies from Git)..."
	@cmake -B build -DCMAKE_BUILD_TYPE=Debug \
		-DV4_LOCAL_PATH="" \
		-DV4FRONT_LOCAL_PATH="" \
		-DV4_USE_V4HAL=$(V4_USE_V4HAL) \
		-DWITH_FILESYSTEM=ON
	@cmake --build build -j

# Release build
release:
	@echo "🚀 Building release..."
	@cmake -B build-release -DCMAKE_BUILD_TYPE=Release \
		-DV4_LOCAL_PATH=$(V4_PATH) \
		-DV4FRONT_LOCAL_PATH=$(V4FRONT_PATH) \
		-DV4_USE_V4HAL=$(V4_USE_V4HAL) \
		-DWITH_FILESYSTEM=ON
	@cmake --build build-release -j

# Release build without filesystem
release-no-fs:
	@echo "🚀 Building release without filesystem..."
	@cmake -B build-release -DCMAKE_BUILD_TYPE=Release \
		-DV4_LOCAL_PATH=$(V4_PATH) \
		-DV4FRONT_LOCAL_PATH=$(V4FRONT_PATH) \
		-DV4_USE_V4HAL=$(V4_USE_V4HAL) \
		-DWITH_FILESYSTEM=OFF
	@cmake --build build-release -j

# Run the REPL
run: build
	@echo "🚀 Starting V4 REPL..."
	@./build/v4-repl

# Run release build
run-release: release
	@echo "🚀 Starting V4 REPL (release)..."
	@./build-release/v4-repl

# Run basic smoke tests
test: build
	@bash test_smoke.sh

# Run unit tests (libv4repl)
test-unit: build
	@echo "🧪 Running libv4repl unit tests..."
	@./build/test_libv4repl

# Run all tests (smoke + unit)
test-all: build
	@echo "🧪 Running all tests..."
	@bash test_smoke.sh
	@echo ""
	@./build/test_libv4repl

# Show binary size
size:
	@echo "📊 Binary Size Report"
	@echo "────────────────────────────────────────"
	@if [ -f build/v4-repl ]; then \
		SIZE=$$(ls -lh build/v4-repl | awk '{print $$5}'); \
		echo "Debug build:           $$SIZE"; \
		cp build/v4-repl /tmp/v4-repl-stripped 2>/dev/null && \
		strip /tmp/v4-repl-stripped 2>/dev/null && \
		SIZE_STRIPPED=$$(ls -lh /tmp/v4-repl-stripped | awk '{print $$5}'); \
		echo "Debug (stripped):      $$SIZE_STRIPPED"; \
		rm -f /tmp/v4-repl-stripped; \
	else \
		echo "Debug build not found. Run 'make build' first."; \
	fi
	@echo ""
	@if [ -f build-release/v4-repl ]; then \
		SIZE=$$(ls -lh build-release/v4-repl | awk '{print $$5}'); \
		echo "Release build:         $$SIZE"; \
		cp build-release/v4-repl /tmp/v4-repl-stripped 2>/dev/null && \
		strip /tmp/v4-repl-stripped 2>/dev/null && \
		SIZE_STRIPPED=$$(ls -lh /tmp/v4-repl-stripped | awk '{print $$5}'); \
		echo "Release (stripped):    $$SIZE_STRIPPED"; \
		rm -f /tmp/v4-repl-stripped; \
	else \
		echo "Release build not found. Run 'make release' to create."; \
	fi
	@echo "────────────────────────────────────────"
	@echo "Tip: Run 'make size-report' for detailed analysis"

# Detailed size report
size-report:
	@bash size_report.sh

# Clean
clean:
	@echo "🧹 Cleaning..."
	@rm -rf build build-release build-debug build-asan build-ubsan build-opt build-size _deps

# Apply formatting
format:
	@echo "✨ Formatting C/C++ code..."
	@find src -type f \( -name '*.cpp' -o -name '*.hpp' -o -name '*.h' -o -name '*.c' \) \
		-not -path "*/vendor/*" -exec clang-format -i {} \;
	@echo "✨ Formatting CMake files..."
	@find . -name 'CMakeLists.txt' -o -name '*.cmake' | xargs cmake-format -i 2>/dev/null || echo "⚠️  cmake-format not found, skipping CMake formatting"
	@echo "✅ Formatting complete!"

# Format check
format-check:
	@echo "🔍 Checking C/C++ formatting..."
	@find src -type f \( -name '*.cpp' -o -name '*.hpp' -o -name '*.h' -o -name '*.c' \) \
		-not -path "*/vendor/*" | xargs clang-format --dry-run --Werror || \
		(echo "❌ C/C++ formatting check failed. Run 'make format' to fix." && exit 1)
	@echo "🔍 Checking CMake formatting..."
	@find . -name 'CMakeLists.txt' -o -name '*.cmake' | xargs cmake-format --check 2>/dev/null || \
		echo "⚠️  cmake-format not found, skipping CMake format check"
	@echo "✅ All formatting checks passed!"

# Sanitizer builds
asan:
	@echo "🛡️  Building with AddressSanitizer..."
	@cmake -B build-asan -DCMAKE_BUILD_TYPE=Debug \
		-DV4_LOCAL_PATH=$(V4_PATH) \
		-DV4FRONT_LOCAL_PATH=$(V4FRONT_PATH) \
		-DV4_USE_V4HAL=$(V4_USE_V4HAL) \
		-DWITH_FILESYSTEM=ON \
		-DCMAKE_CXX_FLAGS="-fsanitize=address -fno-omit-frame-pointer -g"
	@cmake --build build-asan -j
	@echo "🧪 Running smoke tests with AddressSanitizer..."
	@echo -e "1 2 +\nbye" | ./build-asan/v4-repl > /dev/null
	@echo "✅ ASan build passed!"

ubsan:
	@echo "🛡️  Building with UndefinedBehaviorSanitizer..."
	@cmake -B build-ubsan -DCMAKE_BUILD_TYPE=Debug \
		-DV4_LOCAL_PATH=$(V4_PATH) \
		-DV4FRONT_LOCAL_PATH=$(V4FRONT_PATH) \
		-DV4_USE_V4HAL=$(V4_USE_V4HAL) \
		-DWITH_FILESYSTEM=ON \
		-DCMAKE_CXX_FLAGS="-fsanitize=undefined -fno-omit-frame-pointer -g"
	@cmake --build build-ubsan -j
	@echo "🧪 Running smoke tests with UndefinedBehaviorSanitizer..."
	@echo -e "1 2 +\nbye" | ./build-ubsan/v4-repl > /dev/null
	@echo "✅ UBSan build passed!"

# Help
help:
	@echo "V4-repl Makefile targets:"
	@echo ""
	@echo "  make / make all      - Build V4-repl (default: uses local V4/V4-front)"
	@echo "  make build           - Build with local dependencies"
	@echo "  make build-no-fs     - Build without filesystem support (embedded)"
	@echo "  make build-fetch     - Build by fetching V4/V4-front from Git"
	@echo "  make release         - Release build"
	@echo "  make release-no-fs   - Release build without filesystem"
	@echo "  make run             - Build and run the REPL interactively"
	@echo "  make run-release     - Run release build"
	@echo "  make test            - Run smoke tests"
	@echo "  make test-unit       - Run libv4repl unit tests"
	@echo "  make test-all        - Run all tests (smoke + unit)"
	@echo "  make size            - Show binary size (quick check)"
	@echo "  make size-report     - Detailed size analysis with recommendations"
	@echo "  make clean           - Remove build directories"
	@echo "  make format          - Format code with clang-format"
	@echo "  make format-check    - Check formatting without modifying files"
	@echo "  make asan            - Build and test with AddressSanitizer"
	@echo "  make ubsan           - Build and test with UndefinedBehaviorSanitizer"
	@echo "  make help            - Show this help message"
	@echo ""
	@echo "Variables:"
	@echo "  V4_PATH              - Path to V4 source (default: ../V4)"
	@echo "  V4FRONT_PATH         - Path to V4-front source (default: ../V4-front)"
	@echo "  V4_USE_V4HAL         - Use V4-hal C++17 CRTP HAL (default: OFF)"
	@echo ""
	@echo "Examples:"
	@echo "  make                                    # Build with local dependencies"
	@echo "  make run                                # Build and run REPL"
	@echo "  make test                               # Run smoke tests"
	@echo "  make size                               # Check binary size"
	@echo "  make size-report                        # Detailed size analysis"
	@echo "  make build V4_PATH=/path/to/V4          # Use custom V4 path"
	@echo "  make build V4_USE_V4HAL=ON              # Build with V4-hal"
	@echo "  make build-no-fs                        # Build for embedded (no filesystem)"
	@echo "  make release                            # Optimized release build"
