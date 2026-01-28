.PHONY: help build test clean install fmt lint check coverage release

help:
	@echo "Available commands:"
	@echo "  make build     - Build the project"
	@echo "  make test      - Run tests"
	@echo "  make clean     - Clean build artifacts"
	@echo "  make install   - Install the binary"
	@echo "  make fmt       - Format code"
	@echo "  make lint      - Run clippy"
	@echo "  make check     - Run fmt + lint + test"
	@echo "  make coverage  - Generate test coverage"
	@echo "  make release   - Build optimized release"

build:
	cargo build

test:
	cargo test -- --nocapture

clean:
	cargo clean

install:
	cargo install --path .

fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings

check: fmt lint test

coverage:
	cargo tarpaulin --out Html --output-dir coverage

release:
	cargo build --release
	strip target/release/pin-actions

watch:
	cargo watch -x test -x build

doc:
	cargo doc --no-deps --open

bench:
	cargo bench
