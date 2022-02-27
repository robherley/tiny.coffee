package main

import (
	"context"
	"embed"
	"log"
	"net"
	"net/http"
	"os"
	"path"
	"strings"
	"time"
)

var (
	//go:embed static/index.html
	indexHTML []byte

	//go:embed frames/*.txt
	frameFS embed.FS
	frames  [][]byte

	ansiClear  = []byte("\033[H\033[2J")
	ansiReset  = []byte("\033[0m")
	ansiColors = [...][]byte{
		[]byte("\033[1;31m"),
		[]byte("\033[1;32m"),
		[]byte("\033[1;33m"),
		[]byte("\033[1;34m"),
		[]byte("\033[1;35m"),
		[]byte("\033[1;36m"),
	}
)

func init() {
	framesDir, err := frameFS.ReadDir("frames")
	if err != nil {
		log.Fatalln("unable to read frame dir:", err)
	}

	frames = make([][]byte, len(framesDir))
	for i := range framesDir {
		frame, err := frameFS.ReadFile(path.Join("frames", framesDir[i].Name()))
		if err != nil {
			log.Fatalln("unable to read frame file:", err)
		}

		frames[i] = frame
	}
}

func main() {
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		isCurl := strings.Contains(r.Header.Get("user-agent"), "curl")
		log.Println("new req | is curl:", isCurl)

		if isCurl {
			idx := 0
			ticker := time.NewTicker(time.Millisecond * 100)
			reqCancelCtx, cancel := context.WithTimeout(r.Context(), time.Minute*2)
			defer cancel()

			for {
				select {
				case <-reqCancelCtx.Done():
					w.Write([]byte("no more coffee :(\n"))
					return
				case <-ticker.C:
					w.Write(ansiClear)
					w.Write(ansiColors[idx%len(ansiColors)])
					w.Write(frames[idx%len(frames)])
					w.Write(ansiReset)
					if f, ok := w.(http.Flusher); ok {
						f.Flush()
					}
					idx = idx + 1
				}
			}
		} else {
			w.Header().Set("Content-Type", "text/html; charset=utf-8")
			w.Write(indexHTML)
		}
	})

	host := "0.0.0.0"
	if ehost := os.Getenv("HOST"); ehost != "" {
		host = ehost
	}

	port := "8000"
	if eport := os.Getenv("PORT"); eport != "" {
		port = eport
	}

	addr := net.JoinHostPort(host, port)
	log.Println("serving coffee on:", addr)
	log.Fatal(http.ListenAndServe(addr, nil))
}
