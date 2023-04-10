struct Triangle;

impl Triangle {
    fn solution(a: &mut Vec<i32>) -> usize {
        a.sort();
        let n = a.len();

        for i in 0..n - 2 {
            if a[i] + a[i + 1] > a[i + 2] {
                return 1;
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle() {
        assert_eq!(Triangle::solution(&mut vec![10, 2, 5, 1, 8, 20]), 1);
    }
}
