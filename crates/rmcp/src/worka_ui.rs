//! Worka UI (A2UI) builders.
//!
//! This module provides a Flutter-like API for producing A2UI JSON payloads
//! (as rendered by Worka's Flutter host via GenUI).
//!
//! Key properties:
//! - Packs build a widget tree using plain Rust structs (no macros).
//! - The renderer assigns component IDs internally (packs don't manage graphs).
//! - Output is serialized to the same A2UI message shapes understood by the host.

mod action;
mod children;
mod render;
mod value;
mod widgets;

mod wire;

pub use action::{Action, ActionValue};
pub use children::Children;
pub use render::{render, RenderedTree, Surface};
pub use value::{BoolValue, NumberValue, StringArrayValue, StringValue};
pub use widgets::*;

/// Common imports for packs.
pub mod prelude {
    pub use super::{
        render, Action, AudioPlayer, BoolValue, Button, Card, CheckBox, ChoiceOption, Children,
        Column, DateTimeInput, Divider, Icon, Image, List, Modal, MultipleChoice, NumberValue, Row,
        Slider, StringArrayValue, StringValue, Surface, TabItem, Tabs, Text, TextField, Timeline,
        TimelineGroup, TimelineItem, TimelineLane, Video,
    };
}
