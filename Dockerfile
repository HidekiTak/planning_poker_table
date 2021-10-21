FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

WORKDIR /work

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release && \
    strip /work/target/x86_64-unknown-linux-musl/release/planning_poker


FROM scratch
COPY --from=builder \
    /work/target/x86_64-unknown-linux-musl/release/planning_poker /

EXPOSE 8080
ENTRYPOINT ["/planning_poker"]
