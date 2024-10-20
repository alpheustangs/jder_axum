## 0.2.0 (2024-10-14)

### Breaking Changes

- Enum `ResponseErrorCode` renamed to `JsonResponseErrorCode`
- Values renamed in `JsonResponseErrorCode`:
    - `ParseError` => `Parse`
    - `ServerError` => `Server`
    - `UnknownError` => `Unknown`
- Move JSON related stuffs into `response::json` module
- Changes in accepted value type of `error_code`:
    - `String` => `&str`
- Changes in accepted value type of `error_field`:
    - `String` => `&str`
- Changes in accepted value type of `error_message`:
    - `String` => `&str`

### What's New

- Add different derives for different structs
- Add `get_header_from_key_value` function
- Add `get_header_name_from_key` function
- Add `get_header_value_from_value` function
- Add `as_str` for `JsonResponseErrorCode`

### What's Changed

- Changes in accepted value type of `key` in `header` and `headers`:
    - `HeaderName` => `&str`/`String`/`HeaderName`
- Changes in accepted value type of `value` in `header` and `headers`:
    - `String` => `&str`/`String`/`HeaderValue`
- Updates in documentation

## 0.1.0 (2024-10-04)

First release
