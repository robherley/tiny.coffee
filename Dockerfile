FROM rust:1.78 as build

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.toml
COPY src/ src/
COPY frames/ ./frames/
COPY static/ ./static/

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian12

COPY --from=build /usr/local/cargo/bin/tiny_coffee /tiny_coffee

CMD [ "/tiny_coffee" ]
