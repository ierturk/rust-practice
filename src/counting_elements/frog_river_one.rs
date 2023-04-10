use std::collections::HashMap;

pub fn solution(x: i32, a: &Vec<i32>) -> i32 {

    let mut step = HashMap::new();

    for (i, s ) in a.iter().enumerate() {
        step.entry(*s).or_insert(true);

        if step.len() == x as usize {
            return i as i32
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frog_river_one() {
        assert_eq!( solution(5, vec![1, 3, 1, 4, 2, 3, 5, 4].as_ref() ), 6 );
    }
}