pub fn solution(x: i32, y: i32, d: i32) -> i32 {
    let num_of_jumps = (( y as f32)-( x as f32) )/ (d as f32);
    num_of_jumps.ceil() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frog_jmp() {
        assert_eq!(solution(10, 85, 30), 3);
    }
}