# JDER Axum

A JDER builder for Axum.

This package includes several Axum response builders and different extractors based on the JSON response structure specified in [JSON Data Error Response (JDER)](https://github.com/alpheustangs/jder). With the builders and extractors provided, various kinds of responses can be created easily instead of sending plain text responses.

## Quick Start

Create a JSON response for an Axum route:

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

## License

This project is MIT licensed, you can find the license file [here](https://github.com/alpheustangs/jder_axum/blob/main/LICENSE).
