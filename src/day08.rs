use std::collections::{BTreeMap, VecDeque};

use itertools::Itertools;

pub(crate) fn run(input: String) -> eyre::Result<()> {
    let mut playground = parse_input(&input);
    playground.make_connections(1000);
    let result: usize = playground
        .disjoint_set
        .get_sets()
        .into_iter()
        .take(3)
        .map(|set| set.len())
        .product();
    println!("Part 1: {}", result);

    let last_pair = playground.connect_until_single_set();
    let result = last_pair.unwrap().0.x * last_pair.unwrap().1.x;
    println!("Part 2: {}", result);

    Ok(())
}

#[derive(Debug, Clone)]
struct DisjointSet {
    parent: Vec<usize>,
}

impl DisjointSet {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
        }
    }

    fn find(&mut self, item: usize) -> usize {
        if self.parent[item] != item {
            // compress the path as we walk it
            self.parent[item] = self.find(self.parent[item]);
        }

        self.parent[item]
    }

    fn unite(&mut self, item1: usize, item2: usize) {
        let root1 = self.find(item1);
        let root2 = self.find(item2);

        if root1 != root2 {
            self.parent[root2] = root1;
        }
    }

    fn get_sets(&mut self) -> Vec<Vec<usize>> {
        let mut sets = BTreeMap::new();
        for i in 0..self.parent.len() {
            let root = self.find(i);
            sets.entry(root).or_insert(vec![]).push(i);
        }
        sets.values()
            .cloned()
            .sorted_by(|a, b| b.len().cmp(&a.len()))
            .collect::<Vec<_>>()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3 {
    fn distance(&self, other: &Self) -> f64 {
        let base = ((self.x - other.x).pow(2)
            + (self.y - other.y).pow(2)
            + (self.z - other.z).pow(2)) as f64;

        base.sqrt()
    }
}

#[derive(Debug, Clone)]
struct Playground {
    junction_boxes: Vec<Vec3>,
    disjoint_set: DisjointSet,
    distances: VecDeque<(f64, (usize, usize))>,
}

impl Playground {
    fn new(junction_boxes: Vec<Vec3>) -> Self {
        let len = junction_boxes.len();
        let distances = junction_boxes
            .iter()
            .enumerate()
            .combinations(2)
            .map(|pair| {
                let (i1, v1) = pair[0];
                let (i2, v2) = pair[1];
                (v1.distance(v2), (i1, i2))
            })
            .sorted_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
            .collect();

        Self {
            junction_boxes,
            disjoint_set: DisjointSet::new(len),
            distances,
        }
    }

    fn make_connections(&mut self, count: usize) {
        for (_, (i1, i2)) in self.distances.iter().take(count) {
            self.disjoint_set.unite(*i1, *i2);
        }
    }

    fn connect_until_single_set(&mut self) -> Option<(Vec3, Vec3)> {
        let mut last_pair: Option<(usize, usize)> = None;
        while self.disjoint_set.get_sets().len() > 1 {
            let (_, (i1, i2)) = self.distances.pop_front().unwrap();
            self.disjoint_set.unite(i1, i2);
            last_pair = Some((i1, i2));
        }

        last_pair.map(|(i1, i2)| (self.junction_boxes[i1], self.junction_boxes[i2]))
    }
}

fn parse_input(input: &str) -> Playground {
    let junction_boxes = input
        .trim()
        .lines()
        .map(|line| {
            let coords = line
                .split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            Vec3 {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            }
        })
        .collect();
    Playground::new(junction_boxes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        162,817,812
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
        425,690,689
    "};

    #[test]
    fn test_distances() {
        let playground = parse_input(TEST_INPUT);
        let first_distance = playground.distances[0];
        let (i1, i2) = first_distance.1;
        let v1 = playground.junction_boxes[i1];
        let v2 = playground.junction_boxes[i2];

        assert_eq!(
            v1,
            Vec3 {
                x: 162,
                y: 817,
                z: 812
            }
        );
        assert_eq!(
            v2,
            Vec3 {
                x: 425,
                y: 690,
                z: 689
            }
        );
    }

    #[test]
    fn test_part1() {
        let mut playground = parse_input(TEST_INPUT);
        playground.make_connections(10);
        let sets = playground.disjoint_set.get_sets();
        assert_eq!(sets.len(), 11);
        let result: usize = sets.into_iter().take(3).map(|set| set.len()).product();
        assert_eq!(result, 40);
    }

    #[test]
    fn test_part2() {
        let mut playground = parse_input(TEST_INPUT);
        let last_pair = playground.connect_until_single_set();
        assert_eq!(
            last_pair,
            Some((
                Vec3 {
                    x: 216,
                    y: 146,
                    z: 977
                },
                Vec3 {
                    x: 117,
                    y: 168,
                    z: 530
                }
            ))
        );

        let result = last_pair.unwrap().0.x * last_pair.unwrap().1.x;
        assert_eq!(result, 25272);
    }
}
