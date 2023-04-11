struct MinPerimeterRectangle;

impl MinPerimeterRectangle {
    fn solution(n: i32) -> i32 {
        let root_n = (n as f64).sqrt() as i32;
        let mut min_perimeter = i32::MAX;

        for i in 1..=root_n {
            if n % i == 0 {
                let a = i;
                let b = n / i;
                let perimeter = 2 * (a + b);
                if perimeter < min_perimeter {
                    min_perimeter = perimeter;
                }
            }
        }

        min_perimeter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_perimeter_rectangle() {
        assert_eq!(MinPerimeterRectangle::solution(30), 22);
    }
}
