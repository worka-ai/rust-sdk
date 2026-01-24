use rmcp::a2ui::*;
use serde_json::json;

#[test]
fn a2ui_widget_serialization_matches_genui_shapes() {
    let text = ComponentEntry::new(
        "text",
        ComponentKind::Text(TextProps {
            text: StringRef::literal("Hello"),
            usage_hint: Some("h1".into()),
        }),
    );
    assert_eq!(
        serde_json::to_value(&text).unwrap(),
        json!({
            "id": "text",
            "component": {
                "Text": {
                    "text": {"literalString": "Hello"},
                    "usageHint": "h1"
                }
            }
        })
    );

    let image = ComponentEntry::new(
        "img",
        ComponentKind::Image(ImageProps {
            url: StringRef::literal("https://example.com/logo.png"),
            fit: Some("cover".into()),
            usage_hint: Some("mediumFeature".into()),
        }),
    );
    assert_eq!(
        serde_json::to_value(&image).unwrap(),
        json!({
            "id": "img",
            "component": {
                "Image": {
                    "url": {"literalString": "https://example.com/logo.png"},
                    "fit": "cover",
                    "usageHint": "mediumFeature"
                }
            }
        })
    );

    let icon = ComponentEntry::new(
        "icon",
        ComponentKind::Icon(IconProps {
            name: StringRef::literal("check"),
        }),
    );
    assert_eq!(
        serde_json::to_value(&icon).unwrap(),
        json!({
            "id": "icon",
            "component": { "Icon": { "name": { "literalString": "check" } } }
        })
    );

    let divider = ComponentEntry::new(
        "divider",
        ComponentKind::Divider(DividerProps {
            axis: Some("horizontal".into()),
        }),
    );
    assert_eq!(
        serde_json::to_value(&divider).unwrap(),
        json!({
            "id": "divider",
            "component": { "Divider": { "axis": "horizontal" } }
        })
    );

    let row = ComponentEntry::new(
        "row",
        ComponentKind::Row(RowProps {
            children: ComponentChildren::explicit(vec!["a".into(), "b".into()]),
            distribution: Some("spaceBetween".into()),
            alignment: Some("center".into()),
        }),
    );
    assert_eq!(
        serde_json::to_value(&row).unwrap(),
        json!({
            "id": "row",
            "component": {
                "Row": {
                    "children": { "explicitList": ["a", "b"] },
                    "distribution": "spaceBetween",
                    "alignment": "center"
                }
            }
        })
    );

    let column = ComponentEntry::new(
        "column",
        ComponentKind::Column(ColumnProps {
            children: ComponentChildren::template("item", "/items"),
            distribution: Some("start".into()),
            alignment: Some("stretch".into()),
        }),
    );
    assert_eq!(
        serde_json::to_value(&column).unwrap(),
        json!({
            "id": "column",
            "component": {
                "Column": {
                    "children": { "template": { "componentId": "item", "dataBinding": "/items" } },
                    "distribution": "start",
                    "alignment": "stretch"
                }
            }
        })
    );

    let list = ComponentEntry::new(
        "list",
        ComponentKind::List(ListProps {
            children: ComponentChildren::explicit(vec!["one".into(), "two".into()]),
            direction: Some("vertical".into()),
            alignment: Some("start".into()),
        }),
    );
    assert_eq!(
        serde_json::to_value(&list).unwrap(),
        json!({
            "id": "list",
            "component": {
                "List": {
                    "children": { "explicitList": ["one", "two"] },
                    "direction": "vertical",
                    "alignment": "start"
                }
            }
        })
    );

    let button = ComponentEntry::new(
        "button",
        ComponentKind::Button(ButtonProps {
            child: "button-text".into(),
            action: Action::new("submit"),
            primary: Some(true),
        }),
    );
    assert_eq!(
        serde_json::to_value(&button).unwrap(),
        json!({
            "id": "button",
            "component": {
                "Button": {
                    "child": "button-text",
                    "action": { "name": "submit" },
                    "primary": true
                }
            }
        })
    );

    let text_field = ComponentEntry::new(
        "text-field",
        ComponentKind::TextField(TextFieldProps {
            text: Some(StringRef::path("/email")),
            label: Some(StringRef::literal("Email")),
            text_field_type: Some("shortText".into()),
            validation_regexp: Some("^.+@.+$".into()),
            on_submitted_action: Some(Action::new("submit_form")),
        }),
    );
    assert_eq!(
        serde_json::to_value(&text_field).unwrap(),
        json!({
            "id": "text-field",
            "component": {
                "TextField": {
                    "text": { "path": "/email" },
                    "label": { "literalString": "Email" },
                    "textFieldType": "shortText",
                    "validationRegexp": "^.+@.+$",
                    "onSubmittedAction": { "name": "submit_form" }
                }
            }
        })
    );

    let check_box = ComponentEntry::new(
        "check",
        ComponentKind::CheckBox(CheckBoxProps {
            label: StringRef::literal("Agree"),
            value: BoolRef::path("/agree"),
        }),
    );
    assert_eq!(
        serde_json::to_value(&check_box).unwrap(),
        json!({
            "id": "check",
            "component": {
                "CheckBox": {
                    "label": { "literalString": "Agree" },
                    "value": { "path": "/agree" }
                }
            }
        })
    );

    let card = ComponentEntry::new(
        "card",
        ComponentKind::Card(CardProps {
            child: "card-content".into(),
        }),
    );
    assert_eq!(
        serde_json::to_value(&card).unwrap(),
        json!({
            "id": "card",
            "component": { "Card": { "child": "card-content" } }
        })
    );

    let modal = ComponentEntry::new(
        "modal",
        ComponentKind::Modal(ModalProps {
            entry_point_child: "open-btn".into(),
            content_child: "modal-content".into(),
        }),
    );
    assert_eq!(
        serde_json::to_value(&modal).unwrap(),
        json!({
            "id": "modal",
            "component": {
                "Modal": { "entryPointChild": "open-btn", "contentChild": "modal-content" }
            }
        })
    );

    let tabs = ComponentEntry::new(
        "tabs",
        ComponentKind::Tabs(TabsProps {
            tab_items: vec![
                TabItem { title: StringRef::literal("Overview"), child: "overview".into() },
                TabItem { title: StringRef::literal("Details"), child: "details".into() },
            ],
        }),
    );
    assert_eq!(
        serde_json::to_value(&tabs).unwrap(),
        json!({
            "id": "tabs",
            "component": {
                "Tabs": {
                    "tabItems": [
                        { "title": { "literalString": "Overview" }, "child": "overview" },
                        { "title": { "literalString": "Details" }, "child": "details" }
                    ]
                }
            }
        })
    );

    let multiple_choice = ComponentEntry::new(
        "choices",
        ComponentKind::MultipleChoice(MultipleChoiceProps {
            selections: StringArrayRef::path("/choices"),
            options: vec![
                ChoiceOption { label: StringRef::literal("One"), value: "1".into() },
                ChoiceOption { label: StringRef::literal("Two"), value: "2".into() },
            ],
            max_allowed_selections: Some(1),
        }),
    );
    assert_eq!(
        serde_json::to_value(&multiple_choice).unwrap(),
        json!({
            "id": "choices",
            "component": {
                "MultipleChoice": {
                    "selections": { "path": "/choices" },
                    "options": [
                        { "label": { "literalString": "One" }, "value": "1" },
                        { "label": { "literalString": "Two" }, "value": "2" }
                    ],
                    "maxAllowedSelections": 1
                }
            }
        })
    );

    let slider = ComponentEntry::new(
        "slider",
        ComponentKind::Slider(SliderProps {
            value: NumberRef::path("/rating"),
            min_value: Some(0.0),
            max_value: Some(10.0),
        }),
    );
    assert_eq!(
        serde_json::to_value(&slider).unwrap(),
        json!({
            "id": "slider",
            "component": {
                "Slider": {
                    "value": { "path": "/rating" },
                    "minValue": 0.0,
                    "maxValue": 10.0
                }
            }
        })
    );

    let date_time_input = ComponentEntry::new(
        "date",
        ComponentKind::DateTimeInput(DateTimeInputProps {
            value: StringRef::path("/date"),
            enable_date: Some(true),
            enable_time: Some(false),
            first_date: Some("2024-01-01".into()),
            last_date: Some("2024-12-31".into()),
        }),
    );
    assert_eq!(
        serde_json::to_value(&date_time_input).unwrap(),
        json!({
            "id": "date",
            "component": {
                "DateTimeInput": {
                    "value": { "path": "/date" },
                    "enableDate": true,
                    "enableTime": false,
                    "firstDate": "2024-01-01",
                    "lastDate": "2024-12-31"
                }
            }
        })
    );

    let audio = ComponentEntry::new(
        "audio",
        ComponentKind::AudioPlayer(AudioPlayerProps {
            url: StringRef::literal("https://example.com/audio.mp3"),
        }),
    );
    assert_eq!(
        serde_json::to_value(&audio).unwrap(),
        json!({
            "id": "audio",
            "component": {
                "AudioPlayer": { "url": { "literalString": "https://example.com/audio.mp3" } }
            }
        })
    );

    let video = ComponentEntry::new(
        "video",
        ComponentKind::Video(VideoProps {
            url: StringRef::literal("https://example.com/video.mp4"),
        }),
    );
    assert_eq!(
        serde_json::to_value(&video).unwrap(),
        json!({
            "id": "video",
            "component": {
                "Video": { "url": { "literalString": "https://example.com/video.mp4" } }
            }
        })
    );
}
