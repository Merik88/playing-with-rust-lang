FROM rust:1.27.2 AS build
WORKDIR /src
COPY . .
RUN cargo build

FROM build AS test
RUN cargo test

FROM build AS publish
RUN cargo build --release

FROM debian:jessie-slim AS final
COPY --from=publish /src/target/release/docker_example .
CMD ["./docker_example"]