use std::collections::HashMap;

use crate::util::{Coord, Direction4, Grid};

pub(crate) fn run(input: String) -> eyre::Result<()> {
    let mut manifold = Manifold::from_input(&input);
    manifold.run();
    manifold.print();

    let inactive_beams = manifold.inactive_beams();
    println!("Part 1: {}", inactive_beams.len());

    println!(
        "Part 2: {}",
        manifold.count_quantum_manifolds(manifold.start())
    );

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Component {
    Empty,
    Entrance,
    Splitter,
    Beam,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    start: Coord,
    current: Coord,
    end: Option<Coord>,
    split: bool,
    out_of_bounds: bool,
}

impl Beam {
    fn new(start: Coord) -> Self {
        Self {
            start,
            current: start,
            end: None,
            split: false,
            out_of_bounds: false,
        }
    }

    fn is_active(&self) -> bool {
        self.end.is_none()
    }
}

struct Manifold {
    grid: Grid<Component>,
    beams: Vec<Beam>,
    manifold_cache: HashMap<Coord, u64>,
}

impl Manifold {
    fn from_input(input: &str) -> Self {
        let data: Vec<Vec<Component>> = input
            .trim()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Component::Empty,
                        'S' => Component::Entrance,
                        '^' => Component::Splitter,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();

        Self {
            grid: Grid::from_vec(data),
            beams: Vec::new(),
            manifold_cache: HashMap::new(),
        }
    }

    fn start(&self) -> Coord {
        self.grid
            .iter_filled()
            .find(|(_, c)| **c == Component::Entrance)
            .unwrap()
            .0
    }

    fn run(&mut self) {
        self.insert_beam(Beam::new(self.start()));

        loop {
            let mut beams_processed = 0;
            let mut new_beams = Vec::new();

            for beam in self.beams.iter_mut() {
                if !beam.is_active() {
                    continue;
                }

                let next_coord = beam.current.step(Direction4::South, 1);
                if !self.grid.in_bounds(next_coord) {
                    beam.out_of_bounds = true;
                    continue;
                }

                if let Some(component) = self.grid.get(next_coord) {
                    match component {
                        Component::Splitter => {
                            beam.current = next_coord;
                            beam.end = Some(next_coord);
                            beam.split = true;
                            let west = next_coord.step(Direction4::West, 1);
                            let east = next_coord.step(Direction4::East, 1);
                            if self.grid.in_bounds(west) {
                                new_beams.push(Beam::new(west));
                                self.grid.set(west, Component::Beam);
                            }
                            if self.grid.in_bounds(east) {
                                new_beams.push(Beam::new(east));
                                self.grid.set(east, Component::Beam);
                            }
                        }
                        _ => {
                            self.grid.set(next_coord, Component::Beam);
                            beam.current = next_coord;
                        }
                    }
                }

                beams_processed += 1;
            }

            for beam in new_beams {
                self.insert_beam(beam);
            }

            if beams_processed == 0 {
                break;
            }
        }
    }

    fn count_quantum_manifolds(&mut self, pos: Coord) -> u64 {
        if let Some(count) = self.manifold_cache.get(&pos) {
            return *count;
        }

        let next = pos.step(Direction4::South, 1);

        if !self.grid.in_bounds(next) {
            return 1;
        }

        let count = match self.grid.get(next) {
            Some(Component::Splitter) => {
                let left = next.step(Direction4::West, 1);
                let right = next.step(Direction4::East, 1);
                self.count_quantum_manifolds(left) + self.count_quantum_manifolds(right)
            }
            _ => self.count_quantum_manifolds(next),
        };

        self.manifold_cache.insert(pos, count);

        count
    }

    fn insert_beam(&mut self, beam: Beam) {
        if self.beams.iter().any(|b| b.current == beam.current) {
            return;
        }
        self.beams.push(beam);
    }

    fn inactive_beams(&self) -> Vec<&Beam> {
        self.beams.iter().filter(|b| !b.is_active()).collect()
    }

    fn print(&self) {
        self.grid.print(|cell| match cell {
            Some(Component::Empty) => ".",
            Some(Component::Entrance) => "S",
            Some(Component::Splitter) => "^",
            Some(Component::Beam) => "|",
            None => " ",
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
.......S.......
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
...............
    "};

    #[test]
    fn test_part1() {
        let mut manifold = Manifold::from_input(TEST_INPUT);
        manifold.run();
        manifold.print();
        assert_eq!(manifold.inactive_beams().len(), 21);
    }

    #[test]
    fn test_part2() {
        let mut manifold = Manifold::from_input(TEST_INPUT);
        let total = manifold.count_quantum_manifolds(manifold.start());
        assert_eq!(total, 40);
    }
}
