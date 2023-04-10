struct PassingCars;

impl PassingCars {
    fn solution(&self, a: &Vec<u32>) -> i32 {
        let n = a.len();

        let cumsum: Vec<u32> = a
            .iter()
            .scan(0, |acc, &x| {
                *acc += x;
                Some(*acc)
            })
            .collect();

        let mut number_of_passing_car = 0_u32;
        for (i, e) in a.into_iter().enumerate() {
            if *e == 0 {
                number_of_passing_car += cumsum[n - 1] - cumsum[i];
                if number_of_passing_car > 1000000000 {
                    return -1;
                }
            }
        }

        number_of_passing_car as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passing_cars() {
        let passing_cars = PassingCars;
        assert_eq!(passing_cars.solution(&vec![0, 1, 0, 1, 1]), 5);
    }
}
