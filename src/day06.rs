use std::str::FromStr;

pub(crate) fn run(input: String) -> eyre::Result<()> {
    let worksheet: Worksheet = input.parse().unwrap();
    let answers = worksheet.answers();
    let sum = answers.iter().sum::<i128>();
    println!("Part 1: {}", sum);

    let worksheet: Worksheet = transform_worksheet(&input).unwrap();
    let answers = worksheet.answers();
    let sum = answers.iter().sum::<i128>();
    println!("Part 2: {}", sum);

    Ok(())
}

struct Worksheet {
    problems: Vec<Problem>,
}

impl Worksheet {
    fn new() -> Self {
        Self {
            problems: Vec::new(),
        }
    }

    fn add_problem(&mut self, problem: Problem) -> &mut Self {
        self.problems.push(problem);
        self
    }

    fn answers(&self) -> Vec<i128> {
        self.problems.iter().map(|p| p.solve()).collect()
    }
}

impl FromStr for Worksheet {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let first_line = lines.next().ok_or(eyre::eyre!("No lines"))?;
        let mut problems: Vec<Problem> = first_line
            .split_whitespace()
            .map(|s| {
                let operand = s.parse::<i128>().unwrap();
                let mut problem = Problem::new();
                problem.add_operand(operand);
                problem
            })
            .collect();

        for line in lines {
            let tokens = line.split_whitespace().collect::<Vec<_>>();
            if tokens.len() != problems.len() {
                return Err(eyre::eyre!(
                    "Invalid token count: expected {} tokens, got {}",
                    problems.len(),
                    tokens.len()
                ));
            }

            if tokens[0].parse::<i128>().is_ok() {
                for (i, token) in tokens.iter().enumerate() {
                    let operand = token.parse::<i128>()?;
                    problems[i].add_operand(operand);
                }
            } else {
                for (i, token) in tokens.iter().enumerate() {
                    let operator = token.parse::<Operator>()?;
                    problems[i].set_operator(operator);
                }
            }
        }

        Ok(Worksheet { problems })
    }
}

#[derive(Debug)]
struct Problem {
    operands: Vec<i128>,
    operator: Option<Operator>,
}

impl Problem {
    fn new() -> Self {
        Self {
            operands: Vec::new(),
            operator: None,
        }
    }

    fn add_operand(&mut self, operand: i128) -> &mut Self {
        self.operands.push(operand);
        self
    }

    fn set_operator(&mut self, operator: Operator) -> &mut Self {
        self.operator = Some(operator);
        self
    }

    fn solve(&self) -> i128 {
        match self.operator {
            Some(Operator::Add) => self.operands.iter().sum(),
            Some(Operator::Multiply) => self.operands.iter().product(),
            None => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
}

impl FromStr for Operator {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Add),
            "*" => Ok(Operator::Multiply),
            _ => Err(eyre::eyre!("Invalid operator: {}", s)),
        }
    }
}

fn transform_worksheet(input: &str) -> Result<Worksheet, eyre::Error> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return Err(eyre::eyre!("No lines"));
    }

    let char_grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();

    let operator_row = &char_grid[char_grid.len() - 1];
    let data_rows = &char_grid[..char_grid.len() - 1];

    // Find operators and their positions
    let operators: Vec<(usize, Operator)> = operator_row
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| match c {
            '+' => Some((i, Operator::Add)),
            '*' => Some((i, Operator::Multiply)),
            _ => None,
        })
        .collect();

    let max_width = data_rows.iter().map(|r| r.len()).max().unwrap_or(0);

    // Build boundaries from operator positions and the end of the widest line
    let mut boundaries: Vec<usize> = operators.iter().map(|(pos, _)| *pos).collect();
    boundaries.push(max_width);

    let mut worksheet = Worksheet::new();

    for (i, (op_pos, operator)) in operators.iter().enumerate().rev() {
        let col_start = *op_pos;
        let col_end = boundaries[i + 1];

        let mut problem = Problem::new();

        // For each column in range (right-to-left), collect digits top-to-bottom
        for col in (col_start..col_end).rev() {
            let mut digits = String::with_capacity(data_rows.len());
            for row in data_rows {
                if col < row.len() {
                    let c = row[col];
                    if c.is_ascii_digit() {
                        digits.push(c);
                    }
                }
            }
            if !digits.is_empty() {
                problem.add_operand(digits.parse::<i128>()?);
            }
        }

        problem.set_operator(*operator);
        worksheet.add_problem(problem);
    }

    Ok(worksheet)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> String {
        vec![
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +  ",
        ]
        .join("\n")
    }

    #[test]
    fn test_demo_worksheet() {
        let worksheet: Worksheet = test_input().parse().unwrap();
        let answers = worksheet.answers();
        assert_eq!(answers, vec![33210, 490, 4243455, 401]);

        let sum = answers.iter().sum::<i128>();
        assert_eq!(sum, 4277556);
    }

    #[test]
    fn test_part2_worksheet() {
        let worksheet: Worksheet = transform_worksheet(&test_input()).unwrap();
        let answers = worksheet.answers();
        assert_eq!(answers, vec![1058, 3253600, 625, 8544]);

        let sum = answers.iter().sum::<i128>();
        assert_eq!(sum, 3263827);
    }

    #[test]
    fn test_problem() {
        let mut problem = Problem::new();
        problem.add_operand(1);
        problem.add_operand(2);
        problem.set_operator(Operator::Add);
        assert_eq!(problem.solve(), 3);

        let mut problem = Problem::new();
        problem.add_operand(1);
        problem.add_operand(2);
        problem.set_operator(Operator::Multiply);
        assert_eq!(problem.solve(), 2);
    }
}
