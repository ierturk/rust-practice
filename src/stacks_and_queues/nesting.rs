struct Nesting;

impl Nesting {
    fn solution(s: &str) -> i32 {
        if s.is_empty() {
            return 1;
        }

        if s.len() % 2 != 0 {
            return 0;
        }

        let mut open = 0;
        for c in s.chars() {
            if c == '(' {
                open += 1;
            } else {
                if open > 0 {
                    open -= 1;
                } else {
                    return 0;
                }
            }
        }

        if open == 0 {
            return 1;
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nesting() {
        assert_eq!(Nesting::solution("(()(())())"), 1);
        assert_eq!(Nesting::solution("())"), 0);
    }
}
