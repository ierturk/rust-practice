struct CountSemiprimes;

impl CountSemiprimes {
    fn get_fact_arr(n: usize) -> Vec<i32> {
        let mut f = vec![0_i32; n + 1];

        f[1] = 1;
        let mut i = 2;

        while i * i <= n {
            if f[i] == 0 {
                let mut k = i * i;
                while k <= n {
                    if f[k] == 0 {
                        f[k] = i as i32;
                    }
                    k += i;
                }
            }
            i += 1;
        }
        f
    }

    fn solution(n: usize, p: &Vec<i32>, q: &Vec<i32>) -> Vec<i32> {
        let f = CountSemiprimes::get_fact_arr(n);
        let mut prefix_semi_primes = vec![0_i32; n + 1];

        for x in 1..=n {
            if f[x] > 0 && f[x / f[x] as usize] == 0 {
                prefix_semi_primes[x] += 1;
            }
            prefix_semi_primes[x] += prefix_semi_primes[x - 1];
        }

        let m = p.len();
        let mut ans = vec![0_i32; m];

        for i in 0..m {
            ans[i] = prefix_semi_primes[q[i] as usize] - prefix_semi_primes[p[i] as usize - 1];
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_semiprimes() {
        assert_eq!(
            CountSemiprimes::solution(26, &vec![1, 4, 16], &vec![26, 10, 20]),
            vec![10, 4, 0]
        );
    }
}
