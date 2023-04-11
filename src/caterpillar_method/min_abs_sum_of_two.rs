use std::cmp::min;

struct MinAbsSumOfTwo;

impl MinAbsSumOfTwo {
    fn solution(a: &mut Vec<i32>) -> usize {
        if a.len() == 1 {
            return (a[0] + a[0]).abs() as usize;
        }

        a.sort();

        let total_min = a.iter().min();
        if total_min >= Some(&0) {
            return 2 * a[0] as usize;
        }

        let mut minimal_sum = i32::MAX;

        let mut p = 0;
        let mut q = a.len() - 1;
        while p < q {
            minimal_sum = min(minimal_sum, (a[p] + a[q]).abs());
            minimal_sum = min(minimal_sum, (a[p] + a[p]).abs());
            minimal_sum = min(minimal_sum, (a[q] + a[q]).abs());

            if (a[p + 1] + a[q]).abs() <= (a[p] + a[q - 1]).abs() {
                p += 1;
            } else {
                q -= 1;
            }
        }

        min(minimal_sum, (a[p] + a[q]).abs()) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_abs_sum_of_two() {
        assert_eq!(MinAbsSumOfTwo::solution(&mut vec![1, 4, -3]), 1);
        assert_eq!(MinAbsSumOfTwo::solution(&mut vec![-8, 4, 5, -10, 3]), 3);
    }
}
