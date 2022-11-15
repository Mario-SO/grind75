pub fn is_valid(_s: String) -> bool {
    let mut stack = Vec::new();

    for c in _s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => {}
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(is_valid("()".to_string()), true);
    }

    #[test]
    fn ex2() {
        assert_eq!(is_valid("[]()[]".to_string()), true);
    }

    #[test]
    fn ex3() {
        assert_eq!(is_valid("([)]".to_string()), false);
    }
}
