use std::cmp::max;

struct MaxProductOfThree;

impl MaxProductOfThree {
    fn solution(a: &mut Vec<i32>) -> usize {
        a.sort();
        let n = a.len();

        max(
            (a[0] * a[1] * a[n - 1]) as usize,
            (a[n - 3] * a[n - 2] * a[n - 1]) as usize,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_product_of_three() {
        assert_eq!(
            MaxProductOfThree::solution(&mut vec![-3, 1, 2, -2, 5, 6]),
            60
        );
    }
}
