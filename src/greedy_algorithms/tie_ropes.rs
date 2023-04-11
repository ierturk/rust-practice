struct TieRopes;

impl TieRopes {
    fn solution(k: u32, a: &mut Vec<u32>) -> u32 {
        let mut count = 0;
        let mut rop_sum = 0;

        for e in a {
            rop_sum += *e;
            if rop_sum >= k {
                count += 1;
                rop_sum = 0;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tie_ropes() {
        assert_eq!(TieRopes::solution(4, &mut vec![1, 2, 3, 4, 1, 1, 3]), 3);
    }
}
