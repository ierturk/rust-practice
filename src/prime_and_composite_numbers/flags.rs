use std::cmp::max;

struct Flags;

impl Flags {
    fn get_peaks(a: &Vec<i32>) -> Vec<bool> {
        let n = a.len();
        let mut p = vec![false; n];

        for x in 1..n - 1 {
            if a[x] > a[x - 1] && a[x] > a[x + 1] {
                p[x] = true;
            }
        }
        p
    }

    fn get_next(a: &Vec<i32>, peaks: &Vec<bool>) -> Vec<i32> {
        let n = a.len();
        let mut next = vec![0_i32; n];

        next[n - 1] = -1;
        for i in (0..=(n - 2)).rev() {
            if peaks[i] {
                next[i] = i as i32;
            } else {
                next[i] = next[i + 1];
            }
        }
        next
    }

    fn solution(a: &Vec<i32>) -> i32 {
        let n = a.len();

        if n < 3 {
            return 0;
        }

        let p: Vec<bool> = Flags::get_peaks(a);
        let next: Vec<i32> = Flags::get_next(a, &p);

        let mut max_flags = 0;
        let mut d = 1_i32;
        while d * (d - 1) <= n.try_into().unwrap() {
            let mut current_flags = 0_i32;
            let mut pos = 0_i32;
            while pos < (n as i32) && current_flags < d {
                pos = next[pos as usize];
                if pos == -1 {
                    break;
                }
                pos += d;
                current_flags += 1;
            }
            max_flags = max(max_flags, current_flags);

            d += 1;
        }

        max_flags
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flags() {
        assert_eq!(
            Flags::solution(&vec![1, 5, 3, 4, 3, 4, 1, 2, 3, 4, 6, 2]),
            3
        );
    }
}
