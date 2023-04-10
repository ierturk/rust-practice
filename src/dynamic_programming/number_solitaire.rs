use std::cmp::max;

struct NumberSolitaire;

impl NumberSolitaire {
    fn solution(a: &Vec<i32>) -> i32 {
        let n = a.len();

        if n == 2 {
            return a[0] + a[1];
        }

        let mut max_sum = vec![i32::MIN; n];

        max_sum[0] = a[0];

        for i in 1..n {
            for dice in 1..7 {
                if dice > i {
                    break;
                }
                max_sum[i] = max(max_sum[i], a[i] + max_sum[i - dice]);
            }
        }

        max_sum[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_solitaire() {
        assert_eq!(NumberSolitaire::solution(&vec![1, -2, 0, 9, -1, -2]), 8);
    }
}
