#!/bin/sh
set -e

echo "🗃️ Running database migrations..."
sqlx migrate run

echo "🚀 Starting Auth server..."
exec /usr/local/bin/auth

