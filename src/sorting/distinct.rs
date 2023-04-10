use std::collections::HashMap;

struct Distinct;

impl Distinct {
    fn solution(a: &Vec<i32>) -> usize {
        let mut map: HashMap<i32, bool> = HashMap::new();

        for e in a {
            map.entry(*e).or_insert(true);
        }

        map.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distinct() {
        assert_eq!(Distinct::solution(&vec![2, 1, 1, 2, 3, 1]), 3);
    }
}
