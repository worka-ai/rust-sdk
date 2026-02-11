mod basic;
mod controls;
mod layout;
mod media;
mod timeline;

pub use basic::{Card, Divider, Icon, Image, Text};
pub use controls::{
    Button, CheckBox, ChoiceOption, DateTimeInput, Modal, MultipleChoice, Slider, TabItem, Tabs,
    TextField,
};
pub use layout::{Column, Flexible, List, Row};
pub use media::{AudioPlayer, Video};
pub use timeline::{Timeline, TimelineGroup, TimelineItem, TimelineLane};

#[derive(Debug, Clone, PartialEq)]
pub enum Widget {
    Text(Text),
    Image(Image),
    Icon(Icon),
    Divider(Divider),
    Row(Row),
    Column(Column),
    List(List),
    Flexible(Flexible),
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

impl From<Text> for Widget {
    fn from(value: Text) -> Self {
        Widget::Text(value)
    }
}
impl From<Image> for Widget {
    fn from(value: Image) -> Self {
        Widget::Image(value)
    }
}
impl From<Icon> for Widget {
    fn from(value: Icon) -> Self {
        Widget::Icon(value)
    }
}
impl From<Divider> for Widget {
    fn from(value: Divider) -> Self {
        Widget::Divider(value)
    }
}
impl From<Row> for Widget {
    fn from(value: Row) -> Self {
        Widget::Row(value)
    }
}
impl From<Column> for Widget {
    fn from(value: Column) -> Self {
        Widget::Column(value)
    }
}
impl From<List> for Widget {
    fn from(value: List) -> Self {
        Widget::List(value)
    }
}
impl From<Flexible> for Widget {
    fn from(value: Flexible) -> Self {
        Widget::Flexible(value)
    }
}
impl From<Button> for Widget {
    fn from(value: Button) -> Self {
        Widget::Button(value)
    }
}
impl From<TextField> for Widget {
    fn from(value: TextField) -> Self {
        Widget::TextField(value)
    }
}
impl From<CheckBox> for Widget {
    fn from(value: CheckBox) -> Self {
        Widget::CheckBox(value)
    }
}
impl From<Card> for Widget {
    fn from(value: Card) -> Self {
        Widget::Card(value)
    }
}
impl From<Modal> for Widget {
    fn from(value: Modal) -> Self {
        Widget::Modal(value)
    }
}
impl From<Tabs> for Widget {
    fn from(value: Tabs) -> Self {
        Widget::Tabs(value)
    }
}
impl From<MultipleChoice> for Widget {
    fn from(value: MultipleChoice) -> Self {
        Widget::MultipleChoice(value)
    }
}
impl From<Slider> for Widget {
    fn from(value: Slider) -> Self {
        Widget::Slider(value)
    }
}
impl From<DateTimeInput> for Widget {
    fn from(value: DateTimeInput) -> Self {
        Widget::DateTimeInput(value)
    }
}
impl From<AudioPlayer> for Widget {
    fn from(value: AudioPlayer) -> Self {
        Widget::AudioPlayer(value)
    }
}
impl From<Video> for Widget {
    fn from(value: Video) -> Self {
        Widget::Video(value)
    }
}
impl From<Timeline> for Widget {
    fn from(value: Timeline) -> Self {
        Widget::Timeline(value)
    }
}
impl From<TimelineItem> for Widget {
    fn from(value: TimelineItem) -> Self {
        Widget::TimelineItem(value)
    }
}
impl From<TimelineGroup> for Widget {
    fn from(value: TimelineGroup) -> Self {
        Widget::TimelineGroup(value)
    }
}
impl From<TimelineLane> for Widget {
    fn from(value: TimelineLane) -> Self {
        Widget::TimelineLane(value)
    }
}
