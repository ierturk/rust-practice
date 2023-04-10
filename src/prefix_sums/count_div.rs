struct CountDiv;

impl CountDiv {
    fn solution(&self, a: i32, b: i32, k: i32) -> i32 {
        b / k - a / k + if (a % k) == 0 { 1 } else { 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_div() {
        let count_div = CountDiv;
        assert_eq!(count_div.solution(6, 11, 2), 3);
    }
}
