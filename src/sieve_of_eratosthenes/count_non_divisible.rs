struct CountNonDivisible;

impl CountNonDivisible {
    fn solution(a: &Vec<i32>) -> Vec<i32> {
        let n = a.len();
        let max_element = *a.iter().max().unwrap();
        let mut counts = vec![0_i32; max_element as usize + 1];

        for i in a {
            counts[*i as usize] += 1;
        }

        let mut answer = vec![0_i32; n];

        for i in 0..n {
            let mut divisors = 0_i32;
            let num = a[i];

            let mut j = 1_i32;
            while j * j <= num {
                if num % j == 0 {
                    divisors += counts[j as usize];
                    if num / j != j {
                        divisors += counts[(num / j) as usize];
                    }
                }

                j += 1;
            }

            answer[i] = n as i32 - divisors;
        }

        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_non_divisible() {
        assert_eq!(
            CountNonDivisible::solution(&vec![3, 1, 2, 3, 6]),
            vec![2, 4, 3, 2, 0]
        );
    }
}
