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
    pub fn calc(&mut self) -> Result<(), JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        let infix_notation_control = unwrap_html_input_element(document.get_element_by_id("output").unwrap());
        let infix_notation = infix_notation_control.value();
        let processed_input = prefix::translate_infix(infix_notation.trim());
        let vector_and_map = calculator::parse_to_vec_and_map(processed_input.as_str());
        if vector_and_map.is_err() {
            let input_history = unwrap_html_textarea_element(document.get_element_by_id("inputHistory").unwrap());
            if  input_history.selection_end().unwrap() == input_history.selection_start().unwrap() {
                input_history.set_scroll_top(input_history.scroll_height());
            }
            let mut ret = String::new();
            ret.push_str(&input_history.value());
            ret.push_str("Error parsing '");
            ret.push_str(&infix_notation);
            ret.push_str("': \n");
            ret.push_str(&vector_and_map.err().unwrap());
            ret.push_str("\n");
            input_history.set_value(&ret);
            return Ok(());
        } else {
            let (eq, mut var) = vector_and_map.ok().unwrap();
            if eq.is_empty() {
                if var.is_empty() {
                    let input_history = unwrap_html_textarea_element(document.get_element_by_id("inputHistory").unwrap());
                    if  input_history.selection_end().unwrap() == input_history.selection_start().unwrap() {
                        input_history.set_scroll_top(input_history.scroll_height());
                    }
                    let mut ret = String::new();
                    ret.push_str(&input_history.value());
                    ret.push_str("Neither equation nor variable set.\n");
                    input_history.set_value(&ret);
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

            let result_cal = calculator::calculate(&self.equation, &self.variables);
            if result_cal.is_ok() {
                let input_history = unwrap_html_textarea_element(document.get_element_by_id("inputHistory").unwrap());
                if  input_history.selection_end().unwrap() == input_history.selection_start().unwrap() {
                    input_history.set_scroll_top(input_history.scroll_height());
                }
                let mut ret = String::new();
                ret.push_str(&infix_notation);
                ret.push_str("\n");
                input_history.set_value(&ret);

                let output = unwrap_html_input_element(document.get_element_by_id("output").unwrap());
                output.set_value(&calculator::convert_cell_vector_to_string(&result_cal.ok().unwrap()));
                return Ok(());
            } else {
                let input_history = unwrap_html_textarea_element(document.get_element_by_id("inputHistory").unwrap());
                if  input_history.selection_end().unwrap() == input_history.selection_start().unwrap() {
                    input_history.set_scroll_top(input_history.scroll_height());
                }
                let mut ret = String::new();
                ret.push_str(&input_history.value());
                ret.push_str("Error from calculator:");
                ret.push_str(&result_cal.err().unwrap());
                ret.push_str("\n");
                input_history.set_value(&ret);
                return Ok(());
            }
        }
    }
}

/// Function that performs a specific dynamic cast from an Element to an HtmlTextAreaElement
fn unwrap_html_textarea_element(element: web_sys::Element) -> web_sys::HtmlTextAreaElement {
    return element.dyn_into::<web_sys::HtmlTextAreaElement>().unwrap();
}

/// Function that performs a specific dynamic cast from an Element to an HtmlInputElement
fn unwrap_html_input_element(element: web_sys::Element) -> web_sys::HtmlInputElement {
    return element.dyn_into::<web_sys::HtmlInputElement>().unwrap();
}
