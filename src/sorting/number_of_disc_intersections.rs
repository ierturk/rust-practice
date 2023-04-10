struct NumberOfDiscIntersections;

impl NumberOfDiscIntersections {
    fn solution(a: &mut Vec<i32>) -> i32 {
        let n = a.len();
        let mut lower = vec![0_i32; n];
        let mut upper = vec![0_i32; n];

        for i in 0..n {
            lower[i] = i as i32 - a[i];
            upper[i] = i as i32 + a[i];
        }

        lower.sort();
        upper.sort();

        let mut intersection = 0_i32;
        let mut j = 0;

        for i in 0..n {
            while j < n && upper[i] >= lower[j] {
                intersection += j as i32;
                intersection -= i as i32;
                j += 1;
            }
        }

        if intersection > 10_000_000 {
            return -1;
        }

        intersection as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_disc_intersections() {
        assert_eq!(
            NumberOfDiscIntersections::solution(&mut vec![1, 5, 2, 1, 4, 0]),
            11
        );
    }
}
