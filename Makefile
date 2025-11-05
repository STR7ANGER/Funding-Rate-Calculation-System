.PHONY: help setup build test clean deploy docker-up docker-down migrate run-backend logs

help:
	@echo "Available commands:"
	@echo "  make setup       - Install dependencies and initialize project"
	@echo "  make build       - Build all components"
	@echo "  make test        - Run all tests"
	@echo "  make docker-up   - Start Docker containers"
	@echo "  make docker-down - Stop Docker containers"
	@echo "  make deploy      - Deploy Anchor program"
	@echo "  make clean       - Clean build artifacts"
	@echo "  make migrate     - Run SQL migrations"
	@echo "  make run-backend - Run backend locally"
	@echo "  make logs        - Tail backend logs"

setup:
	@echo "Setting up project..."
	rustup update stable || true
	cargo install sqlx-cli --no-default-features --features postgres || true
	@echo "Create a .env file based on .env.example"
	@echo "Done."

build:
	@echo "Building Anchor program..."
	@echo "(Run 'anchor build' manually if Anchor is installed)"
	@echo "Building backend..."
	cargo build --release

test:
	@echo "Running backend tests..."
	cargo test --workspace

docker-up:
	docker-compose up -d
	@echo "Waiting for services to be healthy..."
	sleep 10
	docker-compose ps

docker-down:
	docker-compose down -v

migrate:
	@echo "Apply SQL migrations via container init or sqlx as needed"
	@echo "If using sqlx locally: set DATABASE_URL and run 'sqlx migrate run'"

deploy:
	@echo "Run 'anchor deploy' when Anchor is configured"

clean:
	cargo clean
	@echo "If Anchor is present, run 'anchor clean'"

run-backend:
	cargo run --release --bin funding-rate-backend

logs:
	docker-compose logs -f backend

