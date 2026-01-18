use serde::Deserialize;

/// Serialize Option<f64> as Option<String>
pub fn as_str_opt<S>(
    opt: &Option<f64>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match opt {
        Some(num) => serializer.serialize_some(&num.to_string()),
        None => serializer.serialize_none(),
    }
}

/// Serialize f64 as String
pub fn as_str_f64<S>(
    num: &f64,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&num.to_string())
}

/// Serialize Option<bool> as String ("true" or "false"), or skip if None
pub fn as_str_bool<S>(
    b: &Option<bool>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match b {
        Some(val) => serializer.serialize_some(if *val { "true" } else { "false" }),
        None => serializer.serialize_none(),
    }
}

pub fn retryable_from_int<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::<u8>::deserialize(deserializer)?;
    Ok(match opt {
        Some(1) => Some(true),
        Some(0) => Some(false),
        _ => None,
    })
}

pub fn serialize_as_json_string<T, S>(
    opt: &Option<T>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    T: serde::Serialize,
    S: serde::Serializer,
{
    // This will forcibly serialize as JSON string if present.
    match opt {
        Some(val) => {
            let json_str = serde_json::to_string(val).map_err(serde::ser::Error::custom)?;
            serializer.serialize_some(&json_str)
        }
        None => serializer.serialize_none(),
    }
}
