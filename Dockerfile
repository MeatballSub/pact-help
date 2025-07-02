# docker build --no-cache -t consumer .
FROM rust:slim

ENV PACT_OVERWRITE=true
ENV PACT_OUTPUT_DIR=/usr/src/pacts

RUN apt-get update && \
    apt-get install -y cmake curl pkg-config protobuf-compiler && \
    rm -rf /var/lib/apt/lists/*

COPY /container /usr/src/

WORKDIR /usr/src/proto
RUN mkdir -pv $PACT_OUTPUT_DIR && rm -fr generated && mkdir -p generated/rust && cd rust_generator && cargo clean && cargo build

WORKDIR /usr/src/consumer
RUN cargo build && cargo test --no-run

CMD ["cargo", "test"]