use rmcp::ui::*;
use serde_json::Value;

fn render_single(widget: Widget) -> Value {
    let rendered = render(widget);
    let root = rendered.root;
    let entry = rendered
        .components
        .into_iter()
        .find(|entry| entry.id == root)
        .expect("root component");
    serde_json::to_value(entry).expect("serialize component")
}

fn assert_component_key(value: &Value, expected: &str) {
    let component = value
        .get("component")
        .and_then(Value::as_object)
        .expect("component map");
    assert!(component.contains_key(expected), "missing component {expected}");
}

#[test]
fn serializes_high_level_widgets() {
    let text_widget = Widget::new(WidgetKind::Text(Text {
        text: UiString::Literal("Hello".into()),
        usage_hint: Some("h1".into()),
    }))
    .with_id("text");
    let text = render_single(text_widget);
    assert_eq!(
        text,
        serde_json::json!({"id":"text","component":{"Text":{"text":{"literalString":"Hello"},"usageHint":"h1"}}})
    );

    let row_widget = Widget::new(WidgetKind::Row(Row {
        children: UiChildren::Items(vec![
            Widget::new(WidgetKind::Text(Text { text: UiString::Literal("A".into()), usage_hint: None })).with_id("a"),
            Widget::new(WidgetKind::Text(Text { text: UiString::Literal("B".into()), usage_hint: None })).with_id("b"),
        ]),
        distribution: Some("spaceBetween".into()),
        alignment: Some("center".into()),
    }))
    .with_id("row");
    let row = render_single(row_widget);
    assert_eq!(
        row,
        serde_json::json!({"id":"row","component":{"Row":{"children":{"explicitList":["a","b"]},"distribution":"spaceBetween","alignment":"center"}}})
    );

    let button_widget = Widget::new(WidgetKind::Button(Button {
        child: Box::new(Widget::new(WidgetKind::Text(Text {
            text: UiString::Literal("Click".into()),
            usage_hint: None,
        }))),
        action: UiAction { name: "submit".into(), context: vec![] },
        primary: Some(true),
    }))
    .with_id("btn");
    let button = render_single(button_widget);
    assert_eq!(button["component"]["Button"]["primary"], serde_json::json!(true));

    let timeline_widget = Widget::new(WidgetKind::Timeline(Timeline {
        children: UiChildren::Items(vec![Widget::new(WidgetKind::TimelineItem(TimelineItem {
            item_id: Some("item-1".into()),
            title: Some(UiString::Literal("Deploy".into())),
            subtitle: None,
            timestamp: None,
            kind: Some("step".into()),
            state: None,
            severity: None,
            icon: None,
            content: None,
            action: None,
        }))
        .with_id("item")]),
        orientation: Some("vertical".into()),
        alignment: Some("start".into()),
        auto_follow: Some(UiBool::Literal(true)),
        lane_mode: None,
        current_item_id: None,
    }))
    .with_id("timeline");
    let timeline = render_single(timeline_widget);
    assert_eq!(timeline["component"]["Timeline"]["orientation"], serde_json::json!("vertical"));

    let widgets: Vec<(Widget, &str)> = vec![
        (
            Widget::new(WidgetKind::Image(Image { url: UiString::Literal("https://example.com".into()), fit: None, usage_hint: None })),
            "Image",
        ),
        (Widget::new(WidgetKind::Icon(Icon { name: UiString::Literal("check".into()) })), "Icon"),
        (Widget::new(WidgetKind::Divider(Divider { axis: Some("horizontal".into()) })), "Divider"),
        (Widget::new(WidgetKind::Column(Column { children: UiChildren::Items(vec![]), distribution: None, alignment: None })), "Column"),
        (Widget::new(WidgetKind::List(List { children: UiChildren::Items(vec![]), direction: Some("vertical".into()), alignment: None })), "List"),
        (
            Widget::new(WidgetKind::TextField(TextField {
                text: Some(UiString::Path("/text".into())),
                label: Some(UiString::Literal("Label".into())),
                text_field_type: Some("shortText".into()),
                validation_regexp: None,
                on_submitted_action: None,
            })),
            "TextField",
        ),
        (
            Widget::new(WidgetKind::CheckBox(CheckBox { label: UiString::Literal("Agree".into()), value: UiBool::Literal(true) })),
            "CheckBox",
        ),
        (
            Widget::new(WidgetKind::Card(Card {
                child: Box::new(Widget::new(WidgetKind::Text(Text { text: UiString::Literal("Card".into()), usage_hint: None }))),
            })),
            "Card",
        ),
        (
            Widget::new(WidgetKind::Modal(Modal {
                entry_point: Box::new(Widget::new(WidgetKind::Text(Text { text: UiString::Literal("Open".into()), usage_hint: None }))),
                content: Box::new(Widget::new(WidgetKind::Text(Text { text: UiString::Literal("Body".into()), usage_hint: None }))),
            })),
            "Modal",
        ),
        (
            Widget::new(WidgetKind::Tabs(Tabs {
                tab_items: vec![TabItem {
                    title: UiString::Literal("Tab".into()),
                    child: Widget::new(WidgetKind::Text(Text { text: UiString::Literal("Content".into()), usage_hint: None })),
                }],
            })),
            "Tabs",
        ),
        (
            Widget::new(WidgetKind::MultipleChoice(MultipleChoice {
                selections: UiStringArray::Path("/choices".into()),
                options: vec![ChoiceOption { label: UiString::Literal("One".into()), value: "1".into() }],
                max_allowed_selections: Some(1),
            })),
            "MultipleChoice",
        ),
        (
            Widget::new(WidgetKind::Slider(Slider { value: UiNumber::Literal(1.0), min_value: Some(0.0), max_value: Some(10.0) })),
            "Slider",
        ),
        (
            Widget::new(WidgetKind::DateTimeInput(DateTimeInput {
                value: UiString::Literal("2024-01-01".into()),
                enable_date: Some(true),
                enable_time: Some(false),
                first_date: Some("2024-01-01".into()),
                last_date: Some("2024-12-31".into()),
            })),
            "DateTimeInput",
        ),
        (
            Widget::new(WidgetKind::AudioPlayer(AudioPlayer { url: UiString::Literal("https://example.com/audio.mp3".into()) })),
            "AudioPlayer",
        ),
        (
            Widget::new(WidgetKind::Video(Video { url: UiString::Literal("https://example.com/video.mp4".into()) })),
            "Video",
        ),
        (
            Widget::new(WidgetKind::TimelineItem(TimelineItem {
                item_id: Some("item-1".into()),
                title: Some(UiString::Literal("Title".into())),
                subtitle: None,
                timestamp: None,
                kind: None,
                state: None,
                severity: None,
                icon: None,
                content: None,
                action: None,
            })),
            "TimelineItem",
        ),
        (
            Widget::new(WidgetKind::TimelineGroup(TimelineGroup {
                group_id: "group-1".into(),
                title: Some(UiString::Literal("Group".into())),
                summary: None,
                children: UiChildren::Items(vec![]),
                collapsed: None,
                badge_count: None,
                group_state: None,
            })),
            "TimelineGroup",
        ),
        (
            Widget::new(WidgetKind::TimelineLane(TimelineLane {
                lane_id: "lane-1".into(),
                title: Some(UiString::Literal("Lane".into())),
                children: UiChildren::Items(vec![]),
            })),
            "TimelineLane",
        ),
    ];

    for (widget, expected_key) in widgets {
        let value = render_single(widget.with_id("node"));
        assert_component_key(&value, expected_key);
    }
}
