FROM docker.io/library/rust:slim
RUN apt-get update -qq && apt-get install -y pkg-config libssl-dev curl && rm -rf /var/lib/apt/lists/*
WORKDIR /sdk
COPY . .
RUN cargo build --release 2>&1
COPY ci/smoke_test.rs /smoke_test.rs
RUN rustc /smoke_test.rs -o /smoke_test --edition 2021
CMD ["/smoke_test"]
