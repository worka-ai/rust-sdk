use crate::worka_ui::StringValue;

#[derive(Debug, Clone, PartialEq)]
pub struct AudioPlayer {
    pub url: StringValue,
}

impl AudioPlayer {
    pub fn new(url: impl Into<StringValue>) -> Self {
        Self { url: url.into() }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Video {
    pub url: StringValue,
}

impl Video {
    pub fn new(url: impl Into<StringValue>) -> Self {
        Self { url: url.into() }
    }
}
