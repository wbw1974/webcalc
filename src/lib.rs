#[macro_use]
extern crate log;
extern crate regex;
extern crate unicode_segmentation;
extern crate wasm_bindgen;

mod calculator;
mod infix;
mod prefix;
use std::collections::HashMap;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Calc {
    equation: Vec<calculator::Cell>,
    variables: HashMap<String, f64>
}

#[wasm_bindgen]
impl Calc {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Calc {
        Calc {
            equation: Vec::<calculator::Cell>::new(),
            variables: HashMap::<String, f64>::new()
        }
    }

    pub fn calc(&mut self, infix_notation: String) -> JsValue {
        let processed_input = prefix::translate_infix(infix_notation.trim());
        let vector_and_map = calculator::parse_to_vec_and_map(processed_input.as_str());
        if vector_and_map.is_err() {
            // TODO: web-sys call here to alter web presentation
            let result = CalcReturn {
                state: format!("error"),
                equation: format!(""),
                value: vector_and_map.err().unwrap()
            };
            return JsValue::from_serde(&result).unwrap();
        } else {
            let (eq, mut var) = vector_and_map.ok().unwrap();
            if eq.is_empty() {
                if var.is_empty() {
                    let result = CalcReturn {
                        state: format!("error"),
                        equation: format!(""),
                        value: format!("Neither equation nor variable set.")
                    };
                    return JsValue::from_serde(&result).unwrap();
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

            let equation_out = format!(
                "{}",
                infix::translate_prefix(
                    calculator::convert_cell_vector_to_string(&self.equation).as_str()
                )
            );
            let result_cal = calculator::calculate(&self.equation, &self.variables);
            if result_cal.is_ok() {
                let result = CalcReturn {
                    state: format!("success"),
                    equation: equation_out,
                    value: calculator::convert_cell_vector_to_string(&result_cal.ok().unwrap())
                };
                return JsValue::from_serde(&result).unwrap();
            } else {
                let result = CalcReturn {
                    state: format!("error"),
                    equation: format!(""),
                    value: result_cal.err().unwrap()
                };
                return JsValue::from_serde(&result).unwrap();
            }
        }
    }
}
