FROM rust:1.81.0

WORKDIR /app
COPY . /app
COPY ./srs /srs

RUN cargo build

CMD ["cargo", "run", "--bin", "client"]
