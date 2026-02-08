FROM docker.io/library/rust:slim
RUN apt-get update -qq && apt-get install -y pkg-config libssl-dev curl && rm -rf /var/lib/apt/lists/*
WORKDIR /sdk
COPY . .
RUN sed -i 's/features = \["fs"\]/features = ["fs", "rt", "macros"]/' Cargo.toml
RUN printf '\n[[bin]]\nname = "smoke_test"\npath = "ci/smoke_test.rs"\n' >> Cargo.toml
RUN cargo build --release --bin smoke_test 2>&1
CMD ["/sdk/target/release/smoke_test"]
