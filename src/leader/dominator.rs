struct Dominator;

impl Dominator {
    fn solution(a: &Vec<i32>) -> i32 {
        let mut candidate = -1;
        let mut size = 0;

        for e in a {
            if size == 0 {
                size += 1;
                candidate = *e;
            } else {
                if candidate == *e {
                    size += 1;
                } else {
                    size -= 1;
                }
            }
        }

        if size > 0 {
            let mut index = -1_i32;
            let mut count = 0;
            for i in 0..a.len() {
                if a[i] == candidate {
                    count += 1;
                    if index == -1 {
                        index = i as i32;
                    }
                }
            }

            if count > a.len() / 2 {
                return index;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dominator() {
        assert_eq!(Dominator::solution(&vec![3, 4, 3, 2, 3, -1, 3, 3]), 0);
    }
}
