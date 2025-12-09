use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;
use std::{fmt, vec};

#[derive(Debug, Clone)]
pub struct Position {
    x: i32,
    y: i32,
    z: i32,
}

pub struct Node {
    id: usize,
    position: Position,
    connections: HashSet<usize>,
}

impl Node {
    pub fn new(i: usize, pos: Position) -> Self {
        Self {
            id: i,
            position: pos,
            connections: HashSet::new(),
        }
    }

    pub fn assign_id(&mut self, id: usize) {
        self.id = id;
        self.connections.insert(id);
    }
}

impl FromStr for Node {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s
            .split(',')
            .map(|f| f.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        Ok(Node::new(
            0,
            Position {
                x: x[0],
                y: x[1],
                z: x[2],
            },
        ))
    }
}

impl Position {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn distance_to(&self, other: &Position) -> i32 {
        ((other.x as i64 - self.x as i64).pow(2)
            + (other.y as i64 - self.y as i64).pow(2)
            + (other.z as i64 - self.z as i64).pow(2))
        .isqrt() as i32
    }
}

impl FromStr for Position {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s
            .split(',')
            .map(|f| f.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        Ok(Position::new(x[0], x[1], x[2]))
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

pub fn aoc2(input: &str) -> i64 {
    let nodes: Vec<Node> = parse_input_into_nodes(input);
    let mut _distance_list = create_sorted_distance_list(&nodes);

    let mut clusters: Vec<HashSet<usize>> = vec![];
    let mut last_connected = (0, 0);

    for (i, j, _) in _distance_list.iter() {
        let mut found_clusters = vec![];
        let mut exit_condition = false;
        for y in 0..clusters.len() {
            if clusters[y].contains(i) && clusters[y].contains(j) {
                exit_condition = true;
                break;
            } else if clusters[y].contains(i) {
                found_clusters.push(y);
            } else if clusters[y].contains(j) {
                found_clusters.push(y);
            }
        }
        if exit_condition {
            continue;
        }

        last_connected = (*i, *j);
        println!("Connecting nodes: {} and {}", i, j);

        if found_clusters.len() == 0 {
            println!("Creating new cluster for.");
            let mut new_cluster = HashSet::new();
            new_cluster.insert(*i);
            new_cluster.insert(*j);
            clusters.push(new_cluster);
        } else if found_clusters.len() == 1 {
            println!("Adding singular node to existing cluster.");
            let idx = found_clusters[0];
            clusters[idx].insert(*i);
            clusters[idx].insert(*j);
        } else if found_clusters.len() == 2 {
            println!("Merging two existing clusters.");
            let idx1 = found_clusters[0];
            let idx2 = found_clusters[1];

            let tmp = clusters[idx2].clone();
            clusters[idx1].extend(tmp);
            clusters.remove(idx2);
        }
    }

    // clusters.sort_by(|a, b| b.len().cmp(&a.len()));

    for i in clusters.iter() {
        println!("Cluster of size {}: {:?}", i.len(), i);
    }

    let mut a = 0;
    let mut b = 0;

    for i in nodes.iter() {
        if i.id == last_connected.0 {
            a = i.position.x;
        }

        if i.id == last_connected.1 {
            b = i.position.x;
        }
    }
    println!("{}-{}", a, b);
    a as i64 * b as i64
}

pub fn aoc1(input: &str, number_of_runs: usize) -> usize {
    let nodes: Vec<Node> = parse_input_into_nodes(input);
    let mut _distance_list = create_sorted_distance_list(&nodes);

    let mut clusters: Vec<HashSet<usize>> = vec![];

    for x in 0..number_of_runs {
        let (i, j, _) = _distance_list[x];

        let mut found_clusters = vec![];
        let mut exit_condition = false;
        for y in 0..clusters.len() {
            if clusters[y].contains(&i) && clusters[y].contains(&j) {
                exit_condition = true;
                break;
            } else if clusters[y].contains(&i) {
                found_clusters.push(y);
            } else if clusters[y].contains(&j) {
                found_clusters.push(y);
            }
        }
        if exit_condition {
            continue;
        }

        if found_clusters.is_empty() {
            println!("Creating new cluster for.");
            let mut new_cluster = HashSet::new();
            new_cluster.insert(i);
            new_cluster.insert(j);
            clusters.push(new_cluster);
        } else if found_clusters.len() == 1 {
            println!("Adding singular node to existing cluster.");
            let idx = found_clusters[0];
            clusters[idx].insert(i);
            clusters[idx].insert(j);
        } else if found_clusters.len() == 2 {
            println!("Merging two existing clusters.");
            let idx1 = found_clusters[0];
            let idx2 = found_clusters[1];

            let tmp = clusters[idx2].clone();
            clusters[idx1].extend(tmp);
            clusters.remove(idx2);
        }
    }

    clusters.sort_by(|a, b| b.len().cmp(&a.len()));

    for i in clusters.iter() {
        println!("Cluster of size {}: {:?}", i.len(), i);
    }

    clusters[0].len() * clusters[1].len() * clusters[2].len()
}

// pub fn get_distance(p: (i32, i32, i32), q: (i32, i32, i32)) -> i32 {
//     ((q.0 - p.0).pow(2) + (q.1 - p.1).pow(2) + (q.2 - p.2).pow(2)).isqrt() as i32
// }

fn parse_input_into_nodes(input: &str) -> Vec<Node> {
    let mut result = input
        .lines()
        .map(|s| s.parse::<Node>().unwrap())
        .collect::<Vec<_>>();

    for (id, node) in result.iter_mut().enumerate() {
        node.assign_id(id);
    }

    result
}

fn create_sorted_distance_list(nodes: &[Node]) -> Vec<(usize, usize, i32)> {
    let mut list: Vec<(usize, usize, i32)> = vec![];

    for i in 0..nodes.len() {
        for j in i..nodes.len() {
            if i == j {
                continue;
            }

            let d = nodes[i].position.distance_to(&nodes[j].position);
            list.push((i, j, d));
        }
    }

    list.sort_by(|a, b| a.2.cmp(&b.2));

    list
}
#[cfg(test)]
mod aoc2_real {
    use super::*;
    use std::fs;

    #[test]
    fn ex1() {
        let input = fs::read_to_string("inputs\\8.txt").unwrap();

        assert_eq!(aoc2(&input), 2573952864);
    }
}
#[cfg(test)]
mod aoc2_examples {
    use super::*;

    #[test]
    fn ex1() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

        assert_eq!(aoc2(&input), 25272);
    }
}

#[cfg(test)]
mod aoc1_real {
    use super::*;
    use std::fs;

    #[test]
    fn ex1() {
        let input = fs::read_to_string("inputs\\8.txt").unwrap();

        assert_eq!(aoc1(&input, 1000), 112230);
    }
}
#[cfg(test)]
mod aoc1_examples {
    use super::*;

    #[test]
    fn ex1() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

        assert_eq!(aoc1(&input, 10), 40);
    }
}
