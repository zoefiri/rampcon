use bevy::prelude::*;
use super::base_theme;

const TITLE: &str = "rampcon :::: ";

pub fn text_box() -> TextBundle {
    TextBundle {
        style: Style {
            width: Val::Px(150.0),
            height: Val::Px(65.0),
            border: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Start,
            ..default()
        },
        background_color: base_theme::GRAY.into(),
        ..default()
    }
}

pub fn text(font: &Handle<Font>) -> TextBundle {
    TextBundle::from_section(
        TITLE,
        TextStyle {
            font: font.clone(),
            font_size: 30.0,
            color: base_theme::WHITE,
        },
    )
}
