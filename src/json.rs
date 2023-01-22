use std::collections::HashMap;
use std::fmt;

pub enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

impl fmt::Display for Json {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Json::Null => {
                write!(f, "null")
            }
            Json::Boolean(v) => {
                write!(f, "{}", v)
            }
            Json::Number(v) => {
                write!(f, "{}", v)
            }
            Json::String(v) => {
                write!(f, "\"{}\"", v)
            }
            Json::Array(v) => {
                write!(f, "[")?;
                for (pos, e) in v.iter().enumerate() {
                    write!(f, "{}", e)?;
                    if pos < v.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            Json::Object(v) => {
                write!(f, "{{").expect("");
                for e in v.keys().enumerate() {
                    write!(f, "\"{}\": ", e.1)?;
                    write!(f, "{}", v.get(e.1).unwrap())?;
                    if e.0 < v.keys().len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "}}")
            }
        }
    }
}
