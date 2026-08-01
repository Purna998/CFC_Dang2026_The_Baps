.PHONY: help install dev build test clean docker-up docker-down migrate format lint check validate

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'

install: ## Install dependencies
	@echo "Installing Rust toolchain..."
	rustup update
	rustup component add rustfmt clippy
	@echo "Installing cargo tools..."
	cargo install sqlx-cli --no-default-features --features postgres
	cargo install cargo-watch
	cargo install cargo-audit
	@echo "✓ Dependencies installed"

setup: ## Setup environment
	@echo "Setting up environment..."
	cp -n .env.example .env || true
	@echo "✓ .env file created (edit with your configuration)"
	@echo "Starting Docker services..."
	docker-compose up -d postgres redis
	@echo "Waiting for database..."
	sleep 5
	@echo "Running migrations..."
	sqlx migrate run
	@echo "✓ Setup complete"

dev: ## Run API gateway in development mode with auto-reload
	cargo watch -x 'run --bin api-gateway'

build: ## Build all services
	cargo build --workspace

build-release: ## Build release binaries
	cargo build --release --workspace

test: ## Run all tests
	cargo test --workspace

test-integration: ## Run integration tests
	cargo test --workspace --features integration-tests

clean: ## Clean build artifacts
	cargo clean
	rm -rf target/

docker-up: ## Start all Docker services
	docker-compose up -d

docker-up-tools: ## Start all services including management tools
	docker-compose --profile tools up -d

docker-down: ## Stop all Docker services
	docker-compose down

docker-logs: ## Show Docker logs
	docker-compose logs -f

migrate: ## Run database migrations
	sqlx migrate run

migrate-revert: ## Revert last migration
	sqlx migrate revert

migrate-create: ## Create new migration (usage: make migrate-create NAME=migration_name)
	sqlx migrate add $(NAME)

format: ## Format code
	cargo fmt --all

format-check: ## Check code formatting
	cargo fmt --all -- --check

lint: ## Run clippy lints
	cargo clippy --workspace --all-targets -- -D warnings

check: ## Fast compilation check
	cargo check --workspace

validate: ## Run full validation (format, lint, test, build)
	@echo "Running validation..."
	@bash validate.sh

audit: ## Check for security vulnerabilities
	cargo audit

run-gateway: ## Run API gateway
	cargo run --bin api-gateway

run-gateway-release: ## Run API gateway (release mode)
	cargo run --release --bin api-gateway

db-reset: ## Reset database (WARNING: destroys all data)
	@echo "⚠️  WARNING: This will delete all data!"
	@read -p "Are you sure? [y/N] " -n 1 -r; \
	echo; \
	if [[ $$REPLY =~ ^[Yy]$$ ]]; then \
		docker-compose down -v; \
		docker-compose up -d postgres redis; \
		sleep 5; \
		sqlx migrate run; \
		echo "✓ Database reset complete"; \
	fi

db-shell: ## Open PostgreSQL shell
	docker exec -it eemp-postgres psql -U eemp -d eemp_dev

redis-cli: ## Open Redis CLI
	docker exec -it eemp-redis redis-cli

solana-devnet: ## Start local Solana devnet
	docker-compose up -d solana

solana-airdrop: ## Airdrop 10 SOL to local wallet
	solana airdrop 10 --url http://localhost:8899

logs-gateway: ## Show API gateway logs
	tail -f target/debug/api-gateway.log 2>/dev/null || echo "No logs yet"

benchmark: ## Run benchmarks
	cargo bench --workspace

doc: ## Generate and open documentation
	cargo doc --workspace --no-deps --open

coverage: ## Generate test coverage report (requires cargo-tarpaulin)
	cargo tarpaulin --workspace --out Html --output-dir coverage/

stats: ## Show code statistics
	@echo "Lines of code:"
	@find . -name '*.rs' -not -path "./target/*" | xargs wc -l | tail -1
	@echo ""
	@echo "Number of services:"
	@ls -d services/*/ 2>/dev/null | wc -l
	@echo ""
	@echo "Number of migrations:"
	@ls migrations/*.sql 2>/dev/null | wc -l

watch-test: ## Run tests on file changes
	cargo watch -x test

watch-check: ## Run check on file changes
	cargo watch -x check

ci: format-check lint test build ## Run CI checks locally

all: clean install setup build test ## Full setup and build

.DEFAULT_GOAL := help
