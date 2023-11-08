# Base image
FROM rust:1.72

# Working directory
WORKDIR /app

# Install system dependencies
RUN apt update && apt install lld clang -y

# Copy files from working environment to Docker image 
COPY . .

# Set environmental variable for sqlx offline queries
ENV SQLX_OFFLINE true

# Build our binary using release profile
RUN cargo build --release

ENTRYPOINT [ "./target/release/rusty-krab" ]
