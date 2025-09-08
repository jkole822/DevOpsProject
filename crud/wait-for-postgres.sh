#!/bin/sh
set -e

host="$1"
shift
cmd="$@"

until pg_isready -h "$host" -p 5432 -U "postgres"; do
  echo "Waiting for postgres at $host..."
  sleep 1
done

exec $cmd
