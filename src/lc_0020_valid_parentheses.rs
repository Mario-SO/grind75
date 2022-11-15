pub fn is_valid(_s: String) -> bool {
    let mut valid = 0;
    for c in _s.chars() {
        match c {
            '(' => valid += 1,
            ')' => valid -= 1,
            '[' => valid += 2,
            ']' => valid -= 2,
            '{' => valid += 3,
            '}' => valid -= 3,
            _ => (),
        }
    }
    valid == 0
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
        assert_eq!(is_valid("[]]".to_string()), false);
    }

}
