FROM golang:1.17 as build

WORKDIR /build

COPY go.mod ./
COPY go.sum ./
RUN go mod download
RUN go mod verify

COPY main.go ./
COPY frames/ ./frames/
COPY static/ ./static/

RUN go build -a -ldflags='-extldflags=-static' -o 'tiny.coffee'

FROM gcr.io/distroless/base-debian11

COPY --from=build /build/tiny.coffee /

CMD [ "/tiny.coffee" ]