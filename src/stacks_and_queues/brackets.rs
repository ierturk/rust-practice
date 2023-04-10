use std::collections::VecDeque;

struct Brackets;

impl Brackets {
    fn solution(s: &String) -> u32 {
        let mut deque: VecDeque<char> = VecDeque::new();

        for c in s.chars() {
            match c {
                '(' | '{' | '[' => deque.push_front(c),
                ')' => {
                    if deque.front() == Some(&'(') {
                        deque.pop_front();
                    } else {
                        return 0;
                    }
                }
                '}' => {
                    if deque.front() == Some(&'{') {
                        deque.pop_front();
                    } else {
                        return 0;
                    }
                }
                ']' => {
                    if deque.front() == Some(&'[') {
                        deque.pop_front();
                    } else {
                        return 0;
                    }
                }
                _ => (),
            }
        }

        if deque.is_empty() {
            return 1;
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brackets() {
        assert_eq!(Brackets::solution(&"{[()()]}".to_string()), 1);
        assert_eq!(Brackets::solution(&"([)()]".to_string()), 0);
    }
}
