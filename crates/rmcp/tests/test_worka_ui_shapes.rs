use rmcp::worka_ui::prelude::*;
use serde_json::json;

#[test]
fn worka_ui_serialization_matches_genui_shapes() {
    let doc = render(
        Column::new(Children::items(vec![
            Text::new("Hello").usage_hint("h1").into(),
            Image::new("https://example.com/logo.png")
                .fit("cover")
                .usage_hint("mediumFeature")
                .into(),
            Icon::new("check").into(),
            Divider::horizontal().into(),
            Row::new(Children::items(vec![Text::new("A").into(), Text::new("B").into()]))
                .distribution("spaceBetween")
                .alignment("center")
                .into(),
            List::vertical(Children::items(vec![Text::new("one").into(), Text::new("two").into()]))
                .alignment("start")
                .into(),
            Button::new(
                Text::new("submit"),
                Action::new("submit").with("origin", "worka-ui-test"),
            )
            .primary(true)
            .into(),
            TextField::new()
                .text(StringValue::path("/email"))
                .label("Email")
                .field_type("shortText")
                .validation_regexp("^.+@.+$")
                .on_submitted(Action::new("submit_form"))
                .into(),
            CheckBox::new("Agree", BoolValue::path("/agree")).into(),
            Card::new(Text::new("Card content")).into(),
            Modal::new(Text::new("Open"), Text::new("Body")).into(),
            Tabs::new(vec![
                TabItem::new("Overview", Text::new("Overview content")),
                TabItem::new("Details", Text::new("Details content")),
            ])
            .into(),
            MultipleChoice::new(
                StringArrayValue::path("/choices"),
                vec![ChoiceOption::new("One", "1"), ChoiceOption::new("Two", "2")],
            )
            .max_allowed_selections(1)
            .into(),
            Slider::new(NumberValue::path("/rating")).min(0.0).max(10.0).into(),
            DateTimeInput::new(StringValue::path("/date"))
                .enable_date(true)
                .enable_time(false)
                .first_date("2024-01-01")
                .last_date("2024-12-31")
                .into(),
            AudioPlayer::new("https://example.com/audio.mp3").into(),
            Video::new("https://example.com/video.mp4").into(),
            Timeline::new(Children::items(vec![
                TimelineItem::new("item-1")
                    .title("Step Started")
                    .subtitle("Preparing data")
                    .timestamp("2026-01-24T00:00:00Z")
                    .kind("step")
                    .state("running")
                    .severity("info")
                    .icon("bolt")
                    .action(Action::new("timeline.focus_item"))
                    .into(),
                TimelineGroup::new(
                    "group-1",
                    Children::items(vec![TimelineItem::new("item-2").title("Another").into()]),
                )
                .title("Task Group")
                .summary("2 tasks")
                .collapsed(false)
                .badge_count(2_i64)
                .group_state("running")
                .into(),
                TimelineLane::new("lane-1", Children::items(vec![TimelineItem::new("item-3").into()]))
                    .title("Lane A")
                    .into(),
            ]))
            .orientation("vertical")
            .alignment("alternate")
            .auto_follow(true)
            .lane_mode("single")
            .current_item_id("item-2")
            .into(),
        ])),
    )
    .to_value();

    // Spot-check: ensure expected component kinds exist.
    let components = doc["components"].as_array().expect("components");
    for kind in [
        "Text",
        "Image",
        "Icon",
        "Divider",
        "Row",
        "Column",
        "List",
        "Button",
        "TextField",
        "CheckBox",
        "Card",
        "Modal",
        "Tabs",
        "MultipleChoice",
        "Slider",
        "DateTimeInput",
        "AudioPlayer",
        "Video",
        "Timeline",
        "TimelineItem",
        "TimelineGroup",
        "TimelineLane",
    ] {
        assert!(
            components.iter().any(|c| c["component"].get(kind).is_some()),
            "missing component kind {kind}"
        );
    }

    // Ensure bindings serialize as expected.
    let text_field = components
        .iter()
        .find(|c| c["component"].get("TextField").is_some())
        .expect("TextField");
    assert_eq!(text_field["component"]["TextField"]["text"], json!({ "path": "/email" }));

    // Ensure timeline currentItemId is emitted.
    let timeline = components
        .iter()
        .find(|c| c["component"].get("Timeline").is_some())
        .expect("Timeline");
    assert_eq!(
        timeline["component"]["Timeline"]["currentItemId"],
        json!({ "literalString": "item-2" })
    );
}

#[test]
fn surface_emits_surface_update_and_begin_rendering() {
    let value = Surface::new("surface-1", Text::new("Hello")).to_value();
    let messages = value.as_array().expect("array");
    assert_eq!(messages.len(), 2);
    assert_eq!(messages[0]["surfaceUpdate"]["surfaceId"], json!("surface-1"));
    assert_eq!(messages[1]["beginRendering"]["surfaceId"], json!("surface-1"));
}

#[test]
fn surface_includes_data_model_update_when_provided() {
    let model = json!([{ "key": "worka", "valueString": "ok" }]);
    let value = Surface::new("surface-1", Text::new("Hello"))
        .data_model(model.clone())
        .to_value();
    let messages = value.as_array().expect("array");
    assert_eq!(messages.len(), 3);
    assert_eq!(messages[1]["dataModelUpdate"]["contents"], model);
}
