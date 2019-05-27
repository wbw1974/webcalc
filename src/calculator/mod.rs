use unicode_segmentation::UnicodeSegmentation;
use regex::Regex;
use std::collections::HashMap;

/// Enum that defines parts of an equation: 
/// * An Operator [+, -, *, /]
/// * A Variable [Any non-numeric and not an oerator. Including whole words.]
/// * A Value [Any numeric. e.g. 1, 2, 3.141414...]
/// ```
#[derive(Debug, Clone)]
pub enum Cell {
    Operator(String),
    Variable(String),
    Value(f64),
}

/// Function that parses an equation in prefix notation to either a vector of cells (an equation) or a hashmap.
pub fn parse_to_vec_and_map(in_str: &str) -> Result<(Vec<Cell>, HashMap<String, f64>), String> {
    let mut vec = Vec::<Cell>::new();
    let mut map = HashMap::<String, f64>::new();
    let whitespace = Regex::new(r"^\s+?").unwrap();

    let mut word_or_punctuation = in_str
        .split_word_bounds()
        .filter(|x| !whitespace.is_match(x))
        .peekable();
    loop {
        let item = word_or_punctuation.next();
        debug!("item: {}", item.unwrap());
        match item {
            Some("=") => {
                let variable = word_or_punctuation.next().unwrap();
                let value = word_or_punctuation.next().unwrap().parse::<f64>();
                if word_or_punctuation.next() == None {
                    if variable.is_empty() || value.is_err() {
                        return Err(
                            String::from("Equals can only take the form of variable = value.")
                        );
                    } else {
                        map.insert(String::from(variable), value.unwrap());
                    }
                } else {
                    return Err(
                        String::from("Equals can only take the form of variable = value. Cannot solve for value.")
                    );
                }
            }
            Some("(") | Some(")") => {}
            Some("+") | Some("-") | Some("*") | Some("/") => {
                let s = String::from(item.unwrap());
                let cell_item = Cell::Operator(s);
                vec.push(cell_item);
            }
            Some(_) => {
                let val = item.unwrap().parse::<f64>();
                if val.is_err() {
                    let s = String::from(item.unwrap());
                    let cell_item = Cell::Variable(s);
                    vec.push(cell_item);
                } else {
                    let cell_item = Cell::Value(val.unwrap());
                    vec.push(cell_item);
                }
            }
            None => break,
        }
    }
    return Ok((vec, map));
}

/// Function that attempts to resolve an equation, using the passed in hash map to resolve variables.
pub fn calculate(vec0: &Vec<Cell>, map: &HashMap<String, f64>) -> Result<Vec<Cell>, String> {
    let mut ret = Vec::<Cell>::new();
    let mut stack_ops = Vec::new();
    let mut stack_vals = Vec::new();
    let mut iter = vec0.iter().rev();

    loop {
        match iter.next() {
            Some(cell) => match cell.clone() {
                Cell::Operator(ref val) => {
                    debug!("Operator: {}", val);
                    stack_ops.push(val.clone());
                    if stack_vals.len() >= 2 {
                        let cell1 = stack_vals.pop().unwrap();
                        let cell2 = stack_vals.pop().unwrap();
                        let op = stack_ops.pop().unwrap();
                        let cell_answer = process(&op, &cell1, &cell2);
                        if cell_answer.is_ok() {
                            if let Cell::Value(ref i) = cell_answer.ok().unwrap() {
                                stack_vals.push(*i);
                            }
                        } else {
                            return Err(cell_answer.err().unwrap());
                        }
                    } else {
                        return Err(format!("Not enough values to apply to Operator {}.", val));
                    }
                }
                Cell::Value(ref val) => {
                    debug!("Value: {}", val);
                    stack_vals.push(*val);
                }
                Cell::Variable(ref val) => {
                    debug!("Variable: {}", val);
                    if map.contains_key(val) {
                        match map.get(val) {
                            Some(v) => {
                                stack_vals.push(*v);
                            }
                            None => {
                                return Err(format!("Variable {} has a defined value of None.", val));
                            }
                        }
                    } else {
                        return Err(format!("Variable {} does not have a defined value.", val));
                    }
                }
            },
            None => {
                break;
            }
        }
    }

    if stack_ops.len() > 0 {
        return Err(String::from("Did not use all operators!"));
    }

    ret.push(Cell::Value(stack_vals.pop().unwrap()));
    return Ok(ret);
}

/// Function that converts a vector of cells into a prefix equation.
pub fn convert_cell_vector_to_string(vec: &Vec<Cell>) -> String {
    let mut ret = String::new();
    let mut first = true;

    for cell in vec.iter() {
        let cell_clone = cell.clone();
        if first {
            first = false;
        } else {
            ret.push_str(" ");
        }

        if let Cell::Value(ref ret_val) = cell_clone {
            ret.push_str(format!("{:.8}", ret_val).as_str());
        } else if let Cell::Variable(ref ret_val) = cell_clone {
            ret.push_str(format!("{:.8}", ret_val).as_str());
        } else if let Cell::Operator(ref ret_val) = cell_clone {
            ret.push_str(format!("{:.8}", ret_val).as_str());
        } else {
            ret.push_str("error");
        }
    }
    return ret;
}

/// Private function that performs the defined set of calculation functions (+, -, *, /)
fn process(op: &String, left: &f64, right: &f64) -> Result<Cell, String> {
    match op.as_str() {
        "+" => {
            let cell_item = Cell::Value(left + right);
            return Ok(cell_item);
        }
        "-" => {
            let cell_item = Cell::Value(left - right);
            return Ok(cell_item);
        }
        "*" => {
            let cell_item = Cell::Value(left * right);
            return Ok(cell_item);
        }
        "/" => {
            let cell_item = Cell::Value(left / right);
            return Ok(cell_item);
        }
        _ => {
            return Err(format!("Cannot process: ({}{}{})", op, left, right));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_add() {
        let result = String::from("4.00000000");
        let input = String::from("+ 2 2");
        let vector_and_map = parse_to_vec_and_map(input.as_str());
        if vector_and_map.is_err() {
             panic!("{}", vector_and_map.err().unwrap().as_str());
        }

        let (vec, map) = vector_and_map.ok().unwrap();
        let result_cal = calculate(&vec, &map);
        if result_cal.is_ok() {
            assert_eq!(result, convert_cell_vector_to_string(&result_cal.ok().unwrap()));
        } else {
            assert_eq!(result, result_cal.err().unwrap());
        }
    }

    #[test]
    fn test_simple_subtract() {
        let result = String::from("0.00000000");
        let input = String::from("- 2 2");
        let vector_and_map = parse_to_vec_and_map(input.as_str());
        if vector_and_map.is_err() {
             panic!("{}", vector_and_map.err().unwrap());
        }

        let (vec, map) = vector_and_map.ok().unwrap();
        let result_cal = calculate(&vec, &map);
        if result_cal.is_ok() {
            assert_eq!(result, convert_cell_vector_to_string(&result_cal.ok().unwrap()));
        } else {
            assert_eq!(result, result_cal.err().unwrap());
        }
    }

    #[test]
    fn test_simple_multiply() {
        let result = String::from("4.00000000");
        let input = String::from("* 2 2");
        let vector_and_map = parse_to_vec_and_map(input.as_str());
        if vector_and_map.is_err() {
             panic!("{}", vector_and_map.err().unwrap());
        }

        let (vec, map) = vector_and_map.ok().unwrap();
        let result_cal = calculate(&vec, &map);
        if result_cal.is_ok() {
            assert_eq!(result, convert_cell_vector_to_string(&result_cal.ok().unwrap()));
        } else {
            assert_eq!(result, result_cal.err().unwrap());
        }
    }

    #[test]
    fn test_simple_divide() {
        let result = String::from("1.00000000");
        let input = String::from("/ 2 2");
        let vector_and_map = parse_to_vec_and_map(input.as_str());
        if vector_and_map.is_err() {
             panic!("{}", vector_and_map.err().unwrap());
        }

        let (vec, map) = vector_and_map.ok().unwrap();
        let result_cal = calculate(&vec, &map);
        if result_cal.is_ok() {
            assert_eq!(result, convert_cell_vector_to_string(&result_cal.ok().unwrap()));
        } else {
            assert_eq!(result, result_cal.err().unwrap());
        }
    }

    #[test]
    fn test_complex_one() {
        let result = String::from("14.00000000");
        let input = String::from("+ + 5 6 + 1 2");
        let vector_and_map = parse_to_vec_and_map(input.as_str());
        if vector_and_map.is_err() {
             panic!("{}", vector_and_map.err().unwrap());
        }

        let (vec, map) = vector_and_map.ok().unwrap();
        let result_cal = calculate(&vec, &map);
        if result_cal.is_ok() {
            assert_eq!(result, convert_cell_vector_to_string(&result_cal.ok().unwrap()));
        } else {
            assert_eq!(result, result_cal.err().unwrap());
        }
    }

    #[test]
    fn test_complex_two() {
        let result = String::from("21.96875000");
        let input = String::from("+ ( + 1 * 2 / 3 ( - 4 / 5 ( * 6 / 7 8 ) ) ) ( + 9 10 )");
        let vector_and_map = parse_to_vec_and_map(input.as_str());
        if vector_and_map.is_err() {
             panic!("{}", vector_and_map.err().unwrap());
        }

        let (vec, map) = vector_and_map.ok().unwrap();
        let result_cal = calculate(&vec, &map);
        if result_cal.is_ok() {
            assert_eq!(result, convert_cell_vector_to_string(&result_cal.ok().unwrap()));
        } else {
            assert_eq!(result, result_cal.err().unwrap());
        }
    }

    #[test]
    fn test_substitution_1() {
        let result = String::from("10.00000000");
        let input = String::from("+ + a b + c d");

        let vector_and_map = parse_to_vec_and_map(input.as_str());
        if vector_and_map.is_err() {
             panic!("{}", vector_and_map.err().unwrap());
        }
        let (equation, mut variables) = vector_and_map.ok().unwrap();

        let input = String::from("= a 1");
        let vector_and_map = parse_to_vec_and_map(input.as_str());
        if vector_and_map.is_err() {
            panic!("{}", vector_and_map.err().unwrap().as_str());
        } else {
            let (throw_away, mut var1) = vector_and_map.ok().unwrap();
            if !throw_away.is_empty() {
                panic!("Set variable. Expected equation to be empty.");
            }
            if var1.is_empty() {
                panic!("Set variable. Expected variables to have values.");
            } else {
                for (k, v) in var1.drain().take(1) {
                    variables.insert(k, v);
                }
            }
        }

        let input = String::from("= b 2");
        let vector_and_map = parse_to_vec_and_map(input.as_str());
        if vector_and_map.is_err() {
            panic!("{}", vector_and_map.err().unwrap().as_str());
        } else {
            let (throw_away, mut var1) = vector_and_map.ok().unwrap();
            if !throw_away.is_empty() {
                panic!("Set variable. Expected equation to be empty.");
            }
            if var1.is_empty() {
                panic!("Set variable. Expected variables to have values.");
            } else {
                for (k, v) in var1.drain().take(1) {
                    variables.insert(k, v);
                }
            }
        }

        let input = String::from("= c 3");
        let vector_and_map = parse_to_vec_and_map(input.as_str());
        if vector_and_map.is_err() {
            panic!("{}", vector_and_map.err().unwrap().as_str());
        } else {
            let (throw_away, mut var1) = vector_and_map.ok().unwrap();
            if !throw_away.is_empty() {
                panic!("Set variable. Expected equation to be empty.");
            }
            if var1.is_empty() {
                panic!("Set variable. Expected variables to have values.");
            } else {
                for (k, v) in var1.drain().take(1) {
                    variables.insert(k, v);
                }
            }
        }

        let input = String::from("= d 4");
        let vector_and_map = parse_to_vec_and_map(input.as_str());
        if vector_and_map.is_err() {
            panic!("{}", vector_and_map.err().unwrap().as_str());
        } else {
            let (throw_away, mut var1) = vector_and_map.ok().unwrap();
            if !throw_away.is_empty() {
                panic!("Set variable. Expected equation to be empty.");
            }
            if var1.is_empty() {
                panic!("Set variable. Expected variables to have values.");
            } else {
                for (k, v) in var1.drain().take(1) {
                    variables.insert(k, v);
                }
            }
        }

        let result_cal = calculate(&equation, &variables);
        assert_eq!(result, convert_cell_vector_to_string(&result_cal.ok().unwrap()));
    }
}
