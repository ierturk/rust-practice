use std::cmp;

struct MaxCounters;

impl MaxCounters {
    fn solution(&self, x: usize, a: &Vec<u32>) -> Vec<u32> {

        let mut cnts = vec![0_u32; x];
        let mut min: u32 = 0;
        let mut max: u32 = 0;

        for e in a {
            if *e <= x as u32 {
                let cnt = cmp::max(cnts[(*e-1) as usize], min) + 1;
                cnts[(*e-1) as usize] = cnt;
                if cnt > max {
                    max = cnt;
                }
            } else if *e-1 == x as u32 {
                min = max;
            }
        }

        for c in &mut cnts {
            *c = cmp::max(*c, min);
        }

        cnts
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_counters() {
        let max_counters = MaxCounters;

        assert_eq!( max_counters.solution(5, vec![3, 4, 4, 6, 1, 4, 4].as_ref() ),
                    vec![3, 2, 2, 4, 2] );
    }
}