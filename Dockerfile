FROM rust:bullseye
WORKDIR /usr/src/myapp
COPY . .

#RUN cargo install --path .
RUN cargo build
ENTRYPOINT cargo run