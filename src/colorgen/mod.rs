/*
 * Ok so I need to write a little bit on how this is going to be modelled. 
 * 1. color "model" is comprised of several "inputs", each input is a float with specified
 *    constraints.
 * 2. When given these inputs a color model will generate an RGB value
 * 3. Each "input" should be able to be defined either as a static value or as a mathematical
 *    function, the domain for which would be n, the color index being generated.
 * 
 * more specifically this would look like:
 * + trait for a color model that provides a list of the inputs for that model
 * + I/O driver function that takes an implementation of color model trait and generates a color for it
 *     + Input: map color vars provided via trait -> float vals
 *     + Output: RGB value
 *
 */

pub mod model;
pub mod palette_models;
