use super::super::colorgen::model::{ColorSpace, GenericPaletteSpace};
use super::super::colorgen::palette_models::{hsv, HsvColorSpace};
use bevy::prelude::*;
use evalexpr::*;
use ropey::Rope;
use std::collections::HashMap;

// wrapper for HashMapContext that includes a list of "model vars" that
// shouldn't be user-assigned in the context
pub struct ExprCtx {
    pub model_vars: Vec<String>,
}

// represents an expression for a row field in the UI
pub struct ExprRow {
    pub var: String,
    pub expr: String,
}

// represents a list of models, context expressions, and a context for them.
#[derive(Resource)]
pub struct ExprList {
    // ctx holds mappings of variables to expressions for evalexpr
    pub ctx: HashMapContext,

    // expr_rows contains each user-defined expression row in the expressions list
    pub expr_rows: Vec<ExprRow>,

    // model_expr_rows contains each expression row that must be filled for the color model
    pub model_expr_rows: Vec<ExprRow>,
}

// impl ExprList {
//     pub fn eval_expressions
// }

// expr_list_from_model returns an ExprList for anything that implements ColorSpace
pub fn expr_list_from_model(model: &dyn ColorSpace) -> ExprList {
    let model_expr_rows = model
        .inputs()
        .into_iter()
        .map(|input| ExprRow {
            var: input.to_string(),
            expr: String::new(),
        })
        .collect();

    ExprList {
        ctx: context_map! { "dummy" => 0 }.unwrap(),
        expr_rows: vec![ExprRow {
            var: String::new(),
            expr: String::new(),
        }],
        model_expr_rows,
    }
}

pub fn setup_expr_list(mut commands: Commands) {
    // later on once I have dynamic default color models I need to replace this:
    let model = hsv();
    commands.insert_resource(expr_list_from_model(&model.0));
}

impl ExprList {
    // render_rgb_hexes_simple_domain renders expressions into a simple list of RGB hex colors,
    // generating a color for each number from 0 to color_count and supplying it as ctx var `x`
    pub fn render_rgb_hexes_simple_domain(
        &mut self,
        color_model: &dyn ColorSpace,
        color_count: u32,
    ) {
        let mut evaluated_inputs_map: HashMap<String, f32> = Default::default();
        for n in 0..color_count {
            let _ = self.ctx.set_value("x".to_string(), (n as f64).into());
            for input in color_model.inputs() {
                if let Ok(Value::Float(evaluated_input)) = eval_with_context(input, &self.ctx) {
                    evaluated_inputs_map.insert(input.to_string(), evaluated_input as f32);
                }
            }
            color_model.as_rgba_hex(&evaluated_inputs_map);
        }
    }
}
