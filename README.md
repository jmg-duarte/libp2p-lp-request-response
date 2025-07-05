# libp2p-lp-codec

A Rust library providing length-prefixed codecs for libp2p request-response protocols. This library implements variable-length integer prefixes to encode message sizes, preventing buffer overflow attacks and enabling efficient streaming of structured data.

> **Note**: This library was originally developed in the context of the [polka-storage](https://github.com/eigerco/polka-storage) project.

This library was built to enable Rust applications to interface with libp2p-js request-response protocols using `lpStream`-like streams (e.g. [`cborStream`](https://github.com/achingbrain/it/tree/main/packages/it-cbor-stream))
like the [libp2p-request-response example](https://github.com/libp2p/js-libp2p-examples/blob/7079fb6561cccc3af5066f85a4af14d7b0d8e7d5/examples/js-libp2p-example-custom-protocols/2-request-response.js#L7).
By providing compatible implementations of these codecs, Rust peers can seamlessly communicate with JavaScript peers in multi-language libp2p networks.

## Features

By default, this library provides only the core length-prefixing functionality. Specific serialization formats are available through optional features:

- **`cbor`** - CBOR codec using `cbor4ii`
- **`json`** - JSON codec using `serde_json`

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
libp2p-lp-codec = { git = "https://github.com/jmg-duarte/libp2p-lp-request-response", features = ["cbor"] }
```

### Available Features

#### CBOR Codec

Enable the CBOR codec with the `cbor` feature:

```toml
[dependencies]
libp2p-lp-codec = { git = "https://github.com/jmg-duarte/libp2p-lp-request-response", features = ["cbor"] }
```

```rust
use libp2p_lp_codec::LpCbor;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyRequest {
    data: String,
}

#[derive(Serialize, Deserialize)]
struct MyResponse {
    result: u32,
}

// Use with libp2p request-response
let codec = LpCbor::<MyRequest, MyResponse, &str>::default();
```

#### JSON Codec

Enable the JSON codec with the `json` feature:

```toml
[dependencies]
libp2p-lp-codec = { git = "https://github.com/jmg-duarte/libp2p-lp-request-response", features = ["json"] }
```

```rust
use libp2p_lp_codec::LpJson;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyRequest {
    data: String,
}

#[derive(Serialize, Deserialize)]
struct MyResponse {
    result: u32,
}

// Use with libp2p request-response
let codec = LpJson::<MyRequest, MyResponse, &str>::default();
```

#### Multiple Codecs

You can enable multiple codecs simultaneously:

```toml
[dependencies]
libp2p-lp-codec = { git = "https://github.com/jmg-duarte/libp2p-lp-request-response", features = ["cbor", "json"] }
```

## Examples

### Basic Usage with libp2p

```rust
use libp2p_lp_codec::LpCbor;
use libp2p_request_response::{Config, Behaviour};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Ping {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct Pong {
    response: String,
}

let codec = LpCbor::<Ping, Pong, &str>::default();
let config = Config::default();
let behaviour = Behaviour::with_codec(codec, std::iter::once(("/my-protocol/1.0.0", config)));
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
