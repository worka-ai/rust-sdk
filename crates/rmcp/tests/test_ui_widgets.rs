use rmcp::worka_ui::prelude::*;
use serde_json::{json, Value};

fn root_component(doc: &Value) -> Value {
    let root = doc.get("root").and_then(Value::as_str).expect("root");
    let components = doc.get("components").and_then(Value::as_array).expect("components");
    components
        .iter()
        .find(|entry| entry.get("id").and_then(Value::as_str) == Some(root))
        .cloned()
        .expect("root component entry")
}

#[test]
fn renders_widget_tree_to_document() {
    let doc = render(Text::new("Hello").usage_hint("h1")).to_value();
    assert_eq!(
        doc,
        json!({
            "components": [
                { "id": "w0", "component": { "Text": { "text": { "literalString": "Hello" }, "usageHint": "h1" } } }
            ],
            "root": "w0"
        })
    );

    let doc = render(
        Row::new(Children::items(vec![Text::new("A").into(), Text::new("B").into()]))
            .distribution("spaceBetween")
            .alignment("center"),
    )
    .to_value();

    let root = root_component(&doc);
    assert_eq!(
        root,
        json!({
            "id": "w2",
            "component": {
                "Row": {
                    "children": { "explicitList": ["w0", "w1"] },
                    "distribution": "spaceBetween",
                    "alignment": "center"
                }
            }
        })
    );
}

#[test]
fn renders_surface_message_list() {
    let view = Surface::new("contacts", Text::new("Hi")).to_value();
    let messages = view.as_array().expect("message list");
    assert_eq!(messages.len(), 2);
    assert_eq!(messages[0]["surfaceUpdate"]["surfaceId"], json!("contacts"));
    assert_eq!(messages[1]["beginRendering"]["surfaceId"], json!("contacts"));
}
