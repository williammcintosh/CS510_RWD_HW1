# Base container that comes with rust already installed
FROM clux/muslrust:stable AS chef

USER root
# Cargo-chef allows us to build our dependencies ONLY, separate from our own code
# This takes advantage of Docker's caches and layering capabilities
RUN cargo install cargo-chef

WORKDIR /app

# This layer creates a cargo-chef recipe which details
# Exactly which dependencies need to be built
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# This layer uses the previously created cargo-chef recipe to do the actual building
FROM chef as builder

COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

#Build application itself
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin backend


FROM alpine AS runtime

# Copy SSL certs over
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# Copy the statically built binary over
COPY --from=builder \
  /app/target/x86_64-unknown-linux-musl/release/backend /usr/local/bin

EXPOSE 1337
CMD ["/usr/local/bin/backend"]
