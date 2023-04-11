use std::collections::HashMap;

struct AbsDistinct;

impl AbsDistinct {
    fn solution(a: &mut Vec<i32>) -> u32 {
        let mut map = HashMap::new();

        for e in a {
            map.entry((*e).abs()).or_insert(true);
        }

        map.len() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abs_distinct() {
        assert_eq!(AbsDistinct::solution(&mut vec![-5, -3, -1, 0, 3, 6],), 5);
    }
}
