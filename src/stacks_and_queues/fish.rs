use std::collections::VecDeque;

struct Fish;

impl Fish {
    fn solution(a: &mut Vec<i32>, b: &mut Vec<i32>) -> i32 {
        let n = a.len();
        let mut alive = a.len();

        let mut f: VecDeque<i32> = VecDeque::new();

        for i in 0..n {
            if b[i] == 1 {
                f.push_front(i as i32);
            } else if b[i] == 0 && !f.is_empty() {
                while !f.is_empty() && a[*f.front().unwrap() as usize] < a[i] {
                    f.pop_front();
                    alive -= 1;
                }

                if !f.is_empty() && a[*f.front().unwrap() as usize] > a[i] {
                    alive -= 1;
                }
            }
        }

        alive as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fish() {
        assert_eq!(
            Fish::solution(&mut vec![4, 3, 2, 1, 5], &mut vec![0, 1, 0, 0, 0]),
            2
        );
    }
}
