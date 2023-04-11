struct CountTriangles;

impl CountTriangles {
    fn solution(a: &mut Vec<u32>) -> usize {
        let n = a.len();
        let mut triangles = 0;

        a.sort();

        for p in 0..(n - 2) {
            let mut r = p + 2;

            for q in (p + 1)..(n - 1) {
                while (r < n) && (a[p] > (a[r] - a[q])) {
                    r += 1;
                }
                triangles += r - q - 1;
            }
        }
        triangles
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_triangles() {
        assert_eq!(CountTriangles::solution(&mut vec![10, 2, 5, 1, 8, 12]), 4);
    }
}
