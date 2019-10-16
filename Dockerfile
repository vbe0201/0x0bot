FROM rust:1.38.0

WORKDIR /usr/src/bot

COPY . .

# Compile
RUN cargo build --release

# Run the instance
CMD ["cargo run"]
