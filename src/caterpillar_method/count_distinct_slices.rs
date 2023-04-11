use std::cmp::min;

struct CountDistinctSlices;

impl CountDistinctSlices {
    fn solution(m: usize, a: &mut Vec<u32>) -> u32 {
        let n = a.len();
        let mut z = vec![0; m + 1];
        let mut distinct_slices = 0;
        let mut j = 0;
        let mut sequence_length = 0;

        for i in 0..n {
            z[a[i] as usize] += 1;
            if z[a[i] as usize] < 2 {
                sequence_length += 1;
                distinct_slices += sequence_length;
                continue;
            }

            let mut removed_duplicate = false;

            while j < i && !removed_duplicate {
                z[a[j as usize] as usize] -= 1;

                if z[a[j as usize] as usize] > 0 {
                    distinct_slices += sequence_length;
                    removed_duplicate = true;
                } else {
                    sequence_length -= 1;
                }
                j += 1;
            }
        }

        min(distinct_slices, 1_000_000_000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_distinct_slices() {
        assert_eq!(
            CountDistinctSlices::solution(6, &mut vec![3, 4, 5, 5, 2]),
            9
        );
    }
}
