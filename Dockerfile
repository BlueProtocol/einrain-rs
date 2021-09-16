FROM rust:alpine as dependencies

RUN apk add --no-cache alpine-sdk
RUN cargo install cargo-chef

FROM dependencies as planner
WORKDIR /einrain-rs
# We only pay the installation cost once, 
# it will be cached from the second build onwards
# To ensure a reproducible build consider pinning 
# the cargo-chef version with `--version X.X.X`
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM dependencies as cacher
WORKDIR /einrain-rs
RUN cargo install cargo-chef
COPY --from=planner /einrain-rs/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM dependencies as builder
WORKDIR /einrain-rs
COPY . .
# Copy over the cached dependencies
COPY --from=cacher /einrain-rs/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release --bin einrain-rs

FROM alpine as runtime
WORKDIR /einrain-rs
COPY --from=builder /einrain-rs/target/release/einrain-rs /einrain-rs/einrain-rs
COPY --from=builder /einrain-rs/config.toml /einrain-rs/config.toml

ENV RUST_LOG=info
ENTRYPOINT ["./einrain-rs"]