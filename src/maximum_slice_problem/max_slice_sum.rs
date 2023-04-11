use std::cmp::max;

struct MaxSliceSum;

impl MaxSliceSum {
    fn solution(a: &Vec<i32>) -> i32 {
        let n = a.len();

        if n == 0 {
            return 0;
        }

        let mut max_sum = i32::MIN;
        let mut sum_pre = 0;

        for i in 1..n {
            sum_pre = max(sum_pre + a[i - 1], a[i - 1]);
            max_sum = max(max_sum, sum_pre);
        }

        max_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_slice_sum() {
        assert_eq!(MaxSliceSum::solution(&vec![3, 2, -6, 4, 0]), 5);
    }
}
