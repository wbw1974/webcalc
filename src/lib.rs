extern crate regex;
extern crate unicode_segmentation;
extern crate wasm_bindgen;

mod calculator;
mod infix;
mod prefix;
use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub struct Calc {
    equation: Vec<calculator::Cell>,
    variables: HashMap<String, f64>
}

#[wasm_bindgen]
impl Calc {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Calc {
        return Calc {
            equation: Vec::<calculator::Cell>::new(),
            variables: HashMap::<String, f64>::new()
        }
    }

    #[wasm_bindgen]
    pub fn calc(&mut self, infix_notation: String) -> Result<(), JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let processed_input = prefix::translate_infix(infix_notation.trim());
        let vector_and_map = calculator::parse_to_vec_and_map(processed_input.as_str());
        if vector_and_map.is_err() {
            let thing = unwrap_html_textarea_element(document.get_element_by_id("inputHistory").unwrap());
                let thing1 = thing.selection_end().unwrap();
                let thing2 = thing.selection_start().unwrap();
                let val = thing.value();
                thing.set_value(" ");
                // input_history.value += infix_notation + "\n";
                // TODO: web-sys call here to alter web presentation
                return Ok(());
        } else {
            let (eq, mut var) = vector_and_map.ok().unwrap();
            if eq.is_empty() {
                if var.is_empty() {
            // TODO: web-sys call here to alter web presentation
            return Ok(());
                } else {
                    for (k, v) in var.drain().take(1) {
                        self.variables.insert(k, v);
                    }
                }
            } else {
                self.equation.clear();
                let mut iter = eq.iter();
                loop {
                    match iter.next() {
                        Some(element) => {
                            self.equation.push(element.clone());
                        }
                        None => {
                            break;
                        }
                    }
                }
            }

            // TODO: Might use this in output
            // let equation_out = format!(
            //     "{}",
            //     infix::translate_prefix(
            //         calculator::convert_cell_vector_to_string(&self.equation).as_str()
            //     )
            // );
            let result_cal = calculator::calculate(&self.equation, &self.variables);
            if result_cal.is_ok() {
                // TODO: web-sys call here to alter web presentation
                return Ok(());
            } else {
                // TODO: web-sys call here to alter web presentation
                return Ok(());
            }
        }
    }
}

/// Function that performs a specific dynamic cast from an Element to an HtmlTextAreaElement
fn unwrap_html_textarea_element(element: web_sys::Element) -> web_sys::HtmlTextAreaElement {
    return element.dyn_into::<web_sys::HtmlTextAreaElement>().unwrap();
}
