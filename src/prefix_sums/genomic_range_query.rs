struct GenomicRangeQuery {
    s: String,
    p: Vec<u32>,
    q: Vec<u32>,
}

impl GenomicRangeQuery {
    fn solution(&self) -> Vec<i32> {
        const A_VAL: i32 = 1;
        const C_VAL: i32 = 2;
        const G_VAL: i32 = 3;
        const T_VAL: i32 = 4;

        let n = self.s.len();
        let mut a: Vec<i32> = vec![-1; n];
        let mut c: Vec<i32> = vec![-1; n];
        let mut g: Vec<i32> = vec![-1; n];
        let mut t: Vec<i32> = vec![-1; n];

        let mut a_last_index: i32 = -1;
        let mut c_last_index: i32 = -1;
        let mut g_last_index: i32 = -1;
        let mut t_last_index: i32 = -1;

        for (i, s) in self.s.chars().into_iter().enumerate() {
            match s {
                'A' => a_last_index = i as i32,
                'C' => c_last_index = i as i32,
                'G' => g_last_index = i as i32,
                'T' => t_last_index = i as i32,
                _ => unimplemented!(),
            }

            a[i] = a_last_index;
            c[i] = c_last_index;
            g[i] = g_last_index;
            t[i] = t_last_index;
        }

        let m = self.p.len();
        let mut result: Vec<i32> = vec![0; m];

        for i in 0..m {
            let l = self.p[i] as i32;
            let r = self.q[i] as i32;

            if a[r as usize] >= l {
                result[i] = A_VAL;
            } else if c[r as usize] >= l {
                result[i] = C_VAL;
            } else if g[r as usize] >= l {
                result[i] = G_VAL;
            } else if t[r as usize] >= l {
                result[i] = T_VAL;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genomic_range_query() {
        let genomic_range_query = GenomicRangeQuery {
            s: String::from("CAGCCTA"),
            p: vec![2, 5, 0],
            q: vec![4, 5, 6],
        };
        assert_eq!(genomic_range_query.solution(), vec![2, 4, 1]);
    }
}
