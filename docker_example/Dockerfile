FROM rust:1-slim AS build
WORKDIR /usr/src/app
# Create blank project
RUN USER=root cargo init
# Copy Cargo.toml to get dependencies
COPY Cargo.toml .
# This is a dummy build to get the dependencies cached
RUN cargo build
COPY src src
RUN cargo build

FROM build AS test
RUN cargo test

FROM build AS publish
RUN cargo build --release

FROM debian:stable-slim AS final
COPY --from=publish /usr/src/app/target/release/docker_example /bin/
CMD docker_example