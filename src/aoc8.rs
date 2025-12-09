use std::collections::HashSet;
use std::{fmt, vec};
use std::num::ParseIntError;
use std::str::FromStr;

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
        Self { id: i, position: pos, connections: HashSet::new()}
    }

    pub fn assign_id(&mut self, id: usize) {
        self.id = id;
        self.connections.insert(id);
    }
}

impl FromStr for Node {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s.split(',').map(|f| f.parse::<i32>().unwrap()).collect::<Vec<_>>();

        Ok(Node::new(
            0,
            Position { x: x[0], y: x[1], z: x[2] },
        ))
    }
}

impl Position {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn distance_to(&self, other: &Position) -> i32 {
        ((other.x - self.x).pow(2) + (other.y - self.y).pow(2) + (other.z - self.z).pow(2)).isqrt() as i32
    }
}

impl FromStr for Position {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s.split(',').map(|f| f.parse::<i32>().unwrap()).collect::<Vec<_>>();

        Ok(Position::new(
            x[0],
            x[1],
            x[2],
        ))
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

pub fn aoc1(input: &str) -> i32 {
    // let mut coordinates: Vec<Vec<Position>> = parse_input_into_coordinates(input);
    let mut nodes: Vec<Node> = parse_input_into_nodes(input);

    let mut clusters: Vec<HashSet<usize>> = vec![];

    let number_of_runs = 10;
    for x in 0..number_of_runs {
        let mut a_i = 0;
        let mut b_i = 0;


        // remove the connected nodes from nodes
        // but check the remainin ones to the clusters
        
        let mut min_dist = i32::MAX;
        for i in nodes.iter().enumerate() {
            for j in nodes.iter().enumerate() {
                let dist = i.1.position.distance_to(&j.1.position);
                if dist != 0 && dist < min_dist {
                    let mut already_connected = false;
                    for cluster in clusters.iter_mut() {
                        if cluster.contains(&i.0) && cluster.contains(&j.0) {
                            already_connected = true;
                            // break;
                        }
                    }
                    if already_connected {
                        // println!("Already connected: {} and {}", i.0, j.0);
                        continue;
                    }

                    min_dist = dist;

                    a_i = i.0;
                    b_i = j.0;
                }
            }
        }

        println!("Connecting nodes {}: {} and {}: {}.", a_i, nodes[a_i].position, b_i, nodes[b_i].position);

        let mut found_clusters = vec![];
        for i in 0..clusters.len() {
            if clusters[i].contains(&a_i) {
                found_clusters.push(i);
            }
            else if clusters[i].contains(&b_i) {
                found_clusters.push(i);
            }
        }

        if found_clusters.len() == 0 {
            println!("Creating new cluster for.");
            let mut new_cluster = HashSet::new();
            new_cluster.insert(a_i);
            new_cluster.insert(b_i);
            clusters.push(new_cluster);
        } else if found_clusters.len() == 1 {
            println!("Adding singular node to existing cluster.");
            let idx = found_clusters[0];
            clusters[idx].insert(a_i);
            clusters[idx].insert(b_i);
        } else if found_clusters.len() == 2 {
            println!("Merging two existing clusters.");
            let idx1 = found_clusters[0];
            let idx2 = found_clusters[1];

            let tmp = clusters[idx2].clone();
            clusters[idx1].extend(tmp);
            clusters.remove(idx2);
        }


        // let a = nodes[b_i].connections.clone();
        // nodes[a_i].connections.extend(a);
        // nodes[a_i].connections.insert(b_i);

        // let b = nodes[a_i].connections.clone();
        // nodes[b_i].connections.extend(b);
        // nodes[b_i].connections.insert(a_i);


        // // coordinates[a_i].append(coordinates[b_i].clone().as_mut());
        // // coordinates.remove(b_i);

        // coordinates.remove(b_i);

        // println!("{}: The closest two is {} and {}.", x, coordinates[b_i][0], coordinates[a_i][0]);
    }

    for i in clusters.iter() {
        println!("Cluster of size {}: {:?}", i.len(), i);
    }

    // for node in nodes.iter() {
    //     println!("Node {} at position {}", node.id, node.position);
    //     for connection in node.connections.iter() {
    //         print!(" {}", connection);
    //     }
    //     println!();
    // }
    0
}
pub fn get_distance(p: (i32, i32, i32), q: (i32, i32, i32)) -> i32 {
    ((q.0 - p.0).pow(2) + (q.1 - p.1).pow(2) + (q.2 - p.2).pow(2)).isqrt() as i32
}

fn parse_input_into_coordinates(input: &str) -> Vec<Vec<Position>> {
    let result =input
        .lines()
        .map(|s| vec![s.parse::<Position>().unwrap()])
        .collect::<Vec<_>>();

    result
}

fn parse_input_into_nodes(input: &str) -> Vec<Node> {
    let mut result =input
        .lines()
        .map(|s| s.parse::<Node>().unwrap())
        .collect::<Vec<_>>();

    let mut id: usize = 0;
    for node in result.iter_mut() {
        node.assign_id(id);
        id += 1;
    }

    result
}

#[cfg(test)]
mod aoc1_user_tests {
    use super::*;

    #[test]
    fn ex1() {
        let a = (162, 817, 812);
        let b = ( 57, 618, 57);

        print!("Distance: {}", get_distance(a, b));
    }

    #[test]
    fn ex2() {
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
        let a = aoc1(&input);
    }

    
}