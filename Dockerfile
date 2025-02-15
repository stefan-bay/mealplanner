ARG RUST_VERSION=1.84.1
ARG APP_NAME=mealplanner

## Build stage

FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

# Install host build dependencies.
RUN apk add --no-cache clang lld musl-dev git

COPY . .
RUN mkdir -p ./build && \
    cargo build --locked --release && \
    cp ./target/release/$APP_NAME ./build/server && \
    cp -r ./assets ./build/assets

## Exec stage
FROM alpine:3.18 AS final

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/go/dockerfile-user-best-practices/
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

WORKDIR /app

COPY --from=build /app/build .

EXPOSE 5050

CMD ["/app/server"]
