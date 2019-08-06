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
        match item {
            ")" => {
                ret.push_str(" ");
                ret.push_str(item);
                stack.push(item);
            }
            "=" | "+" | "-" | "*" | "/" => {
                if stack.is_empty() {
                    stack.push(item);
                } else {
                    let stack_top = stack.pop().unwrap();
                    if get_precedence(stack_top) >= get_precedence(item) {
                        if item != ")" {
                            ret.push_str(" ");
                            ret.push_str(stack_top);
                        }
                    } else {
                        stack.push(stack_top);
                    }
                    stack.push(item);
                }
            }
            "(" => {
                if stack.is_empty() {
                    ret.push_str(" ");
                    ret.push_str(item);
                } else {
                    let mut check = true;
                    while check {
                        if stack.is_empty() {
                            check = false;
                        } else {
                            let stack_top = stack.pop().unwrap();
                            if stack_top == ")" {
                                check = false;
                            } else {
                                ret.push_str(" ");
                                ret.push_str(stack_top);
                            }
                        }
                    }
                    ret.push_str(" ");
                    ret.push_str(item);
                }
            }
            _ => {
                let signed = check_signed(infix_notation, position);
                match signed {
                    Some("+") | Some("-") => {
                        ret.push_str(" ");
                        ret.push_str(item);
                        ret.push_str(signed.unwrap());
                        skip = true;
                    },
                    _ => {
                        ret.push_str(" ");
                        ret.push_str(item);
                    }
                }
            }
        }
        position += 1;
    }
    while !stack.is_empty() {
        let stack_top = stack.pop().unwrap();
        ret.push_str(" ");
        ret.push_str(stack_top);
    }

    let rev_ret = ret.split_word_bounds()
        .rev()
        .collect::<String>()
        .trim()
        .to_string();
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
            return None;
        },
        _ => {
            match local_second_item {
                Some("+") | Some("-") | Some("*") | Some("/") | Some("=") => {
                    return local_first_item;
                },
                None => {
                    return None;
                },
                _ => {
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
