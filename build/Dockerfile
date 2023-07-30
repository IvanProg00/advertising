FROM rust:1.71.0

RUN apt-get update && \
  apt-get install -y libpq-dev postgresql && \
  cargo install diesel_cli --no-default-features --features postgres

WORKDIR /app

COPY migrations/ migrations/
COPY src/ src/
COPY ["Cargo.toml", "Cargo.lock", "diesel.toml", "./"]

RUN cargo install --path .

CMD ["cargo", "run"]
