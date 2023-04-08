pub fn solution(mut a: Vec<i32>) -> i32 {
    // int solution(vector<int> &A);

    let n = a.len();

    a.iter_mut().fold(0, |acc, x| {
        *x += acc;
        *x
    });
    // println!("{:?}", a);

    let mut min_diff = i64::MAX;

    let total_sum = a[n-1] as i64;

    for i in a {
        let diff = ( 2 * (i as i64) - total_sum ).abs();
        if diff < min_diff {
            min_diff = diff;
        }
    }

    min_diff as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tape_equilibrium() {
        assert_eq!(solution(vec![3, 1, 2, 4, 3]), 1);
    }
}