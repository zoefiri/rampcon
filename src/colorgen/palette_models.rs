use super::model::{GenericPaletteSpace, ColorSpace};
use std::marker::PhantomData;

use bevy::prelude::*;

#[derive(Resource)]
pub struct HsvColorSpace(pub GenericPaletteSpace<palette::hsv::Hsv>);

pub fn setup_model_resources(mut commands: Commands) {
    commands.insert_resource(hsv());
}

pub fn hsv() -> HsvColorSpace {
    HsvColorSpace(GenericPaletteSpace {
        name: "hsv".into(),
        inputs: vec!["h".into(), "s".into(), "v".into()],
        _palette_color_space: PhantomData,
    })
}

// impl ColorSpace for GenericPaletteSpace<palette::hsv::Hsv> {}
