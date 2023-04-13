struct CommonPrimeDivisors;

impl CommonPrimeDivisors {
    fn gcd(a: i32, b: i32, res: i32) -> i32 {
        if a == b {
            res * a
        } else if (a % 2 == 0) & (b % 2 == 0) {
            CommonPrimeDivisors::gcd(a / 2, b / 2, 2 * res)
        } else if a % 2 == 0 {
            CommonPrimeDivisors::gcd(a / 2, b, res)
        } else if b % 2 == 0 {
            CommonPrimeDivisors::gcd(a, b / 2, res)
        } else if a > b {
            CommonPrimeDivisors::gcd(a - b, b, res)
        } else {
            CommonPrimeDivisors::gcd(a, b - a, res)
        }
    }

    fn has_same_prime_divisors(mut a: i32, mut b: i32) -> bool {
        let gcd_value = CommonPrimeDivisors::gcd(a, b, 1);
        let mut gcd_a;
        let mut gcd_b;

        while a != 1 {
            gcd_a = CommonPrimeDivisors::gcd(a, gcd_value, 1);
            if gcd_a == 1 {
                break;
            }
            a = a / gcd_a;
        }
        if a != 1 {
            return false;
        }

        while b != 1 {
            gcd_b = CommonPrimeDivisors::gcd(b, gcd_value, 1);
            if gcd_b == 1 {
                break;
            }
            b = b / gcd_b;
        }
        return b == 1;
    }

    fn solution(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
        let n = a.len();
        let mut count = 0;
        for i in 0..n {
            if CommonPrimeDivisors::has_same_prime_divisors(a[i], b[i]) {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_prime_divisors() {
        assert_eq!(
            CommonPrimeDivisors::solution(&vec![15, 10, 3], &vec![75, 30, 5]),
            1
        );
    }
}
