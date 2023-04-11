struct CountFactors;

impl CountFactors {
    fn solution(n: i32) -> i32 {
        if n == 1 {
            return 1;
        } else if n == 2 {
            return 2;
        }

        let mut factors = 2;
        let root_n = (n as f64).sqrt() as i32;

        let mut exact_root_n = false;

        if (n as f64).sqrt() == root_n as f64 {
            exact_root_n = true;
        }

        for i in 2..=root_n {
            if (n % i) == 0 {
                if i == root_n && exact_root_n {
                    factors += 1;
                } else {
                    factors += 2;
                }
            }
        }

        factors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_factors() {
        assert_eq!(CountFactors::solution(24), 8);
    }
}
