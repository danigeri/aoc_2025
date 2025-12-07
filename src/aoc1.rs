pub fn aoc1_1(input: &str) -> i32 {
    let turns: Vec<i32> = input.lines().map(|f| if f.chars().nth(0) == Some('R') {f[1..].parse::<i32>().unwrap()} else {f[1..].parse::<i32>().unwrap() * -1}).collect::<Vec<_>>();
    let mut position = 50;
    let mut result = 0;

    for turn in turns {
        position += turn;

        position %= 100;

        if position == 0 {
            result += 1;
        }
    }

    result
}

pub fn aoc1_2(input: &str) -> i32 {
    let turns: Vec<i32> = input.lines().map(|f| if f.chars().nth(0) == Some('R') {f[1..].parse::<i32>().unwrap()} else {f[1..].parse::<i32>().unwrap() * -1}).collect::<Vec<_>>();
    let mut position = 50;
    let mut result = 0;

    for turn in turns {
        position += turn;

        let zero_count = position / 100;
        position %= 100;

        // if position < 0 {
        //     position += 100;x
        // }

        if position == 0 {
            result += 1;
        }

        result += zero_count;
    }

    result
}


#[cfg(test)]
mod aoc1_2 {
    use std::fs;

    use super::*;

    #[test]
    fn aoc2() {
        let input = fs::read_to_string("inputs\\1.txt").unwrap();

        assert_eq!(aoc1_2(&input), 0);
    }
}

#[cfg(test)]
mod aoc1_2_examples {
    use super::*;

    #[test]
    fn ex1() {
        let mut input = "L68\n".to_string();
        input.push_str("L30\n");
        input.push_str("R48\n");
        input.push_str("L5\n");
        input.push_str("R60\n");
        input.push_str("L55\n");
        input.push_str("L1\n");
        input.push_str("L99\n");
        input.push_str("R14\n");
        input.push_str("L82\n");

        assert_eq!(aoc1_2(&input), 6);
    }

    #[test]
    fn ex2() {
        let input = "R1000\n".to_string();

        assert_eq!(aoc1_2(&input), 10);
    }
}

#[cfg(test)]
mod aoc1_2_user_tests {
    use super::*;

    #[test]
    fn easy_1() {
        let input = "L50";

        assert_eq!(aoc1_2(&input), 1);
    }

    #[test]
    fn easy_2() {
        let input = "R50";

        assert_eq!(aoc1_2(&input), 1);
    }

    #[test]
    fn easy_3() {
        let mut input = "R50\n".to_string();
        input.push_str("L50\n");
        input.push_str("L50\n");

        assert_eq!(aoc1_2(&input), 2);
    }

    #[test]
    fn easy_4() {
        let mut input = "R50\n".to_string();
        input.push_str("L100\n");
        input.push_str("R100\n");
        input.push_str("R1000\n");
        input.push_str("L1000\n");
        input.push_str("R10000\n");
        input.push_str("L10000\n");

        assert_eq!(aoc1_2(&input), 7);
    }
}

#[cfg(test)]
mod aoc1_1_result {
    use std::fs;

    use super::*;

    #[test]
    fn aoc1() {
        let input = fs::read_to_string("inputs\\1.txt").unwrap();

        assert_eq!(aoc1_1(&input), 980);
    }
}

#[cfg(test)]
mod aoc1_1_examples {
    use super::*;

    #[test]
    fn ex1() {
        let mut input = "L68\n".to_string();
        input.push_str("L30\n");
        input.push_str("R48\n");
        input.push_str("L5\n");
        input.push_str("R60\n");
        input.push_str("L55\n");
        input.push_str("L1\n");
        input.push_str("L99\n");
        input.push_str("R14\n");
        input.push_str("L82\n");

        assert_eq!(aoc1_1(&input), 6);
    }
}

#[cfg(test)]
mod aoc1_1_user_tests {
    use super::*;

    #[test]
    fn easy_1() {
        let input = "L50";

        assert_eq!(aoc1_1(&input), 1);
    }

    #[test]
    fn easy_2() {
        let input = "R50";

        assert_eq!(aoc1_1(&input), 1);
    }

    #[test]
    fn easy_3() {
        let mut input = "R50\n".to_string();
        input.push_str("L50\n");
        input.push_str("L50\n");

        assert_eq!(aoc1_1(&input), 2);
    }

    #[test]
    fn easy_4() {
        let mut input = "R50\n".to_string();
        input.push_str("L100\n");
        input.push_str("R100\n");
        input.push_str("R1000\n");
        input.push_str("L1000\n");
        input.push_str("R10000\n");
        input.push_str("L10000\n");

        assert_eq!(aoc1_1(&input), 7);
    }
}