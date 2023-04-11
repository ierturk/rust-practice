struct NailingPlanks;

impl NailingPlanks {
    fn check_all_planks_nailed(a: &Vec<i32>, b: &Vec<i32>, c: &Vec<i32>, nail_count: i32) -> bool {
        let mut presum_nailed_status = vec![0_i32; 2 * c.len() + 1];

        for i in 0..(nail_count as usize) {
            presum_nailed_status[c[i] as usize] = 1;
        }

        for i in 1..presum_nailed_status.len() {
            presum_nailed_status[i] += presum_nailed_status[i - 1];
        }

        let mut all_nailed = true;
        for i in 0..a.len() {
            if !all_nailed {
                all_nailed = !all_nailed;
                break;
            }
            all_nailed =
                presum_nailed_status[b[i] as usize] - presum_nailed_status[a[i] as usize - 1] > 0;
        }

        all_nailed
    }

    fn solution(a: &mut Vec<i32>, b: &mut Vec<i32>, c: &mut Vec<i32>) -> i32 {
        let mut result = -1;

        let mut min_nails = 1_i32;
        let mut max_nails = c.len() as i32;

        while min_nails <= max_nails {
            let mid = (max_nails + min_nails) / 2_i32;
            if NailingPlanks::check_all_planks_nailed(a, b, c, mid) {
                result = mid;
                max_nails = mid - 1;
            } else {
                min_nails = mid + 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nailing_planks() {
        assert_eq!(
            NailingPlanks::solution(
                &mut vec![1, 4, 5, 8],
                &mut vec![4, 5, 9, 10],
                &mut vec![4, 6, 7, 10, 2]
            ),
            4
        );
    }
}
