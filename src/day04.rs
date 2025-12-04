use crate::util::{Coord, Grid};

pub(crate) fn run(input: String) -> eyre::Result<()> {
    let grid = parse_input(&input);
    let accessible = accessible_stacks(&grid);
    println!("Part 1: {accessible}");

    let accessible = accessible_stacks_after_removal(&grid);
    println!("Part 2: {accessible}");

    Ok(())
}

fn parse_input(input: &str) -> Grid<()> {
    let mut grid = Grid::new(input.lines().next().unwrap().len(), input.lines().count());
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '@' {
                grid.set((row, col), ());
            }
        }
    }
    grid
}

fn accessible_stacks(grid: &Grid<()>) -> usize {
    let mut accessible = 0;
    for (coord, _) in grid.iter_filled() {
        let neighbors = grid.neighbors8(coord).collect::<Vec<_>>();
        if neighbors.iter().filter(|n| grid.get(**n).is_some()).count() < 4 {
            accessible += 1;
        }
    }
    accessible
}

fn accessible_stacks_after_removal(grid: &Grid<()>) -> usize {
    let mut total_accessible = 0;
    let mut grid = grid.clone();

    let mut last: Option<Vec<Coord>> = None;
    while last.as_ref().is_none() || !last.as_ref().unwrap().is_empty() {
        if let Some(last) = last {
            for coord in last {
                grid.clear(coord);
            }
        }

        last = Some(Vec::new());
        for (coord, _) in grid.iter_filled() {
            let neighbors_count = grid
                .neighbors8(coord)
                .filter(|n| grid.get(*n).is_some())
                .count();

            if neighbors_count < 4 {
                total_accessible += 1;
                if let Some(l) = last.as_mut() { l.push(coord) }
            }
        }
    }

    total_accessible
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
    ..@@.@@@@.\n\
    @@@.@.@.@@\n\
    @@@@@.@.@@\n\
    @.@@@@..@.\n\
    @@.@@@@.@@\n\
    .@@@@@@@.@\n\
    .@.@.@.@@@\n\
    @.@@@.@@@@\n\
    .@@@@@@@@.\n\
    @.@.@@@.@.";

    #[test]
    fn test_parse_input() {
        let grid = parse_input(TEST_INPUT);
        assert_eq!(grid.width(), 10);
        assert_eq!(grid.height(), 10);
        assert_eq!(grid.get((0, 0)), None);
        assert_eq!(grid.get((0, 3)), Some(&()));
    }

    #[test]
    fn test_accessible_stacks() {
        let grid = parse_input(TEST_INPUT);
        assert_eq!(accessible_stacks(&grid), 13);
    }

    #[test]
    fn test_accessible_stacks_after_removal() {
        let grid = parse_input(TEST_INPUT);
        assert_eq!(accessible_stacks_after_removal(&grid), 43);
    }
}
