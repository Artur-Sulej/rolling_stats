# Stage 1: Build the Elixir/Phoenix app
FROM hexpm/elixir:1.17.3-erlang-26.2.5.3-debian-bookworm-20240926-slim AS builder

ENV MIX_ENV=prod
WORKDIR /app

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    build-essential \
    git \
    curl

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup default stable
RUN rustup target add x86_64-unknown-linux-musl

RUN mix local.hex --force && \
    mix local.rebar --force

COPY mix.exs mix.lock ./
COPY native native
RUN mix deps.get
COPY config config
COPY rel rel
COPY lib lib
RUN mix compile
RUN mix release

# Stage 2: Create the release image
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    openssl && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/_build/prod/rel ./

ENV HOME=/app
ENV PORT=4000

EXPOSE $PORT

ENV PHX_SERVER=true
CMD ["rolling_stats/bin/rolling_stats", "start"]
