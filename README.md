# JDER axum

A response builder for axum.

This package includes different axum response builders, extractors and layers based on the JSON response structure specified in [JSON Data Error Response (JDER)](https://github.com/alpheustangs/jder). With the builders and extractors provided, various kinds of responses can be created easily instead of sending plain text responses.

## Installation

To install this package, run the following command:

```bash
cargo add jder_axum
```

## Quick Start

Create a JSON response for an axum route:

```rust
use jder_axum::response::{
    Response,
    json::CreateJsonResponse,
};

async fn route() -> Response {
    CreateJsonResponse::dataless().send()
}
```

And the response will be shown as below:

```json
{
    "success": true,
    "data": null,
    "error": null
}
```

## Compatibility

A compatibility list of `axum` and `jder_axum`:

| `axum` Version | `jder_axum` Version |
| -------------- | ------------------- |
| ~0.8.3         | 0.5.0+              |
| ~0.8.1         | 0.4.0               |
| ~0.7.9         | 0.3.1               |
| ~0.7.7         | 0.1.0 - 0.3.0       |

## License

This project is licensed under the terms of the MIT license.
