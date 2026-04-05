FROM rust:1-bookworm AS build

RUN apt-get update \
    && apt-get install -y --no-install-recommends git ca-certificates \
    && rm -rf /var/lib/apt/lists/*

ARG REPO_URL=https://github.com/ADAMADA8/QuestionAPI
ARG REPO_REF=master

WORKDIR /src
RUN test -n "$REPO_URL"
RUN git clone --depth 1 --branch "$REPO_REF" "$REPO_URL" app

WORKDIR /src/app
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=build /src/app/target/release/QuestionAPI /app/QuestionAPI
RUN mkdir -p /app/data

EXPOSE 8080
WORKDIR /app/data
CMD ["/app/QuestionAPI"]
