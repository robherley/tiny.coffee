FROM golang:1.26 AS build

WORKDIR /build

COPY go.mod ./
COPY go.sum ./
RUN go mod download
RUN go mod verify

COPY main.go ./
COPY frames/ ./frames/
COPY static/ ./static/

RUN CGO_ENABLED=0 go build -a -ldflags='-extldflags=-static' -o 'tiny.coffee'

FROM gcr.io/distroless/static-debian13

COPY --from=build /build/tiny.coffee /

CMD [ "/tiny.coffee" ]