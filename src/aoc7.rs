pub fn aoc1_1(input: &str) -> i32 {
    let mut array: Vec<Vec<u8>> = convert_to_2d_array(input);
    let bounds = (array.len(), array[0].len());

    // Starting position is [0][70]
    let starting_position = (1, get_starting_postion(&array).1); // refactor

    start(starting_position, &mut array, bounds)
}

pub fn aoc1_2(input: &str) -> i32 {
    let mut result = 0;

    result
}

fn convert_to_2d_array(input: &str) -> Vec<Vec<u8>> {
    input
        .split_whitespace()
        .map(|f| f.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn get_starting_postion(array: &Vec<Vec<u8>>) -> (usize, usize) {
    for i in 0..array.len() {
        for j in 0..array[i].len() {
            if array[i][j] == b'S' {
                return (i, j);
            }
        }
    }

    (0, 0)
}

fn start(pos: (usize, usize), array: &mut Vec<Vec<u8>>, bounds: (usize, usize)) -> i32 {
    let mut pos = pos;
    let mut number_of_splits = 0;

    while (array[pos.0][pos.1] != b'^') {
        if pos.0 == usize::MAX || pos.0 >= bounds.0 || pos.1 == usize::MAX  || pos.1 >= bounds.1 { // refactor
            return 0;
        }
        array[pos.0][pos.1] = b'|';
        pos.1 += 1;
    }

    number_of_splits += 1;

    pos = (pos.0, pos.1 - 1);
    number_of_splits += start(pos, array, bounds);

    pos = (pos.0, pos.1 + 1);
    number_of_splits += start(pos, array, bounds);

    number_of_splits
}
#[cfg(test)]
mod aoc1_2 {
    use std::fs;

    use super::*;

    #[test]
    fn aoc2() {
        let input = fs::read_to_string("inputs/1.txt").unwrap();

        assert_eq!(aoc1_2(&input), 5961);
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

        assert_eq!(aoc1_2(&input), 223);
    }

    #[test]
    fn easy_5() {
        let input = "L51\n".to_string();

        assert_eq!(aoc1_2(&input), 1);
    }
}

#[cfg(test)]
mod aoc1_1_result {
    use std::fs;

    use super::*;

    #[test]
    fn aoc1() {
        let input = fs::read_to_string("inputs/7.txt").unwrap();

        assert_eq!(aoc1_1(&input), 980);
    }
}

#[cfg(test)]
mod aoc1_1_examples {
    use super::*;

    #[test]
    fn ex1() {
        let mut input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."; 

        assert_eq!(aoc1_1(&input), 3);
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
