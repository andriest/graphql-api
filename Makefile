.PHONY: build build-release run test test-watch lint fmt fmt-check clean \
        db-setup db-migrate db-rollback db-reset help

# ─── Build ────────────────────────────────────────────────────────────────────

build:
	cargo build

build-release:
	cargo build --release

# ─── Run ──────────────────────────────────────────────────────────────────────

run:
	cargo run

run-release:
	cargo run --release

# ─── Test ─────────────────────────────────────────────────────────────────────

test:
	cargo test

test-verbose:
	cargo test -- --nocapture

test-watch:
	cargo watch -x test

# ─── Lint & Format ────────────────────────────────────────────────────────────

lint:
	cargo clippy --all-targets --all-features --locked -- -D warnings

fmt:
	cargo fmt

fmt-check:
	cargo fmt -- --check

check:
	cargo check

# ─── Database ─────────────────────────────────────────────────────────────────

db-setup:
	diesel setup

db-migrate:
	diesel migration run

db-rollback:
	diesel migration revert

db-reset:
	diesel database reset

# ─── Clean ────────────────────────────────────────────────────────────────────

clean:
	cargo clean

# ─── Help ─────────────────────────────────────────────────────────────────────

help:
	@echo "Usage: make <target>"
	@echo ""
	@echo "  Build:"
	@echo "    build           Build debug binary"
	@echo "    build-release   Build optimized release binary"
	@echo ""
	@echo "  Run:"
	@echo "    run             Run in debug mode"
	@echo "    run-release     Run in release mode"
	@echo ""
	@echo "  Test:"
	@echo "    test            Run all unit tests"
	@echo "    test-verbose    Run tests with stdout output"
	@echo "    test-watch      Watch and re-run tests on file change (requires cargo-watch)"
	@echo ""
	@echo "  Lint & Format:"
	@echo "    lint            Run clippy with strict warnings"
	@echo "    fmt             Auto-format source code"
	@echo "    fmt-check       Check formatting without modifying files"
	@echo "    check           Check compilation without building"
	@echo ""
	@echo "  Database:"
	@echo "    db-setup        Initialize database (diesel setup)"
	@echo "    db-migrate      Run pending migrations"
	@echo "    db-rollback     Revert last migration"
	@echo "    db-reset        Drop and re-run all migrations"
	@echo ""
	@echo "  Other:"
	@echo "    clean           Remove build artifacts"
	@echo "    help            Show this help message"
