FROM rust:latest AS build
RUN USER=root cargo new --bin authy
WORKDIR /authy

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
COPY ./src ./src
RUN cargo build --release


FROM alpine:3
WORKDIR /authy

COPY --from=build /authy/target/release/authy /authy/bin
RUN chmod +x /authy/bin
ENTRYPOINT [ "/authy/bin" ]