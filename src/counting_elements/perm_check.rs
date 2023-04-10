pub fn solution(a: &Vec<u32>) -> u32 {
    let n = a.len();
    let mut test_arr = vec![0_u32; n];

    for e in a {
        if *e as usize > n {
            return 0;
        }
        test_arr[*e as usize - 1] += 1;
        if test_arr[*e as usize - 1] > 1 {
            return 0;
        }
    }

    test_arr[0] = 1;
    for v in test_arr {
        if v == 0 {
            return 0;
        }
    }

    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perm_check() {
        assert_eq!(solution(vec![4, 1, 3, 2].as_ref()), 1);
        assert_eq!(solution(vec![4, 1, 3].as_ref()), 0);
    }
}
