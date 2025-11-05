# Funding Rate Calculation System (Scaffold)

This repository is scaffolded from `ass.md` with a runnable backend, SQL migrations, Docker stack (Postgres, Redis, backend), and an Anchor program skeleton.

## Quick Start

1. Copy env
   - Create `.env` based on `.env.example`.

2. Start infrastructure
   - `docker-compose up -d`

3. Run backend locally
   - `cargo run --release --bin funding-rate-backend`
   - Health check: `curl http://localhost:8080/health` → `ok`

4. Apply migrations
   - Containers initialize DB from `./migrations` on first run
   - Or run locally via `sqlx migrate run` with `DATABASE_URL`

## Structure
- Workspace `Cargo.toml`
- `backend/` Axum service with `/health`
- `migrations/` SQL 001-005
- `programs/funding-rate/` Anchor program skeleton
- `config/development.toml`
- `docker-compose.yml`, backend `Dockerfile`
- `Makefile`



