pub fn solution(arr: Vec<i32>, k: usize) -> Vec<i32> {

    let n:usize = arr.len();
    let mut res: Vec<i32> = vec![0; n];
    let rs = k%n;

    for i in 0..n {
        res[i] = if i <= rs {
            arr[i + rs]
        } else {
            arr[i - rs - 1]
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cyclic_rotation() {
        assert_eq!(solution(vec![0, 1, 2, 3, 4], 2), vec![2, 3, 4, 0, 1]);
        assert_eq!(solution(vec![0, 1, 2, 3, 4], 7), vec![2, 3, 4, 0, 1]);

    }
}