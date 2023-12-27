use super::super::colorgen::model::{GenericPaletteSpace, ColorSpace};
use super::super::colorgen::palette_models::HsvColorSpace;
use super::super::expr::parse::{ExprList, ExprRow};
use bevy_egui::{egui, EguiContexts};
use bevy::prelude::*;

pub fn ui_example_system(
    mut contexts: EguiContexts,
    expr_list_res: ResMut<ExprList>,
    colorspace_res: ResMut<HsvColorSpace>,
) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.heading("rampcon");
        // for 
        // egui_expr_row(ui, )
        // ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
        // if ui.button("Click each year").clicked() {
        //     age += 1;
        // }
        // ui.label(format!("Hello '{name}', age {age}"));
    });
}

fn egui_expr_row(ui: &mut egui::Ui, expr_row: &mut ExprRow) {
    ui.horizontal(|ui| {
        ui.text_edit_singleline(&mut expr_row.var);
        ui.text_edit_singleline(&mut expr_row.expr);
    });
}
