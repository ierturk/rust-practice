struct Peaks;

impl Peaks {
    fn solution(a: &Vec<i32>) -> i32 {
        let n = a.len();
        let mut peaks: Vec<i32> = Vec::new();

        for i in 1..n - 1 {
            if a[i] > a[i - 1] && a[i] > a[i + 1] {
                peaks.push(i as i32);
            }
        }

        for size in 1..=n {
            if n % size != 0 {
                continue;
            }

            let mut find = 0;
            let groups = n / size;
            let mut ok = true;

            for peak_idx in &peaks {
                if *peak_idx / (size as i32) > find {
                    ok = false;
                    break;
                }

                if *peak_idx / (size as i32) == find {
                    find += 1;
                }
            }
            if find != groups as i32 {
                ok = false
            };

            if ok {
                return groups as i32;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_prime_divisors() {
        assert_eq!(
            Peaks::solution(&vec![1, 2, 3, 4, 3, 4, 1, 2, 3, 4, 6, 2]),
            3
        );
    }
}
