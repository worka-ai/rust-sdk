use crate::worka_ui::wire::{BoolRef, NumberRef, StringArrayRef, StringRef};

#[derive(Debug, Clone, PartialEq)]
pub enum StringValue {
    Literal(String),
    Path(String),
}

impl StringValue {
    pub fn literal(value: impl Into<String>) -> Self {
        Self::Literal(value.into())
    }

    pub fn path(path: impl Into<String>) -> Self {
        Self::Path(path.into())
    }

    pub(crate) fn to_ref(&self) -> StringRef {
        match self {
            StringValue::Literal(value) => StringRef::literal(value.clone()),
            StringValue::Path(path) => StringRef::path(path.clone()),
        }
    }
}

impl From<&str> for StringValue {
    fn from(value: &str) -> Self {
        Self::literal(value)
    }
}

impl From<String> for StringValue {
    fn from(value: String) -> Self {
        Self::literal(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NumberValue {
    Literal(f64),
    Path(String),
}

impl NumberValue {
    pub fn literal(value: f64) -> Self {
        Self::Literal(value)
    }

    pub fn path(path: impl Into<String>) -> Self {
        Self::Path(path.into())
    }

    pub(crate) fn to_ref(&self) -> NumberRef {
        match self {
            NumberValue::Literal(value) => NumberRef::literal(*value),
            NumberValue::Path(path) => NumberRef::path(path.clone()),
        }
    }
}

impl From<f64> for NumberValue {
    fn from(value: f64) -> Self {
        Self::literal(value)
    }
}

impl From<i64> for NumberValue {
    fn from(value: i64) -> Self {
        Self::literal(value as f64)
    }
}

impl From<u64> for NumberValue {
    fn from(value: u64) -> Self {
        Self::literal(value as f64)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BoolValue {
    Literal(bool),
    Path(String),
}

impl BoolValue {
    pub fn literal(value: bool) -> Self {
        Self::Literal(value)
    }

    pub fn path(path: impl Into<String>) -> Self {
        Self::Path(path.into())
    }

    pub(crate) fn to_ref(&self) -> BoolRef {
        match self {
            BoolValue::Literal(value) => BoolRef::literal(*value),
            BoolValue::Path(path) => BoolRef::path(path.clone()),
        }
    }
}

impl From<bool> for BoolValue {
    fn from(value: bool) -> Self {
        Self::literal(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StringArrayValue {
    Literal(Vec<String>),
    Path(String),
}

impl StringArrayValue {
    pub fn literal(values: Vec<String>) -> Self {
        Self::Literal(values)
    }

    pub fn path(path: impl Into<String>) -> Self {
        Self::Path(path.into())
    }

    pub(crate) fn to_ref(&self) -> StringArrayRef {
        match self {
            StringArrayValue::Literal(values) => StringArrayRef::literal(values.clone()),
            StringArrayValue::Path(path) => StringArrayRef::path(path.clone()),
        }
    }
}

impl From<Vec<String>> for StringArrayValue {
    fn from(values: Vec<String>) -> Self {
        Self::literal(values)
    }
}
