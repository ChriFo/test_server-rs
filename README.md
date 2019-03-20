# (Integration) Test server

![GitHub tag](https://img.shields.io/github/tag/ChriFo/test-server-rs.svg)
[![Build Status](https://dev.azure.com/fochler/test-server-rs/_apis/build/status/ChriFo.test-server-rs)](https://dev.azure.com/fochler/test-server-rs/_build/latest?definitionId=1)
[![codecov](https://codecov.io/gh/ChriFo/test-server-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/ChriFo/test-server-rs)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

## Usage

```toml
[dev-dependencies]
test-server = { git = "https://github.com/ChriFo/test-server-rs", tag = "v0.5.6" }
```

[HttpResponse](https://actix.rs/api/actix-web/stable/actix_web/struct.HttpResponse.html) and [HttpRequest](https://actix.rs/api/actix-web/stable/actix_web/struct.HttpRequest.html) are re-exports from [actix-web](https://github.com/actix/actix-web).

```rust
extern crate test_server;

use test_server::HttpResponse;

#[test]
fn example_test() {
    // start server at random port
    let _ = test_server::new(0, |_| HttpResponse::Ok().into());

    // start server at given port
    let server = test_server::new(8080, |req| {
        println!("Request: {:#?}", req);
        HttpResponse::Ok().body("hello world")
    });

    // request against server
    let _ = client::get(&server.url());

    assert_eq!(1, server.requests.len());

    let last_request = server.requests.next().unwrap(); 

    assert_eq!("GET", last_request.method);
    assert_eq!("/", last_request.path);
    // body, headers and query params are also available
}
```

For more examples have a look at the [tests](https://github.com/ChriFo/test-server-rs/blob/master/tests/server.rs).
