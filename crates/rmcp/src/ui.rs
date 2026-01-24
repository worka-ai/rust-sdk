//! Ergonomic UI builders that serialize to A2UI.
//!
//! These types are fully typed and avoid exposing the low-level A2UI structs.

use crate::a2ui::{
    Action, ActionContextEntry, ActionValue, AudioPlayerProps, BoolRef, ButtonProps, CardProps,
    CheckBoxProps, ChoiceOption as A2ChoiceOption, ColumnProps, ComponentChildren, ComponentEntry,
    ComponentKind,
    ComponentTemplate, DateTimeInputProps, DividerProps, IconProps, ImageProps, ListProps,
    ModalProps, MultipleChoiceProps, NumberRef, RowProps, SliderProps, StringArrayRef, StringRef,
    TabItem as A2TabItem, TabsProps, TextFieldProps, TextProps, TimelineGroupProps, TimelineItemProps,
    TimelineLaneProps, TimelineProps, VideoProps,
};

#[derive(Debug, Clone, PartialEq)]
pub enum UiString {
    Literal(String),
    Path(String),
}

impl UiString {
    fn to_ref(&self) -> StringRef {
        match self {
            UiString::Literal(value) => StringRef::literal(value.clone()),
            UiString::Path(path) => StringRef::path(path.clone()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UiNumber {
    Literal(f64),
    Path(String),
}

impl UiNumber {
    fn to_ref(&self) -> NumberRef {
        match self {
            UiNumber::Literal(value) => NumberRef::literal(*value),
            UiNumber::Path(path) => NumberRef::path(path.clone()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UiBool {
    Literal(bool),
    Path(String),
}

impl UiBool {
    fn to_ref(&self) -> BoolRef {
        match self {
            UiBool::Literal(value) => BoolRef::literal(*value),
            UiBool::Path(path) => BoolRef::path(path.clone()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UiStringArray {
    Literal(Vec<String>),
    Path(String),
}

impl UiStringArray {
    fn to_ref(&self) -> StringArrayRef {
        match self {
            UiStringArray::Literal(values) => StringArrayRef::literal(values.clone()),
            UiStringArray::Path(path) => StringArrayRef::path(path.clone()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UiActionValue {
    Path(String),
    String(String),
    Number(f64),
    Bool(bool),
}

impl UiActionValue {
    fn to_value(&self) -> ActionValue {
        match self {
            UiActionValue::Path(path) => ActionValue::path(path.clone()),
            UiActionValue::String(value) => ActionValue::literal_string(value.clone()),
            UiActionValue::Number(value) => ActionValue::literal_number(*value),
            UiActionValue::Bool(value) => ActionValue::literal_boolean(*value),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UiActionContextEntry {
    pub key: String,
    pub value: UiActionValue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UiAction {
    pub name: String,
    pub context: Vec<UiActionContextEntry>,
}

impl UiAction {
    fn to_action(&self) -> Action {
        if self.context.is_empty() {
            Action::new(self.name.clone())
        } else {
            let entries = self
                .context
                .iter()
                .map(|entry| ActionContextEntry {
                    key: entry.key.clone(),
                    value: entry.value.to_value(),
                })
                .collect();
            Action::new(self.name.clone()).with_context(entries)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UiChildren {
    Items(Vec<Widget>),
    Template { data_binding: String, template: Box<Widget> },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Widget {
    pub id: Option<String>,
    pub weight: Option<f64>,
    pub kind: WidgetKind,
}

impl Widget {
    pub fn new(kind: WidgetKind) -> Self {
        Self { id: None, weight: None, kind }
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn with_weight(mut self, weight: f64) -> Self {
        self.weight = Some(weight);
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Text {
    pub text: UiString,
    pub usage_hint: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    pub url: UiString,
    pub fit: Option<String>,
    pub usage_hint: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Icon {
    pub name: UiString,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Divider {
    pub axis: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    pub children: UiChildren,
    pub distribution: Option<String>,
    pub alignment: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Column {
    pub children: UiChildren,
    pub distribution: Option<String>,
    pub alignment: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct List {
    pub children: UiChildren,
    pub direction: Option<String>,
    pub alignment: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Button {
    pub child: Box<Widget>,
    pub action: UiAction,
    pub primary: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextField {
    pub text: Option<UiString>,
    pub label: Option<UiString>,
    pub text_field_type: Option<String>,
    pub validation_regexp: Option<String>,
    pub on_submitted_action: Option<UiAction>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CheckBox {
    pub label: UiString,
    pub value: UiBool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Card {
    pub child: Box<Widget>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Modal {
    pub entry_point: Box<Widget>,
    pub content: Box<Widget>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TabItem {
    pub title: UiString,
    pub child: Widget,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tabs {
    pub tab_items: Vec<TabItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChoiceOption {
    pub label: UiString,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MultipleChoice {
    pub selections: UiStringArray,
    pub options: Vec<ChoiceOption>,
    pub max_allowed_selections: Option<i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Slider {
    pub value: UiNumber,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DateTimeInput {
    pub value: UiString,
    pub enable_date: Option<bool>,
    pub enable_time: Option<bool>,
    pub first_date: Option<String>,
    pub last_date: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AudioPlayer {
    pub url: UiString,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Video {
    pub url: UiString,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Timeline {
    pub children: UiChildren,
    pub orientation: Option<String>,
    pub alignment: Option<String>,
    pub auto_follow: Option<UiBool>,
    pub lane_mode: Option<String>,
    pub current_item_id: Option<UiString>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimelineItem {
    pub item_id: String,
    pub title: Option<UiString>,
    pub subtitle: Option<UiString>,
    pub timestamp: Option<UiString>,
    pub kind: Option<String>,
    pub state: Option<String>,
    pub severity: Option<String>,
    pub icon: Option<UiString>,
    pub content: Option<Box<Widget>>,
    pub action: Option<UiAction>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimelineGroup {
    pub group_id: String,
    pub title: Option<UiString>,
    pub summary: Option<UiString>,
    pub children: UiChildren,
    pub collapsed: Option<UiBool>,
    pub badge_count: Option<UiNumber>,
    pub group_state: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimelineLane {
    pub lane_id: String,
    pub title: Option<UiString>,
    pub children: UiChildren,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WidgetKind {
    Text(Text),
    Image(Image),
    Icon(Icon),
    Divider(Divider),
    Row(Row),
    Column(Column),
    List(List),
    Button(Button),
    TextField(TextField),
    CheckBox(CheckBox),
    Card(Card),
    Modal(Modal),
    Tabs(Tabs),
    MultipleChoice(MultipleChoice),
    Slider(Slider),
    DateTimeInput(DateTimeInput),
    AudioPlayer(AudioPlayer),
    Video(Video),
    Timeline(Timeline),
    TimelineItem(TimelineItem),
    TimelineGroup(TimelineGroup),
    TimelineLane(TimelineLane),
}

pub struct UiRender {
    pub root: String,
    pub components: Vec<ComponentEntry>,
}

pub fn render(root: Widget) -> UiRender {
    let mut serializer = UiSerializer::new();
    let root_id = serializer.render_widget(&root);
    UiRender { root: root_id, components: serializer.components }
}

struct UiSerializer {
    counter: usize,
    components: Vec<ComponentEntry>,
}

impl UiSerializer {
    fn new() -> Self {
        Self { counter: 0, components: Vec::new() }
    }

    fn next_id(&mut self) -> String {
        self.counter += 1;
        format!("ui-{}", self.counter)
    }

    fn render_children(&mut self, children: &UiChildren) -> ComponentChildren {
        match children {
            UiChildren::Items(items) => {
                let ids = items.iter().map(|child| self.render_widget(child)).collect();
                ComponentChildren::explicit(ids)
            }
            UiChildren::Template { data_binding, template } => {
                let template_id = self.render_widget(template);
                ComponentChildren {
                    explicit_list: None,
                    template: Some(ComponentTemplate {
                        component_id: template_id,
                        data_binding: data_binding.clone(),
                    }),
                }
            }
        }
    }

    fn render_widget(&mut self, widget: &Widget) -> String {
        let id = widget.id.clone().unwrap_or_else(|| self.next_id());
        let component = self.render_kind(&widget.kind);
        let mut entry = ComponentEntry::new(id.clone(), component);
        if let Some(weight) = widget.weight {
            entry = entry.with_weight(weight);
        }
        self.components.push(entry);
        id
    }

    fn render_kind(&mut self, kind: &WidgetKind) -> ComponentKind {
        match kind {
            WidgetKind::Text(text) => ComponentKind::Text(TextProps {
                text: text.text.to_ref(),
                usage_hint: text.usage_hint.clone(),
            }),
            WidgetKind::Image(image) => ComponentKind::Image(ImageProps {
                url: image.url.to_ref(),
                fit: image.fit.clone(),
                usage_hint: image.usage_hint.clone(),
            }),
            WidgetKind::Icon(icon) => ComponentKind::Icon(IconProps { name: icon.name.to_ref() }),
            WidgetKind::Divider(divider) => ComponentKind::Divider(DividerProps { axis: divider.axis.clone() }),
            WidgetKind::Row(row) => ComponentKind::Row(RowProps {
                children: self.render_children(&row.children),
                distribution: row.distribution.clone(),
                alignment: row.alignment.clone(),
            }),
            WidgetKind::Column(column) => ComponentKind::Column(ColumnProps {
                children: self.render_children(&column.children),
                distribution: column.distribution.clone(),
                alignment: column.alignment.clone(),
            }),
            WidgetKind::List(list) => ComponentKind::List(ListProps {
                children: self.render_children(&list.children),
                direction: list.direction.clone(),
                alignment: list.alignment.clone(),
            }),
            WidgetKind::Button(button) => {
                let child_id = self.render_widget(&button.child);
                ComponentKind::Button(ButtonProps {
                    child: child_id,
                    action: button.action.to_action(),
                    primary: button.primary,
                })
            }
            WidgetKind::TextField(field) => ComponentKind::TextField(TextFieldProps {
                text: field.text.as_ref().map(UiString::to_ref),
                label: field.label.as_ref().map(UiString::to_ref),
                text_field_type: field.text_field_type.clone(),
                validation_regexp: field.validation_regexp.clone(),
                on_submitted_action: field.on_submitted_action.as_ref().map(UiAction::to_action),
            }),
            WidgetKind::CheckBox(check) => ComponentKind::CheckBox(CheckBoxProps {
                label: check.label.to_ref(),
                value: check.value.to_ref(),
            }),
            WidgetKind::Card(card) => {
                let child_id = self.render_widget(&card.child);
                ComponentKind::Card(CardProps { child: child_id })
            }
            WidgetKind::Modal(modal) => {
                let entry_id = self.render_widget(&modal.entry_point);
                let content_id = self.render_widget(&modal.content);
                ComponentKind::Modal(ModalProps {
                    entry_point_child: entry_id,
                    content_child: content_id,
                })
            }
            WidgetKind::Tabs(tabs) => {
                let items = tabs
                    .tab_items
                    .iter()
                    .map(|item| {
                        let child_id = self.render_widget(&item.child);
                        A2TabItem {
                            title: item.title.to_ref(),
                            child: child_id,
                        }
                    })
                    .collect();
                ComponentKind::Tabs(TabsProps { tab_items: items })
            }
            WidgetKind::MultipleChoice(choice) => {
                let opts = choice
                    .options
                    .iter()
                    .map(|option| A2ChoiceOption {
                        label: option.label.to_ref(),
                        value: option.value.clone(),
                    })
                    .collect();
                ComponentKind::MultipleChoice(MultipleChoiceProps {
                    selections: choice.selections.to_ref(),
                    options: opts,
                    max_allowed_selections: choice.max_allowed_selections,
                })
            }
            WidgetKind::Slider(slider) => ComponentKind::Slider(SliderProps {
                value: slider.value.to_ref(),
                min_value: slider.min_value,
                max_value: slider.max_value,
            }),
            WidgetKind::DateTimeInput(input) => ComponentKind::DateTimeInput(DateTimeInputProps {
                value: input.value.to_ref(),
                enable_date: input.enable_date,
                enable_time: input.enable_time,
                first_date: input.first_date.clone(),
                last_date: input.last_date.clone(),
            }),
            WidgetKind::AudioPlayer(audio) => {
                ComponentKind::AudioPlayer(AudioPlayerProps { url: audio.url.to_ref() })
            }
            WidgetKind::Video(video) => ComponentKind::Video(VideoProps { url: video.url.to_ref() }),
            WidgetKind::Timeline(timeline) => ComponentKind::Timeline(TimelineProps {
                children: self.render_children(&timeline.children),
                orientation: timeline.orientation.clone(),
                alignment: timeline.alignment.clone(),
                auto_follow: timeline.auto_follow.as_ref().map(UiBool::to_ref),
                lane_mode: timeline.lane_mode.clone(),
                current_item_id: timeline.current_item_id.as_ref().map(UiString::to_ref),
            }),
            WidgetKind::TimelineItem(item) => {
                let content_child = item.content.as_ref().map(|child| self.render_widget(child));
                ComponentKind::TimelineItem(TimelineItemProps {
                    item_id: item.item_id.clone(),
                    title: item.title.as_ref().map(UiString::to_ref),
                    subtitle: item.subtitle.as_ref().map(UiString::to_ref),
                    timestamp: item.timestamp.as_ref().map(UiString::to_ref),
                    kind: item.kind.clone(),
                    state: item.state.clone(),
                    severity: item.severity.clone(),
                    icon: item.icon.as_ref().map(UiString::to_ref),
                    content_child,
                    action: item.action.as_ref().map(UiAction::to_action),
                })
            }
            WidgetKind::TimelineGroup(group) => ComponentKind::TimelineGroup(TimelineGroupProps {
                group_id: group.group_id.clone(),
                title: group.title.as_ref().map(UiString::to_ref),
                summary: group.summary.as_ref().map(UiString::to_ref),
                children: self.render_children(&group.children),
                collapsed: group.collapsed.as_ref().map(UiBool::to_ref),
                badge_count: group.badge_count.as_ref().map(UiNumber::to_ref),
                group_state: group.group_state.clone(),
            }),
            WidgetKind::TimelineLane(lane) => ComponentKind::TimelineLane(TimelineLaneProps {
                lane_id: lane.lane_id.clone(),
                title: lane.title.as_ref().map(UiString::to_ref),
                children: self.render_children(&lane.children),
            }),
        }
    }
}
