use std::cmp::{max, min};

struct MinAbsSum;

impl MinAbsSum {
    fn solution(a: &mut Vec<i32>) -> u32 {
        let n: usize = a.len();

        if n == 0 {
            return 0;
        }

        let mut max_val = 0_i32;

        a.iter_mut().for_each(|x| {
            max_val = max(max_val, *x);
            *x = x.abs()
        });

        let mut count = vec![0_i32; (max_val + 1) as usize];
        for e in &mut *a {
            count[*e as usize] += 1;
        }

        let sum = a.iter().sum();
        let mut dp = vec![-1_i32; (sum + 1) as usize];

        dp[0] = 0;

        for a in 1..max_val + 1 {
            if count[a as usize] > 0 {
                for j in 0..sum {
                    if dp[j as usize] >= 0 {
                        dp[j as usize] = count[a as usize];
                    } else if j >= a && dp[j as usize - a as usize] > 0 {
                        dp[j as usize] = dp[j as usize - a as usize] - 1;
                    }
                }
            }
        }

        let mut result = sum;

        for i in 0..(sum / 2 + 1) {
            if dp[i as usize] >= 0 {
                result = min(result, sum - 2 * i);
            }
        }

        result as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_abs_sum() {
        assert_eq!(MinAbsSum::solution(&mut vec![1, 5, 2, -2]), 0);
    }
}
