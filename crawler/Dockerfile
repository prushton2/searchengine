FROM alpine:3.22.1

RUN apk add rust cargo
RUN apk add openssl-dev musl-dev
RUN apk add pkgconfig

WORKDIR /app
COPY . /app

RUN cargo build --release

CMD ["./target/release/crawler"]