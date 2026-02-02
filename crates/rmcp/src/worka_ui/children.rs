use crate::worka_ui::widgets::Widget;

#[derive(Debug, Clone, PartialEq)]
pub enum Children {
    Items(Vec<Widget>),
    Template {
        data_binding: String,
        template: Box<Widget>,
    },
}

impl Children {
    pub fn items(children: Vec<Widget>) -> Self {
        Self::Items(children)
    }

    pub fn template(data_binding: impl Into<String>, template: impl Into<Widget>) -> Self {
        Self::Template {
            data_binding: data_binding.into(),
            template: Box::new(template.into()),
        }
    }

    pub fn empty() -> Self {
        Self::Items(Vec::new())
    }
}

