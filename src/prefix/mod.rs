use unicode_segmentation::UnicodeSegmentation;
use regex::Regex;

/// Function that translates a infix notated equation to an prefix notated equation.
/// # Example
/// a + b -> + a b
///
/// TODO: capture and fix negative signifier
///
/// a + -b -> b - + a -> -b + a -> -b a + -> + a -b
/// ...
pub fn translate_infix(infix_notation: &str) -> String {
    debug!("input string: {}", infix_notation);
    let whitespace = Regex::new(r"^\s+?").unwrap();
    let mut ret = String::new();
    let mut stack = Vec::new();
    let word_or_punctuation = infix_notation
        .split_word_bounds()
        .filter(|x| !whitespace.is_match(x))
        .rev();
    let mut position = 0;
    let mut skip = false;
    for item in word_or_punctuation {
        if skip {
            skip = false;
            position += 1;
            continue;
        }
        debug!("item: {}", item);
        match item {
            ")" => {
                ret.push_str(" ");
                ret.push_str(item);
                debug!("ret: {}", ret);
                stack.push(item);
            }
            "=" | "+" | "-" | "*" | "/" => {
                debug!("match: operator: {}", item);
                if stack.is_empty() {
                    debug!("push '{}' onto empty stack", item);
                    stack.push(item);
                } else {
                    let stack_top = stack.pop().unwrap();
                    debug!("pop stack: {}", stack_top);
                    if get_precedence(stack_top) >= get_precedence(item) {
                        if item != ")" {
                            ret.push_str(" ");
                            ret.push_str(stack_top);
                            debug!("ret: {}", ret);
                        }
                    } else {
                        debug!("push {} back onto stack", stack_top);
                        stack.push(stack_top);
                    }
                    debug!("push {} onto stack", item);
                    stack.push(item);
                }
            }
            "(" => {
                debug!("match: left-parenthesis: {}", item);
                if stack.is_empty() {
                    ret.push_str(" ");
                    ret.push_str(item);
                    debug!("ret: {}", ret);
                } else {
                    let mut check = true;
                    while check {
                        if stack.is_empty() {
                            check = false;
                        } else {
                            let stack_top = stack.pop().unwrap();
                            debug!("pop stack: {}", stack_top);
                            if stack_top == ")" {
                                check = false;
                            } else {
                                ret.push_str(" ");
                                ret.push_str(stack_top);
                                debug!("ret: {}", ret);
                            }
                        }
                    }
                    ret.push_str(" ");
                    ret.push_str(item);
                    debug!("ret: {}", ret);
                }
            }
            _ => {
                debug!("match: all others: {}", item);
                let signed = check_signed(infix_notation, position);
                match signed {
                    Some("+") | Some("-") => {
                        debug!("returned signed: {}", signed.unwrap());
                        ret.push_str(" ");
                        ret.push_str(item);
                        ret.push_str(signed.unwrap());
                        skip = true;
                    },
                    _ => {
                        debug!("returned signed: None");
                        ret.push_str(" ");
                        ret.push_str(item);
                    }
                }
                debug!("ret: {}", ret);
            }
        }
        position += 1;
    }
    debug!("empty stack");
    while !stack.is_empty() {
        let stack_top = stack.pop().unwrap();
        debug!("pop stack: {}", stack_top);
        ret.push_str(" ");
        ret.push_str(stack_top);
        debug!("ret: {}", ret);
    }

    debug!("ret: {}", ret);
    let rev_ret = ret.split_word_bounds()
        .rev()
        .collect::<String>()
        .trim()
        .to_string();
    debug!("rev_ret: {}", rev_ret);
    return rev_ret;
}

fn get_precedence(op: &str) -> i32 {
    match op {
        "=" => 3,
        "*" | "/" => 2,
        "+" | "-" => 1,
        _ => 0,
    }
}

fn check_signed(infix_notation: &str, position: usize) -> Option<&str> {
    let whitespace = Regex::new(r"^\s+?").unwrap();
    let mut local = infix_notation
        .split_word_bounds()
        .filter(|x| !whitespace.is_match(x))
        .rev();
    let local_first_item = local.nth(position + 1);
    let local_second_item = local.next();

    match local_first_item {
        None => {
            debug!("local_first_item: None");
                return None;
        },
        _ => {
            debug!("local_first_item: {}", local_first_item.unwrap());
            match local_second_item {
                Some("+") | Some("-") | Some("*") | Some("/") | Some("=") => {
                    debug!("local_second_item: {}", local_second_item.unwrap());
                    return local_first_item;
                },
                None => {
                    debug!("local_second_item: None");
                    return None;
                },
                _ => {
                    debug!("local_second_item: {}", local_second_item.unwrap());
                    return None;
                },
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_add() {
        let result = "+ 2 2";
        let input = "2 + 2";
        assert_eq!(result, translate_infix(input));
    }

    #[test]
    fn test_simple_subtract() {
        let result = "- 2 2";
        let input = "2 - 2";
        assert_eq!(result, translate_infix(input));
    }

    #[test]
    fn test_simple_multiply() {
        let result = "* 2 2";
        let input = "2 * 2";
        assert_eq!(result, translate_infix(input));
    }

    #[test]
    fn test_simple_divide() {
        let result = "/ 2 2";
        let input = "2 / 2";
        assert_eq!(result, translate_infix(input));
    }

    #[test]
    fn test_complex_one() {
        let input = "2 + pi / 35";
        let result = "+ 2 / pi 35";
        assert_eq!(result, translate_infix(input));
    }

    #[test]
    fn test_complex_two() {
        let input = "a + b * c / d";
        let result = "+ a * b / c d";
        assert_eq!(result, translate_infix(input));
    }
    #[test]
    fn test_complex_three() {
        let input = "(a + b * c) / (d - f / g)";
        let result = "/ ( + a * b c ) ( - d / f g )";
        assert_eq!(result, translate_infix(input));
    }
    #[test]
    fn test_complex_four() {
        let input = "(a + b * c / (d - f / (g * h / i)))";
        let result = "( + a * b / c ( - d / f ( * g / h i ) ) )";
        assert_eq!(result, translate_infix(input));
    }
    #[test]
    fn test_complex_five() {
        let input = "(j + k) * (a + b * c / (d - f / (g * h / i)))";
        let result = "* ( + j k ) ( + a * b / c ( - d / f ( * g / h i ) ) )";
        assert_eq!(result, translate_infix(input));
    }
    #[test]
    fn test_complex_six() {
        let input = "(a + b * c / (d - f / (g * h / i))) + (j + k)";
        let result = "+ ( + a * b / c ( - d / f ( * g / h i ) ) ) ( + j k )";
        assert_eq!(result, translate_infix(input));
    }
    #[test]
    fn test_prefix_equals() {
        let input = "a = 3";
        let result = "= a 3";
        assert_eq!(result, translate_infix(input));
    }

    #[test]
    fn test_prefix_equals_negative() {
        let input = "a = -3";
        let result = "= a -3";
        assert_eq!(result, translate_infix(input));
    }

    #[test]
    fn test_prefix_equals_negative_2() {
        let input = "(a + b * c / (d - f / (g * h / -i))) + (j + k)";
        let result = "+ ( + a * b / c ( - d / f ( * g / h -i ) ) ) ( + j k )";
        assert_eq!(result, translate_infix(input));
    }
}
