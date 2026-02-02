use crate::worka_ui::{Action, BoolValue, NumberValue, StringArrayValue, StringValue};
use super::Widget;

#[derive(Debug, Clone, PartialEq)]
pub struct Button {
    pub child: Box<Widget>,
    pub action: Action,
    pub primary: Option<bool>,
}

impl Button {
    pub fn new(child: impl Into<Widget>, action: Action) -> Self {
        Self { child: Box::new(child.into()), action, primary: None }
    }

    pub fn primary(mut self, value: bool) -> Self {
        self.primary = Some(value);
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextField {
    pub text: Option<StringValue>,
    pub label: Option<StringValue>,
    pub text_field_type: Option<String>,
    pub validation_regexp: Option<String>,
    pub on_submitted_action: Option<Action>,
}

impl TextField {
    pub fn new() -> Self {
        Self {
            text: None,
            label: None,
            text_field_type: None,
            validation_regexp: None,
            on_submitted_action: None,
        }
    }

    pub fn text(mut self, value: impl Into<StringValue>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<StringValue>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn field_type(mut self, value: impl Into<String>) -> Self {
        self.text_field_type = Some(value.into());
        self
    }

    pub fn validation_regexp(mut self, value: impl Into<String>) -> Self {
        self.validation_regexp = Some(value.into());
        self
    }

    pub fn on_submitted(mut self, action: Action) -> Self {
        self.on_submitted_action = Some(action);
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CheckBox {
    pub label: StringValue,
    pub value: BoolValue,
}

impl CheckBox {
    pub fn new(label: impl Into<StringValue>, value: impl Into<BoolValue>) -> Self {
        Self { label: label.into(), value: value.into() }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Modal {
    pub entry_point: Box<Widget>,
    pub content: Box<Widget>,
}

impl Modal {
    pub fn new(entry_point: impl Into<Widget>, content: impl Into<Widget>) -> Self {
        Self { entry_point: Box::new(entry_point.into()), content: Box::new(content.into()) }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TabItem {
    pub title: StringValue,
    pub child: Widget,
}

impl TabItem {
    pub fn new(title: impl Into<StringValue>, child: impl Into<Widget>) -> Self {
        Self { title: title.into(), child: child.into() }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tabs {
    pub tab_items: Vec<TabItem>,
}

impl Tabs {
    pub fn new(tab_items: Vec<TabItem>) -> Self {
        Self { tab_items }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChoiceOption {
    pub label: StringValue,
    pub value: String,
}

impl ChoiceOption {
    pub fn new(label: impl Into<StringValue>, value: impl Into<String>) -> Self {
        Self { label: label.into(), value: value.into() }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MultipleChoice {
    pub selections: StringArrayValue,
    pub options: Vec<ChoiceOption>,
    pub max_allowed_selections: Option<i64>,
}

impl MultipleChoice {
    pub fn new(selections: impl Into<StringArrayValue>, options: Vec<ChoiceOption>) -> Self {
        Self { selections: selections.into(), options, max_allowed_selections: None }
    }

    pub fn max_allowed_selections(mut self, value: i64) -> Self {
        self.max_allowed_selections = Some(value);
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Slider {
    pub value: NumberValue,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
}

impl Slider {
    pub fn new(value: impl Into<NumberValue>) -> Self {
        Self { value: value.into(), min_value: None, max_value: None }
    }

    pub fn min(mut self, value: f64) -> Self {
        self.min_value = Some(value);
        self
    }

    pub fn max(mut self, value: f64) -> Self {
        self.max_value = Some(value);
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DateTimeInput {
    pub value: StringValue,
    pub enable_date: Option<bool>,
    pub enable_time: Option<bool>,
    pub first_date: Option<String>,
    pub last_date: Option<String>,
}

impl DateTimeInput {
    pub fn new(value: impl Into<StringValue>) -> Self {
        Self {
            value: value.into(),
            enable_date: None,
            enable_time: None,
            first_date: None,
            last_date: None,
        }
    }

    pub fn enable_date(mut self, value: bool) -> Self {
        self.enable_date = Some(value);
        self
    }

    pub fn enable_time(mut self, value: bool) -> Self {
        self.enable_time = Some(value);
        self
    }

    pub fn first_date(mut self, value: impl Into<String>) -> Self {
        self.first_date = Some(value.into());
        self
    }

    pub fn last_date(mut self, value: impl Into<String>) -> Self {
        self.last_date = Some(value.into());
        self
    }
}
