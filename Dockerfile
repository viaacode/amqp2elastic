FROM clux/muslrust:1.50.0-stable as builder
WORKDIR /volume
COPY . .
RUN cargo build --release

FROM scratch
COPY --from=builder /volume/target/x86_64-unknown-linux-musl/release/amqp2elastic .
ENTRYPOINT [ "/amqp2elastic" ]
