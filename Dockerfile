ARG RUST_VERSION=1.84.1
ARG APP_NAME=mealplanner

## Build stage

FROM rust:${RUST_VERSION} AS build
ARG APP_NAME
WORKDIR /app

COPY . .

# build migrator
RUN (cd migration && cargo build --locked --release)

# build server
RUN cargo build --locked --release

RUN mkdir -p build && \
    cp ./target/release/$APP_NAME ./build/server && \
    cp ./migration/target/release/migration ./build/migration && \
    cp -r ./assets ./build/assets && \
    cp ./docker-container-start.sh ./build/docker-container-start.sh

# Exec stage
FROM rust:${RUST_VERSION}-slim AS final

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/go/dockerfile-user-best-practices/
# ARG UID=10001
# RUN adduser \
#     --disabled-password \
#     --gecos "" \
#     --home "/nonexistent" \
#     --shell "/sbin/nologin" \
#     --no-create-home \
#     --uid "${UID}" \
#     appuser
# USER appuser

WORKDIR /app

COPY --from=build /app/build .

EXPOSE 5050

CMD ["/app/docker-container-start.sh"]
