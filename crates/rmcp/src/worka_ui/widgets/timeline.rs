use super::Widget;
use crate::worka_ui::{Action, BoolValue, Children, NumberValue, StringValue};

#[derive(Debug, Clone, PartialEq)]
pub struct Timeline {
    pub children: Children,
    pub orientation: Option<String>,
    pub alignment: Option<String>,
    pub auto_follow: Option<BoolValue>,
    pub lane_mode: Option<String>,
    pub current_item_id: Option<StringValue>,
}

impl Timeline {
    pub fn new(children: Children) -> Self {
        Self {
            children,
            orientation: None,
            alignment: None,
            auto_follow: None,
            lane_mode: None,
            current_item_id: None,
        }
    }

    pub fn orientation(mut self, value: impl Into<String>) -> Self {
        self.orientation = Some(value.into());
        self
    }

    pub fn alignment(mut self, value: impl Into<String>) -> Self {
        self.alignment = Some(value.into());
        self
    }

    pub fn auto_follow(mut self, value: impl Into<BoolValue>) -> Self {
        self.auto_follow = Some(value.into());
        self
    }

    pub fn lane_mode(mut self, value: impl Into<String>) -> Self {
        self.lane_mode = Some(value.into());
        self
    }

    pub fn current_item_id(mut self, value: impl Into<StringValue>) -> Self {
        self.current_item_id = Some(value.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimelineItem {
    pub item_id: String,
    pub title: Option<StringValue>,
    pub subtitle: Option<StringValue>,
    pub timestamp: Option<StringValue>,
    pub kind: Option<String>,
    pub state: Option<String>,
    pub severity: Option<String>,
    pub icon: Option<StringValue>,
    pub content: Option<Box<Widget>>,
    pub action: Option<Action>,
}

impl TimelineItem {
    pub fn new(item_id: impl Into<String>) -> Self {
        Self {
            item_id: item_id.into(),
            title: None,
            subtitle: None,
            timestamp: None,
            kind: None,
            state: None,
            severity: None,
            icon: None,
            content: None,
            action: None,
        }
    }

    pub fn title(mut self, value: impl Into<StringValue>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn subtitle(mut self, value: impl Into<StringValue>) -> Self {
        self.subtitle = Some(value.into());
        self
    }

    pub fn timestamp(mut self, value: impl Into<StringValue>) -> Self {
        self.timestamp = Some(value.into());
        self
    }

    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn severity(mut self, value: impl Into<String>) -> Self {
        self.severity = Some(value.into());
        self
    }

    pub fn icon(mut self, value: impl Into<StringValue>) -> Self {
        self.icon = Some(value.into());
        self
    }

    pub fn content(mut self, widget: impl Into<Widget>) -> Self {
        self.content = Some(Box::new(widget.into()));
        self
    }

    pub fn action(mut self, action: Action) -> Self {
        self.action = Some(action);
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimelineGroup {
    pub group_id: String,
    pub title: Option<StringValue>,
    pub summary: Option<StringValue>,
    pub children: Children,
    pub collapsed: Option<BoolValue>,
    pub badge_count: Option<NumberValue>,
    pub group_state: Option<String>,
}

impl TimelineGroup {
    pub fn new(group_id: impl Into<String>, children: Children) -> Self {
        Self {
            group_id: group_id.into(),
            title: None,
            summary: None,
            children,
            collapsed: None,
            badge_count: None,
            group_state: None,
        }
    }

    pub fn title(mut self, value: impl Into<StringValue>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn summary(mut self, value: impl Into<StringValue>) -> Self {
        self.summary = Some(value.into());
        self
    }

    pub fn collapsed(mut self, value: impl Into<BoolValue>) -> Self {
        self.collapsed = Some(value.into());
        self
    }

    pub fn badge_count(mut self, value: impl Into<NumberValue>) -> Self {
        self.badge_count = Some(value.into());
        self
    }

    pub fn group_state(mut self, value: impl Into<String>) -> Self {
        self.group_state = Some(value.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimelineLane {
    pub lane_id: String,
    pub title: Option<StringValue>,
    pub children: Children,
}

impl TimelineLane {
    pub fn new(lane_id: impl Into<String>, children: Children) -> Self {
        Self {
            lane_id: lane_id.into(),
            title: None,
            children,
        }
    }

    pub fn title(mut self, value: impl Into<StringValue>) -> Self {
        self.title = Some(value.into());
        self
    }
}
