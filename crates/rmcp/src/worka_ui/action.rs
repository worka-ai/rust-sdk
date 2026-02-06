use crate::worka_ui::wire;

#[derive(Debug, Clone, PartialEq)]
pub enum ActionValue {
    Path(String),
    String(String),
    Number(f64),
    Bool(bool),
}

impl ActionValue {
    pub fn path(path: impl Into<String>) -> Self {
        Self::Path(path.into())
    }

    pub(crate) fn to_wire(&self) -> wire::ActionValue {
        match self {
            ActionValue::Path(path) => wire::ActionValue::path(path.clone()),
            ActionValue::String(value) => wire::ActionValue::literal_string(value.clone()),
            ActionValue::Number(value) => wire::ActionValue::literal_number(*value),
            ActionValue::Bool(value) => wire::ActionValue::literal_boolean(*value),
        }
    }
}

impl From<&str> for ActionValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for ActionValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<bool> for ActionValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<f64> for ActionValue {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<i64> for ActionValue {
    fn from(value: i64) -> Self {
        Self::Number(value as f64)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ActionContextEntry {
    pub key: String,
    pub value: ActionValue,
}

impl ActionContextEntry {
    pub fn new(key: impl Into<String>, value: impl Into<ActionValue>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Action {
    pub name: String,
    pub context: Vec<ActionContextEntry>,
}

impl Action {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            context: Vec::new(),
        }
    }

    pub fn with(mut self, key: impl Into<String>, value: impl Into<ActionValue>) -> Self {
        self.context.push(ActionContextEntry::new(key, value));
        self
    }

    pub(crate) fn to_wire(&self) -> wire::Action {
        if self.context.is_empty() {
            wire::Action::new(self.name.clone())
        } else {
            let context = self
                .context
                .iter()
                .map(|entry| wire::ActionContextEntry {
                    key: entry.key.clone(),
                    value: entry.value.to_wire(),
                })
                .collect();
            wire::Action::new(self.name.clone()).with_context(context)
        }
    }
}
