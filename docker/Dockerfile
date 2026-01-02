FROM python:3.12-slim

# Install uv
COPY --from=ghcr.io/astral-sh/uv:latest /uv /bin/uv

WORKDIR /app

RUN apt-get update && apt-get install -y \
    git \
    && rm -rf /var/lib/apt/lists/*

COPY pyproject.toml .

RUN uv pip install --system -r pyproject.toml

COPY . .

RUN useradd --create-home --shell /bin/bash app \
    && chown -R app:app /app
USER app

ENV PYTHONPATH=/app
ENV PYTHONUNBUFFERED=1

# Default command
CMD ["python", "main.py"]