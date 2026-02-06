use crate::worka_ui::Children;

#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    pub children: Children,
    pub distribution: Option<String>,
    pub alignment: Option<String>,
}

impl Row {
    pub fn new(children: Children) -> Self {
        Self {
            children,
            distribution: None,
            alignment: None,
        }
    }

    pub fn distribution(mut self, distribution: impl Into<String>) -> Self {
        self.distribution = Some(distribution.into());
        self
    }

    pub fn alignment(mut self, alignment: impl Into<String>) -> Self {
        self.alignment = Some(alignment.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Column {
    pub children: Children,
    pub distribution: Option<String>,
    pub alignment: Option<String>,
}

impl Column {
    pub fn new(children: Children) -> Self {
        Self {
            children,
            distribution: None,
            alignment: None,
        }
    }

    pub fn distribution(mut self, distribution: impl Into<String>) -> Self {
        self.distribution = Some(distribution.into());
        self
    }

    pub fn alignment(mut self, alignment: impl Into<String>) -> Self {
        self.alignment = Some(alignment.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct List {
    pub children: Children,
    pub direction: Option<String>,
    pub alignment: Option<String>,
}

impl List {
    pub fn new(children: Children) -> Self {
        Self {
            children,
            direction: None,
            alignment: None,
        }
    }

    pub fn vertical(children: Children) -> Self {
        Self {
            children,
            direction: Some("vertical".into()),
            alignment: None,
        }
    }

    pub fn horizontal(children: Children) -> Self {
        Self {
            children,
            direction: Some("horizontal".into()),
            alignment: None,
        }
    }

    pub fn alignment(mut self, alignment: impl Into<String>) -> Self {
        self.alignment = Some(alignment.into());
        self
    }
}
