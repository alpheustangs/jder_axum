## 0.3.1 (2024-12-16)

### What's Changed

- Update dependencies
- Update documentation

## 0.3.0 (2024-10-26)

### Breaking Changes

- Values renamed in `JsonResponseErrorCode`

### What's Changed

- `status` in `CreateJsonResponse` accept more types of input now
- `version` in `CreateJsonResponse` accept more types of input now
- `error_code` in `CreateJsonResponse` accept more types of input now
- `error_field` in `CreateJsonResponse` accept more types of input now
- `error_message` in `CreateJsonResponse` accept more types of input now

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
