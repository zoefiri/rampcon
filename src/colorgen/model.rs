use bevy::prelude::*;
use palette::{
    convert::FromColorUnclamped, convert::IntoColor, rgb::Rgb, stimulus::FromStimulus, Clamp,
};
use std::collections::HashMap;
use std::marker::PhantomData;

pub trait ColorSpaceData {
    // inputs provides the inputs this color space requires
    fn inputs(&self) -> &Vec<String>;

    // name provides a user-facing name for this colorspace
    fn name(self) -> String;
}

pub trait AsRGBA {
    // converts the color space data into a #rrggbbaa hexcode, as an u32
    fn hsv_as_rgba_hex(self, hsv_components: [f32; 3]) -> u32;
    fn as_rgba_hex(&self, input_vals: &HashMap<String, f32>) -> u32;
    fn many_as_rgba_hex(self, input_vals_vec: Vec<&HashMap<String, f32>>) -> Vec<u32>;
}

pub trait FromHSVLikeVals<T> {
    // converts any HSV-like values into a given color space
    fn from_hsv_like(self) -> T;
}

pub trait PaletteColorSpace:
    Copy
    + Into<[f32; 4]>
    + IntoColor<Rgb>
    + Clamp
    + FromColorUnclamped<std::vec::Vec<f32>>
    + FromColorUnclamped<[f32; 3]>
{
}

pub trait ColorSpace: AsRGBA + ColorSpaceData {}

pub struct ColorSpaceRes(dyn ColorSpace);

// generic colorspace for wrapping a Palette provided colorspace
pub struct GenericPaletteSpace<T: PaletteColorSpace> {
    // name is a user-facing name for this colorspace
    pub name: String,

    // inputs is necessary so that we have an ordered list of the
    // components a Palette colorspace takes and can use
    // from_components()
    pub inputs: Vec<String>,

    // _palette_color_space exists to identify the Palette colorspace of this GenericPaletteSpace
    pub _palette_color_space: PhantomData<T>,
}

impl<T: PaletteColorSpace> ColorSpaceData for GenericPaletteSpace<T> {
    fn inputs(&self) -> &Vec<String> {
        return &self.inputs;
    }

    fn name(self) -> String {
        return self.name;
    }
}

impl<T: PaletteColorSpace> AsRGBA for GenericPaletteSpace<T> {
    fn hsv_as_rgba_hex(self, hsv_components: [f32; 3]) -> u32 {
        let rgb_color: Rgb = Rgb::from(hsv_components);
        let rgb_bytes: [f32; 3] = rgb_color.into();

        (rgb_bytes[0] as u32) << 24
            | (rgb_bytes[1] as u32) << 16
            | (rgb_bytes[2] as u32) << 8
            | 0xff
    }

    fn as_rgba_hex(&self, input_vals: &HashMap<String, f32>) -> u32 {
        let mut component_vec: Vec<f32> = vec![];
        for input in &self.inputs {
            if let Some(input_val) = input_vals.get(input) {
                component_vec.push(*input_val);
            }
        }

        let color: T = component_vec.into_color();

        color_as_rgb(color)
    }

    fn many_as_rgba_hex(self, input_vals_vec: Vec<&HashMap<String, f32>>) -> Vec<u32> {
        let mut hex_vec: Vec<u32> = vec![];
        for input_vals in input_vals_vec {
            hex_vec.push(self.as_rgba_hex(input_vals));
        }

        hex_vec
    }
}

fn color_as_rgb<T: PaletteColorSpace>(color: T) -> u32 {
    let rgb_color: Rgb = color.into_color();
    let rgb_bytes: [f32; 3] = rgb_color.into();

    (rgb_bytes[0] as u32) << 24 | (rgb_bytes[1] as u32) << 16 | (rgb_bytes[2] as u32) << 8 | 0xff
}

// impl<T: PaletteColorSpace> ColorSpace for T { }

// impl<T: PaletteColorSpace> ColorSpace for GenericPaletteSpace<T> where T: AsRGBA + ColorSpaceData {}
impl<T: PaletteColorSpace> ColorSpace for GenericPaletteSpace<T> {}
