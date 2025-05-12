use http::{Error as HTTPError, HeaderName, HeaderValue};

/// Convert a key into a header name.
///
/// Returns a header name if successful, else returns an error.
///
/// To validate the value as well, see [`get_header_from_key_value`].
///
/// ## Example
///
/// ```no_run
/// use axum::http::HeaderName;
/// use jder_axum::response::header::get_header_name_from_key;
///
/// let name: HeaderName =
///     get_header_name_from_key("key").unwrap();
/// ```
pub fn get_header_name_from_key<K>(key: K) -> Result<HeaderName, HTTPError>
where
    HeaderName: TryFrom<K>,
    <HeaderName as TryFrom<K>>::Error: Into<HTTPError>,
{
    let k: HeaderName =
        <HeaderName as TryFrom<K>>::try_from(key).map_err(Into::into)?;

    Ok(k)
}

/// Convert a value into a header value.
///
/// Returns a header value if successful, else returns an error.
///
/// To validate the key as well, see [`get_header_from_key_value`].
///
/// ## Example
///
/// ```no_run
/// use axum::http::HeaderValue;
/// use jder_axum::response::header::get_header_value_from_value;
///
/// let value: HeaderValue =
///     get_header_value_from_value("value").unwrap();
/// ```
pub fn get_header_value_from_value<V>(
    value: V
) -> Result<HeaderValue, HTTPError>
where
    HeaderValue: TryFrom<V>,
    <HeaderValue as TryFrom<V>>::Error: Into<HTTPError>,
{
    let v: HeaderValue =
        <HeaderValue as TryFrom<V>>::try_from(value).map_err(Into::into)?;

    Ok(v)
}

/// Convert a key-value pair into a header.
///
/// Returns a tuple of header name and header value if successful,
/// else returns an error.
///
/// To validate the key only, see [`get_header_name_from_key`].
///
/// To validate the value only, see [`get_header_value_from_value`].
///
/// ## Example
///
/// ```no_run
/// use axum::http::{HeaderName, HeaderValue};
/// use jder_axum::response::header::get_header_from_key_value;
///
/// let header: (HeaderName, HeaderValue) =
///     get_header_from_key_value("key", "value").unwrap();
/// ```
pub fn get_header_from_key_value<K, V>(
    key: K,
    value: V,
) -> Result<(HeaderName, HeaderValue), HTTPError>
where
    HeaderName: TryFrom<K>,
    <HeaderName as TryFrom<K>>::Error: Into<HTTPError>,
    HeaderValue: TryFrom<V>,
    <HeaderValue as TryFrom<V>>::Error: Into<HTTPError>,
{
    let k: HeaderName = get_header_name_from_key(key)?;
    let v: HeaderValue = get_header_value_from_value(value)?;

    Ok((k, v))
}
