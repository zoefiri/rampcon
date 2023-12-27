use super::base_theme;
use bevy::prelude::*;
use ropey::Rope;

#[derive(Component, Resource)]
pub struct Focused(pub Option<Entity>);

#[derive(Component, Debug)]
pub struct InputField {
    pub value: Rope,
    pub position: usize,
}

impl InputField {
    pub fn default() -> InputField {
        InputField {
            value: Rope::new(),
            position: 0,
        }
    }

    pub fn field_display_child(font: Handle<Font>) -> TextBundle {
        TextBundle::from_sections([
            TextSection {
                value: "".to_string(),
                style: TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: base_theme::WHITE,
                },
            },
            TextSection {
                value: "".to_string(),
                style: TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: base_theme::WHITE,
                },
            },
        ])
    }

    pub fn cursor_child() -> NodeBundle {
        NodeBundle{
            background_color: BackgroundColor(super::base_theme::BLACK),
            border_color: BorderColor(super::base_theme::WHITE),
            focus_policy: bevy::ui::FocusPolicy::Pass,
            style: Style {
                width: Val::Px(10.0), 
                ..default()
            },
            ..default()
        }
    }

    pub fn default_field_components(geom: bevy::math::Vec2) -> (InputField, ButtonBundle) {
        (
            InputField::default(),
            ButtonBundle {
                style: Style {
                    width: Val::Px(geom.x),
                    height: Val::Px(geom.y),
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
            },
        )
    }
}

impl Focused {
    pub fn change_focus(&self, commands: &mut Commands, next_focus_entity: Entity) {
        // grab the entity at the focus resource (if there is one), and remove the Focused component from it.
        if let Some(focus_res) = self.0 {
            commands.entity(focus_res).remove::<Focused>();
        }

        // insert the Focused component wrapping the entity ref onto the entity for the new focus
        let new_focused_component = Focused(Some(next_focus_entity));
        commands
            .entity(next_focus_entity)
            .insert(new_focused_component);

        // store the new focus at the focus resource
        commands.insert_resource(Focused(Some(next_focus_entity)));
    }
}

pub fn input_mouse_refocus(
    mut commands: Commands,
    query: Query<
        (Entity, &Interaction),
        (Changed<Interaction>, With<InputField>, Without<Focused>),
    >,
    focused_resource: ResMut<Focused>,
) -> anyhow::Result<()> {
    for (new_focus_entity, interaction) in query.iter() {
        println!("register input refocus!! ");
        match interaction {
            Interaction::Pressed => {
                println!("refocus press engaged!");
                focused_resource.change_focus(&mut commands, new_focus_entity);
            }

            _ => (),
        };
    }

    Ok(())
}

pub fn handle_text_input(
    mut commands: Commands,
    asset_server: Res<AssetServer>,

    mut evr_char: EventReader<ReceivedCharacter>,
    kbd: Res<Input<KeyCode>>,
    mut input_field_query: Query<(&mut InputField, &mut Children), With<Focused>>,
    mut display_text_query: Query<&mut Text>,
    mut child_query: Query<&mut Children, (With<Text>, Without<Focused>)>,
) {
    println!(
        "input field query: {:?}",
        input_field_query.get_single_mut()
    );
    if let Ok((mut input_field, children)) = input_field_query.get_single_mut() {
        println!("handling text input...\n");

        let display_text_child = &mut display_text_query.get_mut(children[0]);
        let active_text_section: &mut TextSection;
        let post_text_section: &mut TextSection;

        if let Ok(display_sections) = display_text_child {
            let (active_sections, post_sections) = display_sections.sections.split_at_mut(1);
            [active_text_section, post_text_section] = [&mut active_sections[0], &mut post_sections[0]]
        } else {
            return
        }

        // this breaks the text module lmao :sob:, might need to modify its guts somwehow.
        if input_field.value.to_string() == "yes" {
            let mut text_children = child_query.get_mut(children[0]);
            let display_font: Handle<Font> = asset_server.load("GermaniaOne-Regular.ttf");
            let cursor_entity = commands.spawn(InputField::field_display_child(display_font)).id();
            commands.entity(children[0]).insert_children(
                1,
                &[cursor_entity]
            );
        }

        // match each key-release for directional inputs
        for release in kbd.get_just_released() {
            match release {
                KeyCode::Up => {
                    // if not at the start of the line jump to it
                    // if input_field.position != 0 {
                    //     input_field.position = 0;
                    // }
                } // focus ↑

                KeyCode::Down | KeyCode::Return => (), // focus ↓

                KeyCode::Left => {
                    // move cursor ←
                    if input_field.position > 0 {
                        input_field.position -= 1;
                    }
                }

                KeyCode::Right => {
                    // move cursor →
                    if input_field.position + 1 < input_field.value.len_chars().try_into().unwrap()
                    {
                        input_field.position += 1;
                    }
                }

                KeyCode::Back => {
                    // Delete char @ idx position
                    if input_field.position > 0 {
                        let del_range = input_field.position - 1..input_field.position;
                        input_field.value.remove(del_range);
                        input_field.position -= 1;
                    }
                    active_text_section.value = input_field.value.to_string();
                }

                _ => (),
            }
        }

        for ev in evr_char.read() {
            // if not a control char, insert it into the rope at the current position
            if !ev.char.is_control() {
                let position: usize = input_field.position.try_into().unwrap();
                input_field.value.insert_char(position, ev.char);
                input_field.position += 1;
            }
            // should probably get this and the block in the match above into something cleaner
            active_text_section.value = input_field.value.to_string();
            print!("active text: {}", active_text_section.value)
        }
    }
}
