use crate::worka_ui::StringValue;
use super::Widget;

#[derive(Debug, Clone, PartialEq)]
pub struct Text {
    pub text: StringValue,
    pub usage_hint: Option<String>,
}

impl Text {
    pub fn new(text: impl Into<StringValue>) -> Self {
        Self { text: text.into(), usage_hint: None }
    }

    pub fn usage_hint(mut self, hint: impl Into<String>) -> Self {
        self.usage_hint = Some(hint.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    pub url: StringValue,
    pub fit: Option<String>,
    pub usage_hint: Option<String>,
}

impl Image {
    pub fn new(url: impl Into<StringValue>) -> Self {
        Self { url: url.into(), fit: None, usage_hint: None }
    }

    pub fn fit(mut self, fit: impl Into<String>) -> Self {
        self.fit = Some(fit.into());
        self
    }

    pub fn usage_hint(mut self, hint: impl Into<String>) -> Self {
        self.usage_hint = Some(hint.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Icon {
    pub name: StringValue,
}

impl Icon {
    pub fn new(name: impl Into<StringValue>) -> Self {
        Self { name: name.into() }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Divider {
    pub axis: Option<String>,
}

impl Divider {
    pub fn horizontal() -> Self {
        Self { axis: Some("horizontal".into()) }
    }

    pub fn vertical() -> Self {
        Self { axis: Some("vertical".into()) }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Card {
    pub child: Box<Widget>,
}

impl Card {
    pub fn new(child: impl Into<Widget>) -> Self {
        Self { child: Box::new(child.into()) }
    }
}
