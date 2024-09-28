> [!CAUTION]
> This project is in a very early stage of development and is not yet ready for use.

---

# retch-http

A stealthy (drop-in?) replacement for the `fetch` API in Node.JS, written in Rust.

## Features

- modifiable browser-like HTTP headers (Firefox, Chrome)
- HTTP/2 support
- automatic `gzip` decompression

## Roadmap

- Full `fetch` API compatibility
    - Currently, only the `GET` method is supported
- Modifiable TLS fingerprint
- Stealthier HTTP/2 behaviour

----

Jindřich Bär, 2024

