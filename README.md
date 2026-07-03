# tiny.coffee

Fork of [`parrot.live`](https://github.com/hugomd/parrot.live), but with 100% less parrot and 100% more coffee. And now in go!

## ☕ Try it

```console
$ curl -L tiny.coffee
```

## ⚙️ Run it

Zero deps and only two env vars, `HOST` and `PORT`. Go wild.

### Go (>=1.16)

```console
$ go build main.go
```

### Container

```console
$ podman run -p 8000:8000 ghcr.io/robherley/tiny.coffee
```