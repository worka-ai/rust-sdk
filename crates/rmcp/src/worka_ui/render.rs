use crate::worka_ui::children::Children;
use crate::worka_ui::widgets::Widget;
use crate::worka_ui::wire;

/// A rendered widget tree ready to be embedded in a pack view payload.
///
/// This is the "document" form expected by the Flutter host:
/// `{ "components": [...], "root": "..." }`.
#[derive(Debug, Clone, PartialEq)]
pub struct RenderedTree {
    pub root: String,
    pub components: Vec<serde_json::Value>,
}

impl RenderedTree {
    pub fn to_value(&self) -> serde_json::Value {
        serde_json::json!({
            "components": self.components,
            "root": self.root,
        })
    }
}

/// Render a widget tree into A2UI `components` + `root`.
pub fn render(root: impl Into<Widget>) -> RenderedTree {
    let rendered = render_wire(root.into());
    let components = rendered
        .components
        .into_iter()
        .map(|entry| serde_json::to_value(entry).unwrap_or(serde_json::Value::Null))
        .collect();
    RenderedTree { root: rendered.root, components }
}

/// Convenience wrapper for packs that want the message-list form:
/// `[ {surfaceUpdate: {...}}, {dataModelUpdate: {...}}, {beginRendering: {...}} ]`
#[derive(Debug, Clone)]
pub struct Surface {
    surface_id: String,
    root: Widget,
    data_model: Option<serde_json::Value>,
}

impl Surface {
    pub fn new(surface_id: impl Into<String>, root: impl Into<Widget>) -> Self {
        Self { surface_id: surface_id.into(), root: root.into(), data_model: None }
    }

    pub fn data_model(mut self, contents: serde_json::Value) -> Self {
        self.data_model = Some(contents);
        self
    }

    pub fn to_value(self) -> serde_json::Value {
        let wire = self.to_wire_messages();
        serde_json::to_value(wire).unwrap_or(serde_json::Value::Array(vec![]))
    }

    fn to_wire_messages(self) -> Vec<wire::A2uiMessage> {
        let WireTree { root, components } = render_wire(self.root);

        let mut out = Vec::new();
        out.push(wire::A2uiMessage::SurfaceUpdate(wire::SurfaceUpdate {
            surface_id: self.surface_id.clone(),
            components,
        }));

        if let Some(contents) = self.data_model {
            out.push(wire::A2uiMessage::DataModelUpdate(wire::DataModelUpdate {
                surface_id: self.surface_id.clone(),
                path: None,
                contents,
            }));
        }

        out.push(wire::A2uiMessage::BeginRendering(wire::BeginRendering {
            surface_id: self.surface_id,
            root,
            catalog_id: None,
            styles: None,
        }));

        out
    }
}

#[derive(Debug)]
struct WireTree {
    root: String,
    components: Vec<wire::ComponentEntry>,
}

fn render_wire(root: Widget) -> WireTree {
    let mut renderer = Renderer::new();
    let root_id = renderer.render_widget(root);
    WireTree { root: root_id, components: renderer.components }
}

struct Renderer {
    next_id: u64,
    components: Vec<wire::ComponentEntry>,
}

impl Renderer {
    fn new() -> Self {
        Self { next_id: 0, components: Vec::new() }
    }

    fn alloc_id(&mut self) -> String {
        let id = format!("w{}", self.next_id);
        self.next_id += 1;
        id
    }

    fn render_children(&mut self, children: Children) -> wire::ComponentChildren {
        match children {
            Children::Items(items) => {
                let ids = items.into_iter().map(|w| self.render_widget(w)).collect();
                wire::ComponentChildren::explicit(ids)
            }
            Children::Template { data_binding, template } => {
                let template_id = self.render_widget(*template);
                wire::ComponentChildren::template(template_id, data_binding)
            }
        }
    }

    fn render_widget(&mut self, widget: Widget) -> String {
        match widget {
            Widget::Text(widget) => {
                let id = self.alloc_id();
                self.components.push(wire::ComponentEntry::new(
                    id.clone(),
                    wire::ComponentKind::Text(wire::TextProps {
                        text: widget.text.to_ref(),
                        usage_hint: widget.usage_hint,
                    }),
                ));
                id
            }
            Widget::Image(widget) => {
                let id = self.alloc_id();
                self.components.push(wire::ComponentEntry::new(
                    id.clone(),
                    wire::ComponentKind::Image(wire::ImageProps {
                        url: widget.url.to_ref(),
                        fit: widget.fit,
                        usage_hint: widget.usage_hint,
                    }),
                ));
                id
            }
            Widget::Icon(widget) => {
                let id = self.alloc_id();
                self.components.push(wire::ComponentEntry::new(
                    id.clone(),
                    wire::ComponentKind::Icon(wire::IconProps {
                        name: widget.name.to_ref(),
                    }),
                ));
                id
            }
            Widget::Divider(widget) => {
                let id = self.alloc_id();
                self.components.push(wire::ComponentEntry::new(
                    id.clone(),
                    wire::ComponentKind::Divider(wire::DividerProps { axis: widget.axis }),
                ));
                id
            }
            Widget::Row(widget) => {
                let children = self.render_children(widget.children);
                let id = self.alloc_id();
                self.components.push(wire::ComponentEntry::new(
                    id.clone(),
                    wire::ComponentKind::Row(wire::RowProps {
                        children,
                        distribution: widget.distribution,
                        alignment: widget.alignment,
                    }),
                ));
                id
            }
            Widget::Column(widget) => {
                let children = self.render_children(widget.children);
                let id = self.alloc_id();
                self.components.push(wire::ComponentEntry::new(
                    id.clone(),
                    wire::ComponentKind::Column(wire::ColumnProps {
                        children,
                        distribution: widget.distribution,
                        alignment: widget.alignment,
                    }),
                ));
                id
            }
            Widget::List(widget) => {
                let children = self.render_children(widget.children);
                let id = self.alloc_id();
                self.components.push(wire::ComponentEntry::new(
                    id.clone(),
                    wire::ComponentKind::List(wire::ListProps {
                        children,
                        direction: widget.direction,
                        alignment: widget.alignment,
                    }),
                ));
                id
            }
            Widget::Button(widget) => {
                let child_id = self.render_widget(*widget.child);
                let id = self.alloc_id();
                self.components.push(wire::ComponentEntry::new(
                    id.clone(),
                    wire::ComponentKind::Button(wire::ButtonProps {
                        child: child_id,
                        action: widget.action.to_wire(),
                        primary: widget.primary,
                    }),
                ));
                id
            }
            Widget::TextField(widget) => {
                let id = self.alloc_id();
                self.components.push(wire::ComponentEntry::new(
                    id.clone(),
                    wire::ComponentKind::TextField(wire::TextFieldProps {
                        text: widget.text.map(|t| t.to_ref()),
                        label: widget.label.map(|t| t.to_ref()),
                        text_field_type: widget.text_field_type,
                        validation_regexp: widget.validation_regexp,
                        on_submitted_action: widget.on_submitted_action.map(|a| a.to_wire()),
                    }),
                ));
                id
            }
            Widget::CheckBox(widget) => {
                let id = self.alloc_id();
                self.components.push(wire::ComponentEntry::new(
                    id.clone(),
                    wire::ComponentKind::CheckBox(wire::CheckBoxProps {
                        label: widget.label.to_ref(),
                        value: widget.value.to_ref(),
                    }),
                ));
                id
            }
            Widget::Card(widget) => {
                let child_id = self.render_widget(*widget.child);
                let id = self.alloc_id();
                self.components.push(wire::ComponentEntry::new(
                    id.clone(),
                    wire::ComponentKind::Card(wire::CardProps { child: child_id }),
                ));
                id
            }
            Widget::Modal(widget) => {
                let entry_id = self.render_widget(*widget.entry_point);
                let content_id = self.render_widget(*widget.content);
                let id = self.alloc_id();
                self.components.push(wire::ComponentEntry::new(
                    id.clone(),
                    wire::ComponentKind::Modal(wire::ModalProps {
                        entry_point_child: entry_id,
                        content_child: content_id,
                    }),
                ));
                id
            }
            Widget::Tabs(widget) => {
                let tab_items = widget
                    .tab_items
                    .into_iter()
                    .map(|item| wire::TabItem {
                        title: item.title.to_ref(),
                        child: self.render_widget(item.child),
                    })
                    .collect();
                let id = self.alloc_id();
                self.components.push(wire::ComponentEntry::new(
                    id.clone(),
                    wire::ComponentKind::Tabs(wire::TabsProps { tab_items }),
                ));
                id
            }
            Widget::MultipleChoice(widget) => {
                let options = widget
                    .options
                    .into_iter()
                    .map(|option| wire::ChoiceOption {
                        label: option.label.to_ref(),
                        value: option.value,
                    })
                    .collect();
                let id = self.alloc_id();
                self.components.push(wire::ComponentEntry::new(
                    id.clone(),
                    wire::ComponentKind::MultipleChoice(wire::MultipleChoiceProps {
                        selections: widget.selections.to_ref(),
                        options,
                        max_allowed_selections: widget.max_allowed_selections,
                    }),
                ));
                id
            }
            Widget::Slider(widget) => {
                let id = self.alloc_id();
                self.components.push(wire::ComponentEntry::new(
                    id.clone(),
                    wire::ComponentKind::Slider(wire::SliderProps {
                        value: widget.value.to_ref(),
                        min_value: widget.min_value,
                        max_value: widget.max_value,
                    }),
                ));
                id
            }
            Widget::DateTimeInput(widget) => {
                let id = self.alloc_id();
                self.components.push(wire::ComponentEntry::new(
                    id.clone(),
                    wire::ComponentKind::DateTimeInput(wire::DateTimeInputProps {
                        value: widget.value.to_ref(),
                        enable_date: widget.enable_date,
                        enable_time: widget.enable_time,
                        first_date: widget.first_date,
                        last_date: widget.last_date,
                    }),
                ));
                id
            }
            Widget::AudioPlayer(widget) => {
                let id = self.alloc_id();
                self.components.push(wire::ComponentEntry::new(
                    id.clone(),
                    wire::ComponentKind::AudioPlayer(wire::AudioPlayerProps {
                        url: widget.url.to_ref(),
                    }),
                ));
                id
            }
            Widget::Video(widget) => {
                let id = self.alloc_id();
                self.components.push(wire::ComponentEntry::new(
                    id.clone(),
                    wire::ComponentKind::Video(wire::VideoProps {
                        url: widget.url.to_ref(),
                    }),
                ));
                id
            }
            Widget::Timeline(widget) => {
                let children = self.render_children(widget.children);
                let id = self.alloc_id();
                self.components.push(wire::ComponentEntry::new(
                    id.clone(),
                    wire::ComponentKind::Timeline(wire::TimelineProps {
                        children,
                        orientation: widget.orientation,
                        alignment: widget.alignment,
                        auto_follow: widget.auto_follow.map(|v| v.to_ref()),
                        lane_mode: widget.lane_mode,
                        current_item_id: widget.current_item_id.map(|v| v.to_ref()),
                    }),
                ));
                id
            }
            Widget::TimelineItem(widget) => {
                let content_child = widget.content.map(|child| self.render_widget(*child));
                let id = self.alloc_id();
                self.components.push(wire::ComponentEntry::new(
                    id.clone(),
                    wire::ComponentKind::TimelineItem(wire::TimelineItemProps {
                        item_id: widget.item_id,
                        title: widget.title.map(|v| v.to_ref()),
                        subtitle: widget.subtitle.map(|v| v.to_ref()),
                        timestamp: widget.timestamp.map(|v| v.to_ref()),
                        kind: widget.kind,
                        state: widget.state,
                        severity: widget.severity,
                        icon: widget.icon.map(|v| v.to_ref()),
                        content_child,
                        action: widget.action.map(|a| a.to_wire()),
                    }),
                ));
                id
            }
            Widget::TimelineGroup(widget) => {
                let children = self.render_children(widget.children);
                let id = self.alloc_id();
                self.components.push(wire::ComponentEntry::new(
                    id.clone(),
                    wire::ComponentKind::TimelineGroup(wire::TimelineGroupProps {
                        group_id: widget.group_id,
                        title: widget.title.map(|v| v.to_ref()),
                        summary: widget.summary.map(|v| v.to_ref()),
                        children,
                        collapsed: widget.collapsed.map(|v| v.to_ref()),
                        badge_count: widget.badge_count.map(|v| v.to_ref()),
                        group_state: widget.group_state,
                    }),
                ));
                id
            }
            Widget::TimelineLane(widget) => {
                let children = self.render_children(widget.children);
                let id = self.alloc_id();
                self.components.push(wire::ComponentEntry::new(
                    id.clone(),
                    wire::ComponentKind::TimelineLane(wire::TimelineLaneProps {
                        lane_id: widget.lane_id,
                        title: widget.title.map(|v| v.to_ref()),
                        children,
                    }),
                ));
                id
            }
        }
    }
}
