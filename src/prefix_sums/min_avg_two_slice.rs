struct MinAvgTwoSlice;

impl MinAvgTwoSlice {
    fn solution(a: &Vec<i32>) -> usize {
        let n = a.len();
        let mut min_idx = 0;
        let mut min_avg = f32::MAX;

        for i in 0..n - 1 {
            let mut sum = a[i] + a[i + 1];
            let mut avg = (sum as f32) / 2_f32;

            if avg < min_avg {
                min_avg = avg;
                min_idx = i;
            }

            if i + 2 < n {
                sum += a[i + 2];
                avg = (sum as f32) / 3_f32;

                if avg < min_avg {
                    min_avg = avg;
                    min_idx = i;
                }
            }
        }

        min_idx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_avg_two_slice() {
        assert_eq!(MinAvgTwoSlice::solution(&vec![4, 2, 2, 5, 1, 5, 8]), 1);
    }
}
