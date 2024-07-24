use json::JsonValue;

pub enum Error {
    JsonError(json::Error),
    JsonTypeError {
        key: Option<String>,
        expected: JsonType,
        got: JsonType,
    },
    HmathError(hmath::ConversionError),
    UnknownKey(String),
    JsonArrayLengthError {
        key: Option<String>,
        expected: usize,
        got: usize,
    },
    InvalidColorName(String),
    InvalidColorMode(String),
}

impl From<json::Error> for Error {
    fn from(e: json::Error) -> Self {
        Error::JsonError(e)
    }
}

impl From<hmath::ConversionError> for Error {
    fn from(e: hmath::ConversionError) -> Self {
        Error::HmathError(e)
    }
}

/// used to represent json types in `Error`
pub enum JsonType {
    Any,
    Null,
    Boolean,
    Integer,
    Number,
    String,
    Array(Box<JsonType>),
    Object,
}

pub(crate) fn get_type(v: &JsonValue) -> JsonType {
    match v {
        JsonValue::Null => JsonType::Null,
        JsonValue::Short(_)
        | JsonValue::String(_) => JsonType::String,
        JsonValue::Number(_) => JsonType::Number,
        JsonValue::Boolean(_) => JsonType::Boolean,
        JsonValue::Object(_) => JsonType::Object,
        JsonValue::Array(_) => JsonType::Array(Box::new(JsonType::Any)),
    }
}
