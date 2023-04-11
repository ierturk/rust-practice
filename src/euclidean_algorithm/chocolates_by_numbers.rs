struct ChocolatesByNumbers;

impl ChocolatesByNumbers {
    fn gcd_by_division(a: i32, b: i32) -> i32 {
        if a % b == 0 {
            b
        } else {
            ChocolatesByNumbers::gcd_by_division(b, a % b)
        }
    }

    fn solution(n: i32, m: i32) -> i32 {
        n / ChocolatesByNumbers::gcd_by_division(n, m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chocolates_by_numbers() {
        assert_eq!(ChocolatesByNumbers::solution(10, 4), 5);
    }
}
