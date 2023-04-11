use std::collections::VecDeque;

struct StoneWall;

impl StoneWall {
    fn solution(h: &Vec<i32>) -> i32 {
        let mut stack: VecDeque<i32> = VecDeque::new();
        let mut stones = 0;

        for height in h {
            while !stack.is_empty() && stack.front().unwrap() > height {
                stack.pop_front();
            }
            if !stack.is_empty() && stack.front().unwrap() == height {
                continue;
            }

            stones += 1;
            stack.push_front(*height);
        }
        stones
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stone_wall() {
        assert_eq!(StoneWall::solution(&vec![8, 8, 5, 7, 9, 8, 7, 4, 8]), 7);
    }
}
