FROM alpine:3.22.1

RUN apk add go gcc

WORKDIR /app
COPY . /app

CMD ["go", "run", "."]