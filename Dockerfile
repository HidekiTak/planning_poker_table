FROM --platform=$BUILDPLATFORM rust:latest AS builder

RUN update-ca-certificates
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev gcc-x86-64-linux-gnu

WORKDIR /work

COPY Cargo.* .

ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'
ENV CC='gcc'
ENV CC_x86_64_unknown_linux_musl=x86_64-linux-gnu-gcc
ENV CC_x86_64-unknown-linux-musl=x86_64-linux-gnu-gcc

RUN mkdir -p src \
    && echo 'fn main() {}' > src/main.rs \
    && cargo build --release --target x86_64-unknown-linux-musl \
    && rm -Rf src

COPY src src
RUN cargo build --release --target x86_64-unknown-linux-musl && \
    x86_64-linux-gnu-strip /work/target/x86_64-unknown-linux-musl/release/planning_poker

FROM --platform=linux/x86_64 alpine:latest
# amazonlinux:2023.0.20230315.0
COPY --from=builder \
    /work/target/x86_64-unknown-linux-musl/release/planning_poker /

EXPOSE 8080
ENTRYPOINT ["/planning_poker"]
