struct MissingInteger;

impl MissingInteger {
    fn solution(&self, a: &Vec<i32>) -> u32 {
        let mut test_arr = vec![false; 1000_000_001];

        for e in a {
            if *e > 0 {
                test_arr[*e as usize] = true;
            }
        }

        for (i, c) in test_arr.into_iter().enumerate() {
            if i > 0 && !c {
                return i as u32;
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_counters() {
        let max_counters = MissingInteger;

        assert_eq!(max_counters.solution(vec![1, 3, 6, 4, 1, 2].as_ref()), 5);
        assert_eq!(max_counters.solution(vec![1, 2, 3].as_ref()), 4);
        assert_eq!(max_counters.solution(vec![-1, -3].as_ref()), 1);
    }
}
