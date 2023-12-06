FROM rust:latest

# 2. Copy the files in your machine to the Docker image
COPY ./ ./

CMD cargo test