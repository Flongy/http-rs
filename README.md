# HTTP/1.1 Server in Rust

## Description
This project is inspired by Python's standard library module [http.server](https://docs.python.org/3/library/http.server.html). Python lets you implement `do_GET`, `do_POST` or other types of requests and the server redirects all user requests to these methods. 

`http-rs` will _eventually_ get the same feature through implementing the corresponding functions.

For now it's not as versatile but you still can set your request handlers in the `match` in `main.rs`: 
```rust
match method {
    Method::GET  => { /* Your cool GET request handler */ },
    Method::POST => { /* Your omega POST request handler */ },
    _ => writer.write_all(responses::NOT_IMPLEMENTED).await.unwrap(),
};
```

The example handler function for implementing static website is located in [static_site.rs](src/examples/static_site.rs).

## Example
To compile the server you'll need [tokio](https://github.com/tokio-rs/tokio) library. You can use `cargo` to download project dependencies and compile: 
```console
cargo build
cargo run
```

Now you can open your browser and go to `localhost:8000` to retrieve the webpage.

The served webpage is located in `www` folder which has [materialize](https://materializecss.com/) demo page by default.

## Todo
- [x] HTTP request handling
- [x] Async HTTP server
- [ ] User definable handlers
- [ ] User definable settings (at least ip-address)
- [ ] HTTPS
