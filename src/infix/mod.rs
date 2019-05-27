use unicode_segmentation::UnicodeSegmentation;
use regex::Regex;

/// Function that translates a prefix notated equation to an infix notated equation.
/// # Example
/// + a b -> a + b
/// ...
pub fn translate_prefix(prefix_notation: &str) -> String {
    debug!("input string: {}", prefix_notation);
    let whitespace = Regex::new(r"^\s+?").unwrap();
    let mut ret = String::new();
    let mut operators = Vec::new();
    let mut word_or_punctuation = prefix_notation
        .split_word_bounds()
        .filter(|x| !whitespace.is_match(x))
        .peekable();

    loop {
        let item = word_or_punctuation.next();
        debug!("item: {}", item.unwrap());
        match item {
            Some("(") => {
                ret.push_str(item.unwrap());
                debug!("ret: {}", ret);
            }
            Some(")") => {
                ret.push_str(item.unwrap());
                match word_or_punctuation.peek() {
                    Some(&")") => {
                        // Do nothing.
                    }
                    _ => if !operators.is_empty() {
                        ret.push_str(" ");
                        ret.push_str(operators.pop().unwrap());
                        ret.push_str(" ");
                    },
                }
            }
            Some("=") | Some("+") | Some("-") | Some("*") | Some("/") => {
                debug!("match: operator: {}", item.unwrap());
                operators.push(item.unwrap());
            }
            Some(_) => {
                debug!("match: all others: {}", item.unwrap());
                ret.push_str(item.unwrap());
                if operators.is_empty() {
                    // do nothing

                } else {
                    match word_or_punctuation.peek() {
                        Some(&")") => {
                            // do nothing
                        }
                        _ => if !operators.is_empty() {
                            ret.push_str(" ");
                            ret.push_str(operators.pop().unwrap());
                            ret.push_str(" ");
                        },
                    }
                }
            }
            None => break,
        }
    }

    return ret;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_add() {
        let input = "+ 2 2";
        let result = "2 + 2";
        assert_eq!(result, translate_prefix(input));
    }

    #[test]
    fn test_simple_subtract() {
        let input = "- 2 2";
        let result = "2 - 2";
        assert_eq!(result, translate_prefix(input));
    }

    #[test]
    fn test_simple_multiply() {
        let input = "* 2 2";
        let result = "2 * 2";
        assert_eq!(result, translate_prefix(input));
    }

    #[test]
    fn test_simple_divide() {
        let input = "/ 2 2";
        let result = "2 / 2";
        assert_eq!(result, translate_prefix(input));
    }

    #[test]
    fn test_complex_one() {
        let result = "2 + pi / 35";
        let input = "+ 2 / pi 35";
        assert_eq!(result, translate_prefix(input));
    }

    #[test]
    fn test_complex_two() {
        let result = "a + b * c / d";
        let input = "+ a * b / c d";
        assert_eq!(result, translate_prefix(input));
    }
    #[test]
    fn test_complex_three() {
        let result = "(a + b * c) / (d - f / g)";
        let input = "/ ( + a * b c ) ( - d / f g )";
        assert_eq!(result, translate_prefix(input));
    }
    #[test]
    fn test_complex_four() {
        let result = "(a + b * c / (d - f / (g * h / i)))";
        let input = "( + a * b / c ( - d / f ( * g / h i ) ) )";
        assert_eq!(result, translate_prefix(input));
    }
    #[test]
    fn test_complex_five() {
        let result = "(j + k) * (a + b * c / (d - f / (g * h / i)))";
        let input = "* ( + j k ) ( + a * b / c ( - d / f ( * g / h i ) ) )";
        assert_eq!(result, translate_prefix(input));
    }
    #[test]
    fn test_complex_six() {
        let result = "(a + b * c / (d - f / (g * h / i))) + (j + k)";
        let input = "+ ( + a * b / c ( - d / f ( * g / h i ) ) ) ( + j k )";
        assert_eq!(result, translate_prefix(input));
    }
    #[test]
    fn test_infix_equals() {
        let result = "a = 3";
        let input = "= a 3";
        assert_eq!(result, translate_prefix(input));
    }
}
