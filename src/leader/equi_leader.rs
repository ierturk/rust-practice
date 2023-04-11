use std::collections::VecDeque;
struct EquiLeader;

impl EquiLeader {
    fn solution(a: &Vec<i32>) -> i32 {
        let n = a.len();
        let mut candidates: VecDeque<i32> = VecDeque::new();

        for i in 0..n {
            if !candidates.is_empty() {
                let candidate = *candidates.front().unwrap();
                if candidate != a[i] {
                    candidates.pop_front();
                } else {
                    candidates.push_front(a[i]);
                }
            } else {
                candidates.push_front(a[i]);
            }
        }

        if candidates.is_empty() {
            return 0;
        }

        let candidate = *candidates.front().unwrap();
        let mut count = 0;

        for i in 0..n {
            if a[i] == candidate {
                count += 1;
            }
        }

        let half: f32 = (n as f32) / 2_f32;
        if !((count as f32) > half) {
            return 0;
        }
        let leader = candidate;
        let mut num_equi_leaders = 0;
        let mut count_x = 0;

        for i in 0..n {
            if a[i] == leader {
                count_x += 1;
            }

            let count_y = count - count_x;
            let size_x = i + 1;
            let size_y = n - size_x;

            let half_size_x: f32 = (size_x as f32) / 2_f32;
            let half_size_y: f32 = (size_y as f32) / 2_f32;

            if (count_x > half_size_x as i32) && (count_y > half_size_y as i32) {
                num_equi_leaders += 1;
            }
        }

        num_equi_leaders
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equi_leader() {
        assert_eq!(EquiLeader::solution(&vec![4, 3, 4, 4, 4, 2]), 2);
    }
}
