package main

import (
	"log"
	"net"
	"net/http"
	"os"

	"github.com/robherley/tiny.coffee/api"
)

func main() {
	host := "0.0.0.0"
	if ehost := os.Getenv("HOST"); ehost != "" {
		host = ehost
	}

	port := "8000"
	if eport := os.Getenv("PORT"); eport != "" {
		port = eport
	}

	server := &http.Server{
		Addr:    net.JoinHostPort(host, port),
		Handler: http.HandlerFunc(api.Handler),
	}

	log.Println("serving coffee on:", server.Addr)
	log.Fatal(server.ListenAndServe())
}
