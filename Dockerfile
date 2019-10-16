FROM rust:1.38.0

WORKDIR /usr/src/bot

COPY . .

# Compile
RUN cargo install --path .

# Run the instance
CMD ["iqbot"]
