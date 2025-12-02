pub(crate) fn run(input: String) -> eyre::Result<()> {
    let instructions = parse_input(input);
    let mut safe = Safe::new(50);
    let mut zero_counts = 0;
    for instruction in &instructions {
        safe.rotate(instruction.0, instruction.1);
        if safe.position == 0 {
            zero_counts += 1;
        }
    }
    println!("Part 1: {zero_counts}");

    let mut safe = Safe::new(50);
    let mut zero_counts = 0;
    for instruction in &instructions {
        zero_counts += safe.rotate(instruction.0, instruction.1);
    }
    println!("Part 2: {zero_counts}");

    Ok(())
}

fn parse_input(input: String) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let (direction, clicks) = line.split_at(1);
            let direction = match direction {
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => unreachable!(),
            };
            Instruction(direction, clicks.parse().unwrap())
        })
        .collect()
}

struct Safe {
    position: isize,
}

impl Safe {
    fn new(position: isize) -> Self {
        Self { position }
    }

    fn rotate(&mut self, direction: Direction, clicks: usize) -> usize {
        let mut times_hit_zero = 0;

        for _ in 0..clicks {
            match direction {
                Direction::Left => self.position -= 1,
                Direction::Right => self.position += 1,
            }

            if self.position < 0 {
                self.position += 100;
            } else if self.position >= 100 {
                self.position -= 100;
            }

            if self.position == 0 {
                times_hit_zero += 1;
            }
        }

        times_hit_zero
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Instruction(Direction, usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe() {
        let mut safe = Safe::new(50);

        let overflows = safe.rotate(Direction::Left, 10);
        assert_eq!(overflows, 0);
        assert_eq!(safe.position, 40);

        let overflows = safe.rotate(Direction::Right, 90);
        assert_eq!(overflows, 1);
        assert_eq!(safe.position, 30);

        let overflows = safe.rotate(Direction::Left, 300);
        assert_eq!(overflows, 3);
        assert_eq!(safe.position, 30);
    }

    #[test]
    fn test_parse_input() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82\n".to_string();
        let instructions = parse_input(input);
        assert_eq!(
            instructions,
            vec![
                Instruction(Direction::Left, 68),
                Instruction(Direction::Left, 30),
                Instruction(Direction::Right, 48),
                Instruction(Direction::Left, 5),
                Instruction(Direction::Right, 60),
                Instruction(Direction::Left, 55),
                Instruction(Direction::Left, 1),
                Instruction(Direction::Left, 99),
                Instruction(Direction::Right, 14),
                Instruction(Direction::Left, 82),
            ]
        );
    }

    #[test]
    fn test_part1() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82\n".to_string();
        let instructions = parse_input(input);
        let mut safe = Safe::new(50);
        let mut zero_counts = 0;
        for instruction in instructions {
            safe.rotate(instruction.0, instruction.1);
            if safe.position == 0 {
                zero_counts += 1;
            }
        }

        assert_eq!(zero_counts, 3);
    }

    #[test]
    fn test_part2() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82\n".to_string();
        let instructions = parse_input(input);
        let mut safe = Safe::new(50);
        let mut zero_counts = 0;
        for instruction in instructions {
            zero_counts += safe.rotate(instruction.0, instruction.1);
        }

        assert_eq!(zero_counts, 6);
    }
}
