//! A2UI builder types for Worka packs.
//!
//! This module mirrors the default A2UI catalog supported by GenUI:
//! AudioPlayer, Button, Card, CheckBox, Column, DateTimeInput, Divider, Icon,
//! Image, List, Modal, MultipleChoice, Row, Slider, Tabs, TextField, Text, Video.
//!
//! It provides typed component props and JSON serialization via serde.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StringRef {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "literalString", skip_serializing_if = "Option::is_none")]
    pub literal_string: Option<String>,
}

impl StringRef {
    pub fn literal(value: impl Into<String>) -> Self {
        Self { path: None, literal_string: Some(value.into()) }
    }

    pub fn path(path: impl Into<String>) -> Self {
        Self { path: Some(path.into()), literal_string: None }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NumberRef {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "literalNumber", skip_serializing_if = "Option::is_none")]
    pub literal_number: Option<f64>,
}

impl NumberRef {
    pub fn literal(value: f64) -> Self {
        Self { path: None, literal_number: Some(value) }
    }

    pub fn path(path: impl Into<String>) -> Self {
        Self { path: Some(path.into()), literal_number: None }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoolRef {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "literalBoolean", skip_serializing_if = "Option::is_none")]
    pub literal_boolean: Option<bool>,
}

impl BoolRef {
    pub fn literal(value: bool) -> Self {
        Self { path: None, literal_boolean: Some(value) }
    }

    pub fn path(path: impl Into<String>) -> Self {
        Self { path: Some(path.into()), literal_boolean: None }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StringArrayRef {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "literalArray", skip_serializing_if = "Option::is_none")]
    pub literal_array: Option<Vec<String>>,
}

impl StringArrayRef {
    pub fn literal(values: Vec<String>) -> Self {
        Self { path: None, literal_array: Some(values) }
    }

    pub fn path(path: impl Into<String>) -> Self {
        Self { path: Some(path.into()), literal_array: None }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "literalString", skip_serializing_if = "Option::is_none")]
    pub literal_string: Option<String>,
    #[serde(rename = "literalNumber", skip_serializing_if = "Option::is_none")]
    pub literal_number: Option<f64>,
    #[serde(rename = "literalBoolean", skip_serializing_if = "Option::is_none")]
    pub literal_boolean: Option<bool>,
}

impl ActionValue {
    pub fn path(path: impl Into<String>) -> Self {
        Self {
            path: Some(path.into()),
            literal_string: None,
            literal_number: None,
            literal_boolean: None,
        }
    }

    pub fn literal_string(value: impl Into<String>) -> Self {
        Self {
            path: None,
            literal_string: Some(value.into()),
            literal_number: None,
            literal_boolean: None,
        }
    }

    pub fn literal_number(value: f64) -> Self {
        Self {
            path: None,
            literal_string: None,
            literal_number: Some(value),
            literal_boolean: None,
        }
    }

    pub fn literal_boolean(value: bool) -> Self {
        Self {
            path: None,
            literal_string: None,
            literal_number: None,
            literal_boolean: Some(value),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionContextEntry {
    pub key: String,
    pub value: ActionValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Action {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Vec<ActionContextEntry>>,
}

impl Action {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), context: None }
    }

    pub fn with_context(mut self, entries: Vec<ActionContextEntry>) -> Self {
        self.context = Some(entries);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentTemplate {
    #[serde(rename = "componentId")]
    pub component_id: String,
    #[serde(rename = "dataBinding")]
    pub data_binding: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentChildren {
    #[serde(rename = "explicitList", skip_serializing_if = "Option::is_none")]
    pub explicit_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<ComponentTemplate>,
}

impl ComponentChildren {
    pub fn explicit(ids: Vec<String>) -> Self {
        Self { explicit_list: Some(ids), template: None }
    }

    pub fn template(component_id: impl Into<String>, data_binding: impl Into<String>) -> Self {
        Self {
            explicit_list: None,
            template: Some(ComponentTemplate {
                component_id: component_id.into(),
                data_binding: data_binding.into(),
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentEntry {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
    pub component: ComponentKind,
}

impl ComponentEntry {
    pub fn new(id: impl Into<String>, component: ComponentKind) -> Self {
        Self { id: id.into(), weight: None, component }
    }

    pub fn with_weight(mut self, weight: f64) -> Self {
        self.weight = Some(weight);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ComponentKind {
    #[serde(rename = "AudioPlayer")]
    AudioPlayer(AudioPlayerProps),
    #[serde(rename = "Button")]
    Button(ButtonProps),
    #[serde(rename = "Card")]
    Card(CardProps),
    #[serde(rename = "CheckBox")]
    CheckBox(CheckBoxProps),
    #[serde(rename = "Column")]
    Column(ColumnProps),
    #[serde(rename = "DateTimeInput")]
    DateTimeInput(DateTimeInputProps),
    #[serde(rename = "Divider")]
    Divider(DividerProps),
    #[serde(rename = "Icon")]
    Icon(IconProps),
    #[serde(rename = "Image")]
    Image(ImageProps),
    #[serde(rename = "List")]
    List(ListProps),
    #[serde(rename = "Modal")]
    Modal(ModalProps),
    #[serde(rename = "MultipleChoice")]
    MultipleChoice(MultipleChoiceProps),
    #[serde(rename = "Row")]
    Row(RowProps),
    #[serde(rename = "Slider")]
    Slider(SliderProps),
    #[serde(rename = "Tabs")]
    Tabs(TabsProps),
    #[serde(rename = "TextField")]
    TextField(TextFieldProps),
    #[serde(rename = "Text")]
    Text(TextProps),
    #[serde(rename = "Video")]
    Video(VideoProps),
    #[serde(rename = "Timeline")]
    Timeline(TimelineProps),
    #[serde(rename = "TimelineItem")]
    TimelineItem(TimelineItemProps),
    #[serde(rename = "TimelineGroup")]
    TimelineGroup(TimelineGroupProps),
    #[serde(rename = "TimelineLane")]
    TimelineLane(TimelineLaneProps),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RowProps {
    pub children: ComponentChildren,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColumnProps {
    pub children: ComponentChildren,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListProps {
    pub children: ComponentChildren,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextProps {
    pub text: StringRef,
    #[serde(rename = "usageHint", skip_serializing_if = "Option::is_none")]
    pub usage_hint: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageProps {
    pub url: StringRef,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fit: Option<String>,
    #[serde(rename = "usageHint", skip_serializing_if = "Option::is_none")]
    pub usage_hint: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IconProps {
    pub name: StringRef,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DividerProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ButtonProps {
    pub child: String,
    pub action: Action,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextFieldProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<StringRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<StringRef>,
    #[serde(rename = "textFieldType", skip_serializing_if = "Option::is_none")]
    pub text_field_type: Option<String>,
    #[serde(rename = "validationRegexp", skip_serializing_if = "Option::is_none")]
    pub validation_regexp: Option<String>,
    #[serde(rename = "onSubmittedAction", skip_serializing_if = "Option::is_none")]
    pub on_submitted_action: Option<Action>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckBoxProps {
    pub label: StringRef,
    pub value: BoolRef,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CardProps {
    pub child: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModalProps {
    #[serde(rename = "entryPointChild")]
    pub entry_point_child: String,
    #[serde(rename = "contentChild")]
    pub content_child: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TabItem {
    pub title: StringRef,
    pub child: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TabsProps {
    #[serde(rename = "tabItems")]
    pub tab_items: Vec<TabItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChoiceOption {
    pub label: StringRef,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultipleChoiceProps {
    pub selections: StringArrayRef,
    pub options: Vec<ChoiceOption>,
    #[serde(rename = "maxAllowedSelections", skip_serializing_if = "Option::is_none")]
    pub max_allowed_selections: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SliderProps {
    pub value: NumberRef,
    #[serde(rename = "minValue", skip_serializing_if = "Option::is_none")]
    pub min_value: Option<f64>,
    #[serde(rename = "maxValue", skip_serializing_if = "Option::is_none")]
    pub max_value: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DateTimeInputProps {
    pub value: StringRef,
    #[serde(rename = "enableDate", skip_serializing_if = "Option::is_none")]
    pub enable_date: Option<bool>,
    #[serde(rename = "enableTime", skip_serializing_if = "Option::is_none")]
    pub enable_time: Option<bool>,
    #[serde(rename = "firstDate", skip_serializing_if = "Option::is_none")]
    pub first_date: Option<String>,
    #[serde(rename = "lastDate", skip_serializing_if = "Option::is_none")]
    pub last_date: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioPlayerProps {
    pub url: StringRef,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoProps {
    pub url: StringRef,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimelineProps {
    pub children: ComponentChildren,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<String>,
    #[serde(rename = "autoFollow", skip_serializing_if = "Option::is_none")]
    pub auto_follow: Option<BoolRef>,
    #[serde(rename = "laneMode", skip_serializing_if = "Option::is_none")]
    pub lane_mode: Option<String>,
    #[serde(rename = "currentItemId", skip_serializing_if = "Option::is_none")]
    pub current_item_id: Option<StringRef>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimelineItemProps {
    #[serde(rename = "itemId")]
    pub item_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<StringRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<StringRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<StringRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<StringRef>,
    #[serde(rename = "contentChild", skip_serializing_if = "Option::is_none")]
    pub content_child: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimelineGroupProps {
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<StringRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<StringRef>,
    pub children: ComponentChildren,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collapsed: Option<BoolRef>,
    #[serde(rename = "badgeCount", skip_serializing_if = "Option::is_none")]
    pub badge_count: Option<NumberRef>,
    #[serde(rename = "groupState", skip_serializing_if = "Option::is_none")]
    pub group_state: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimelineLaneProps {
    #[serde(rename = "laneId")]
    pub lane_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<StringRef>,
    pub children: ComponentChildren,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SurfaceUpdate {
    #[serde(rename = "surfaceId")]
    pub surface_id: String,
    pub components: Vec<ComponentEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SurfaceStyles {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    #[serde(rename = "primaryColor", skip_serializing_if = "Option::is_none")]
    pub primary_color: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BeginRendering {
    #[serde(rename = "surfaceId")]
    pub surface_id: String,
    pub root: String,
    #[serde(rename = "catalogId", skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub styles: Option<SurfaceStyles>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataModelUpdate {
    #[serde(rename = "surfaceId")]
    pub surface_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    pub contents: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteSurface {
    #[serde(rename = "surfaceId")]
    pub surface_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum A2uiMessage {
    #[serde(rename = "surfaceUpdate")]
    SurfaceUpdate(SurfaceUpdate),
    #[serde(rename = "beginRendering")]
    BeginRendering(BeginRendering),
    #[serde(rename = "dataModelUpdate")]
    DataModelUpdate(DataModelUpdate),
    #[serde(rename = "deleteSurface")]
    DeleteSurface(DeleteSurface),
}
