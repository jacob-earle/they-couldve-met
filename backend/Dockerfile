FROM rust:1.62 as builder

WORKDIR /usr/src/api-server
COPY . .

# Installing the wikipedia scraper as a binary that we can run from within the container
RUN cargo install --path ./wikipedia-scraper

# Main command installation
RUN cargo install --path ./api-server

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev libpq-dev ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/api-server /usr/local/bin/api-server
COPY --from=builder /usr/local/cargo/bin/wikipedia-scraper /usr/local/bin/wikipedia-scraper

# Copying the sparql-query file for use with the wikipedia scraper
COPY ./sparql-query /usr/local/sparql-query

CMD ["api-server"]