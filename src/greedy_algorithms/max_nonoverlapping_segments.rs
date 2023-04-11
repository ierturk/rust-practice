struct MaxNonoverlappingSegments;

impl MaxNonoverlappingSegments {
    fn solution(a: &mut Vec<i32>, b: &mut Vec<i32>) -> u32 {
        let a_len = a.len();
        let b_len = b.len();

        if a_len == 0 || a_len != b_len {
            return 0;
        }

        let mut count = 1;
        let mut prev_end = b[0];

        for i in 1..a_len {
            if a[i] > prev_end {
                count += 1;
                prev_end = b[i];
            }
        }

        return count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_nonoverlapping_segments() {
        assert_eq!(
            MaxNonoverlappingSegments::solution(
                &mut vec![1, 3, 7, 9, 9],
                &mut vec![5, 6, 8, 9, 10]
            ),
            3
        );
    }
}
