FROM rust:buster

COPY ./build.sh ./
RUN chmod +x /build.sh

copy ./target/aarch64-unknown-linux-gnu/debug/builder /builder
RUN chmod +x ./builder

ENTRYPOINT ["/build.sh"]
