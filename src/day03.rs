pub(crate) fn run(input: String) -> eyre::Result<()> {
    let bank = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let total_joltage = bank
        .iter()
        .map(|line| largest_joltage_pt1(line))
        .sum::<u32>();
    println!("Part 1: {}", total_joltage);

    let total_joltage = bank
        .iter()
        .map(|line| largest_joltage_pt2(line, 12))
        .sum::<u128>();
    println!("Part 2: {}", total_joltage);

    Ok(())
}

fn largest_joltage_pt1(bank: &[u32]) -> u32 {
    let mut pairs = bank.iter().enumerate().collect::<Vec<_>>();
    pairs.sort_by(|a, b| b.1.cmp(a.1));
    let first = pairs
        .iter().find(|(i, _)| *i != bank.len() - 1)
        .unwrap();

    let second = pairs.iter().find(|(i, _)| i > &first.0).unwrap();

    first.1 * 10 + second.1
}

fn largest_joltage_pt2(bank: &[u32], length: usize) -> u128 {
    let mut pairs = bank.iter().cloned().enumerate().collect::<Vec<_>>();
    pairs.sort_by(|a, b| b.1.cmp(&a.1));
    let digits = largest_joltage_pt2_helper(pairs, length, Vec::new());
    let str = digits
        .iter()
        .fold(String::new(), |acc, x| format!("{acc}{x}"));
    str.parse::<u128>().unwrap()
}

fn largest_joltage_pt2_helper(
    pairs: Vec<(usize, u32)>,
    length: usize,
    mut acc: Vec<u32>,
) -> Vec<u32> {
    if length == 0 {
        return acc;
    }

    let max_index = pairs.iter().map(|(i, _)| *i).max().unwrap();
    let first = *pairs
        .iter().find(|(i, _)| *i <= max_index - length + 1)
        .unwrap();

    acc.push(first.1);

    let remaining_pairs = pairs
        .into_iter()
        .filter(|(i, _)| i > &first.0)
        .collect::<Vec<_>>();

    largest_joltage_pt2_helper(remaining_pairs, length - 1, acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
        987654321111111\n\
        811111111111119\n\
        234234234234278\n\
        818181911112111";

    fn parse_input_line(line: &str) -> Vec<u32> {
        line.chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>()
    }

    #[test]
    fn test_parse_bank() {
        let input = "1234567890";
        let bank = input
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(bank, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
    }

    #[test]
    fn test_largest_joltage_pt1() {
        let bank = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        assert_eq!(largest_joltage_pt1(&bank), 90);

        let bank = vec![1, 2, 3, 4, 5, 6, 7, 8, 0, 9];
        assert_eq!(largest_joltage_pt1(&bank), 89);
    }

    #[test]
    fn test_largest_joltage_pt2() {
        let bank = parse_input_line("987654321111111");
        assert_eq!(largest_joltage_pt2(&bank, 12), 987654321111);

        let bank = parse_input_line("811111111111119");
        assert_eq!(largest_joltage_pt2(&bank, 12), 811111111119);

        let bank = parse_input_line("234234234234278");
        assert_eq!(largest_joltage_pt2(&bank, 12), 434234234278);

        let bank = parse_input_line("818181911112111");
        assert_eq!(largest_joltage_pt2(&bank, 12), 888911112111);
    }

    #[test]
    fn test_total_joltage_pt1() {
        let bank = TEST_INPUT
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let total_joltage = bank
            .iter()
            .map(|line| largest_joltage_pt1(line))
            .sum::<u32>();
        assert_eq!(total_joltage, 357);
    }
}
