pub fn solution(num: i32) -> i32 {
    let number_of_bits = (num as f32 + 1.).log2().ceil();

    let mut found_one = false;
    let mut maximum_gap = 0;
    let mut possible_gap = 0;

    for i in 0..number_of_bits as i32 {
        if found_one && (num & (1 << i)) == 0 {
            possible_gap += 1;
        } else if num & (1 << i) != 0 {
            if found_one {
                if possible_gap > maximum_gap {
                    maximum_gap = possible_gap;
                }
                possible_gap = 0;
            }
            found_one = true;
        }
    }

    maximum_gap
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_gap() {
        assert_eq!(solution(0b001010000100), 4);
        assert_eq!(solution(0b000000000), 0);
    }
}
