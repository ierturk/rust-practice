use std::cmp::max;

struct MaxDoubleSliceSum;

impl MaxDoubleSliceSum {
    fn solution(a: &Vec<i32>) -> i32 {
        let n = a.len();
        if n == 3 {
            return 0;
        }

        let mut sum_pref = vec![0_i32; n];
        let mut sum_suff = vec![0_i32; n];

        for i in 1..(n - 1) {
            sum_pref[i] = max(sum_pref[i - 1] + a[i], 0);
        }

        for i in (1..(n - 1)).rev() {
            sum_suff[i] = max(sum_suff[i + 1] + a[i], 0);
        }

        let mut double_slice = 0;
        for i in 1..(n - 1) {
            double_slice = max(double_slice, sum_pref[i - 1] + sum_suff[i + 1]);
        }

        double_slice
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_double_slice_sum() {
        assert_eq!(
            MaxDoubleSliceSum::solution(&vec![3, 2, 6, -1, 4, 5, -1, 2]),
            17
        );
    }
}
