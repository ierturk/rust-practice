use std::cmp::min;
use std::collections::LinkedList;

struct FibFrog;

impl FibFrog {
    fn solution(a: &Vec<i32>) -> i32 {
        let n = a.len();
        let mut locations = vec![false; n];

        let mut f = vec![0, 1];
        f[1] = 1;

        let mut i = 2_usize;
        while f[i - 1] + f[i - 2] <= 100000 {
            f.push(f[i - 1] + f[i - 2]);
            i += 1;
        }

        let mut minimum_jumps = i32::MAX;

        struct PathStruct {
            location: i32,
            jumps: i32,
        }

        let mut paths: LinkedList<PathStruct> = LinkedList::new();
        paths.push_front(PathStruct {
            location: -1,
            jumps: 0,
        });

        while !paths.is_empty() {
            let jumps = paths.front().unwrap().jumps;
            let location = paths.front().unwrap().location;
            paths.pop_front();

            if location >= 0 {
                locations[location as usize] = false;
            }

            if jumps + 1 >= minimum_jumps {
                continue;
            }

            for i in 2..f.len() {
                let candidate = (location + f[i]) as usize;

                if candidate < n && a[candidate] != 0 {
                    if locations[candidate] {
                        continue;
                    }

                    paths.push_back(PathStruct {
                        location: candidate as i32,
                        jumps: jumps + 1,
                    });

                    locations[candidate] = true;
                } else if candidate == n {
                    minimum_jumps = min(minimum_jumps, jumps + 1);
                    break;
                } else if candidate > n {
                    break;
                }
            }
        }

        if minimum_jumps == i32::MAX {
            minimum_jumps = -1;
        }

        minimum_jumps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_frog() {
        assert_eq!(FibFrog::solution(&vec![0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0]), 3);
    }
}
