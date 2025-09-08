FROM python:3.9-slim-bullseye as builder
WORKDIR /app
RUN apt-get update && apt-get install -y build-essential libpq-dev && rm -rf /var/lib/apt/lists/*
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt
COPY . /app

FROM debian:bullseye-slim
WORKDIR /app
RUN apt-get update && apt-get install -y libpq-dev postgresql-client && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local /usr/local
COPY --from=builder /app /app
EXPOSE 8000
RUN chmod +x wait-for-postgres.sh
CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8000"]