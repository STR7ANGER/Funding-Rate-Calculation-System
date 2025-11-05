#!/bin/bash

echo "Monitoring Funding Rate System..."

# Check Docker services
echo "=== Docker Services ==="
docker-compose ps

echo ""
echo "=== Backend Logs (last 50 lines) ==="
docker-compose logs --tail=50 backend

echo ""
echo "=== Database Connection ==="
docker-compose exec -T postgres pg_isready -U funding_user -d funding_rates

echo ""
echo "=== Redis Connection ==="
docker-compose exec -T redis redis-cli ping

echo ""
echo "=== Health Check ==="
curl -s http://localhost:8080/health | jq . || echo "Backend not responding"

echo ""
echo "Monitoring complete!"

