use bevy::prelude::*;
use super::base_theme;

pub fn new_btn() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            width: Val::Px(150.0),
            height: Val::Px(65.0),
            border: UiRect::all(Val::Px(5.0)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        border_color: BorderColor(base_theme::BLACK),
        background_color: base_theme::GRAY.into(),
        ..default()
    }
}

