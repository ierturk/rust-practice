struct MinMaxDivision;

impl MinMaxDivision {
    fn check(a: &Vec<i32>, k_max_block_count: i32, mid_sum_candidate: i32) -> bool {
        let mut blocks = 1;
        let mut sum = 0;

        for e in a {
            sum += *e;
            if sum > mid_sum_candidate {
                blocks += 1;
                sum = *e;
            }

            if blocks > k_max_block_count {
                return false;
            }
        }
        true
    }

    fn solution(k: i32, _m: i32, a: &mut Vec<i32>) -> i32 {
        let n = a.len();
        let mut min_sum = *a.iter().max().unwrap();
        let mut max_sum = a.iter().sum();

        if n == 1 {
            return min_sum;
        }
        if k == 1 {
            return max_sum;
        }

        let mut result = min_sum;

        while min_sum <= max_sum {
            let mid_sum_candidate = (min_sum + max_sum) / 2;

            if MinMaxDivision::check(a, k, mid_sum_candidate) {
                result = mid_sum_candidate;
                max_sum = mid_sum_candidate - 1;
            } else {
                min_sum = mid_sum_candidate + 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_max_division() {
        assert_eq!(
            MinMaxDivision::solution(3, 5, &mut vec![2, 1, 5, 1, 2, 2, 2]),
            6
        );
    }
}
