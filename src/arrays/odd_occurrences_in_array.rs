pub fn solution(arr: &Vec<u32>) -> usize {

    const LIMIT: usize = 1_000_000_001;
    let mut test_arr = vec![0_u32; LIMIT];

    for a in arr {
        test_arr[*a as usize] += 1;
    }

    for (i , v) in test_arr.into_iter().enumerate() {
        if v != 0 && v%2 != 0 {
            return i;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn odd_occurrences_in_array() {
        assert_eq!(solution(vec![9, 3, 9, 3, 7, 9].as_ref()), 7);
    }
}