FROM rust:1.81.0

WORKDIR /app
COPY . /app

RUN cargo build

CMD ["cargo", "run", "--bin", "client"]
