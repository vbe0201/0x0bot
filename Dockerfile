FROM rust:1.38.0

WORKDIR /usr/src/bot

COPY . .

# Export environment variables
ARG DISCORD_TOKEN
ENV DISCORD_TOKEN=$DISCORD_TOKEN

ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL

# Compile
RUN cargo install --path .

# Run the instance
CMD ["0x0bot"]
