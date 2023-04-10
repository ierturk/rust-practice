pub fn solution(a: Vec<i32>) -> i32 {
    let n = a.len() as i32;
    let sum: i32 = a.iter().sum();

    (n + 1) * (n + 2) / 2 - sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perm_missing_elem() {
        assert_eq!(solution(vec![1, 2, 3, 5, 6]), 4);
    }
}
