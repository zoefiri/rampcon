use bevy::prelude::*;
use super::base_theme;

pub fn text_box(w:f32, h:f32) -> TextBundle {
    TextBundle {
        style: Style {
            top: Val::Px(300.0),
            width: Val::Px(w),
            height: Val::Px(h),
            border: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Start,
            ..default()
        },
        focus_policy: bevy::ui::FocusPolicy::Pass,
        background_color: base_theme::GRAY.into(),
        ..default()
    }
}

pub fn text(text: &str, font: &Handle<Font>) -> TextBundle {
    TextBundle::from_section(
        text,
        TextStyle {
            font: font.clone(),
            font_size: 30.0,
            color: base_theme::WHITE,
        },
    )
}
