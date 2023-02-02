FROM rust:1.67 as builder

WORKDIR /usr/src/smfnotify

# Create project to build and cache dependencies.
RUN cargo init --bin
COPY ./Cargo.lock ./Cargo.toml ./
RUN cargo build --release && \
    rm ./src/main.rs && \
    rm ./target/release/deps/smfnotify*

# Add and compile actual source code.
COPY ./src ./src
RUN cargo build --release

FROM rust:1.67-slim-bullseye
COPY --from=builder /usr/src/smfnotify/target/release/smfnotify .
CMD ["./smfnotify", "--config", "config.toml"]
