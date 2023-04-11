use std::cmp::max;

struct MaxProfit;

impl MaxProfit {
    fn solution(a: &Vec<i32>) -> i32 {
        let n = a.len();
        if n == 0 {
            return 0;
        }

        let mut max_profit = 0;
        let mut profit_sum = 0;
        for i in 1..n {
            profit_sum = max(0, profit_sum + a[i] - a[i - 1]);
            max_profit = max(max_profit, profit_sum);
        }

        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        assert_eq!(
            MaxProfit::solution(&vec![23171, 21011, 21123, 21366, 21013, 21367]),
            356
        );
    }
}
