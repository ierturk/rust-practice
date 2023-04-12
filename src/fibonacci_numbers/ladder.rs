struct Ladder;

impl Ladder {
    fn solution(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
        let na = a.len();
        // let nb = b.len();
        let max_a = *a.iter().max().unwrap();
        let max_b = *b.iter().max().unwrap();

        let mut l = vec![0_i32; na];
        let mut fib = vec![0_u32; max_a as usize + 2];
        fib[1] = 1;

        for i in 2..(max_a as usize + 2) {
            fib[i] = (fib[i - 1] + fib[i - 2]) & ((1 << max_b) - 1);
        }

        for i in 0..na {
            let mask = (1 << b[i]) - 1;
            l[i] = fib[a[i] as usize + 1] as i32 & mask;
        }

        l
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ladder() {
        assert_eq!(
            Ladder::solution(&vec![4, 4, 5, 5, 1], &vec![3, 2, 4, 3, 1]),
            vec![5, 1, 8, 0, 1]
        );
    }
}
