# Local Setup Guide

## Prerequisites

1. Install Rust 1.83+
2. Install Solana CLI 2.0.20
3. Install Anchor 0.30.1
4. Install Docker and Docker Compose
5. Install PostgreSQL client tools

## Quick Start

1. Clone the repository
2. Run setup script:
   ```bash
   ./scripts/setup.sh
   ```

3. Configure environment:
   ```bash
   cp .env.example .env
   # Edit .env with your settings
   ```

4. Start Docker services:
   ```bash
   make docker-up
   ```

5. Apply database migrations:
   ```bash
   make migrate
   ```

6. Build the project:
   ```bash
   make build
   ```

7. Run the backend:
   ```bash
   make run-backend
   ```

## Verify Installation

1. Check health endpoint:
   ```bash
   curl http://localhost:8080/health
   ```

2. Check database:
   ```bash
   docker-compose exec postgres psql -U funding_user -d funding_rates
   ```

3. Check Redis:
   ```bash
   docker-compose exec redis redis-cli ping
   ```

## Troubleshooting

See main README.md for troubleshooting tips.

