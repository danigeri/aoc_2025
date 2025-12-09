pub fn aoc_1(input: &str) -> i32 {
    0
}

pub fn aoc_2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod aoc1_tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_aoc_1() {
        let input = fs::read_to_string("AOCx_input.txt").unwrap();

        assert_eq!(aoc_1(&input), 0);
    }
}

#[cfg(test)]
mod aoc2_tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_aoc_2() {
        let input = fs::read_to_string("AOCx_input.txt").unwrap();

        assert_eq!(aoc_2(&input), 0);
    }
}