use std::str::FromStr;

pub(crate) fn run(input: String) -> eyre::Result<()> {
    let product_ranges = input
        .split(',')
        .map(|s| s.parse::<ProductRange>().unwrap())
        .collect::<Vec<_>>();

    let invalid_ids = product_ranges
        .iter()
        .flat_map(|pr| pr.invalid_ids_type1())
        .collect::<Vec<_>>();

    let sum = invalid_ids.iter().sum::<u128>();
    println!("Part 1: {}", sum);

    let invalid_ids = product_ranges
        .iter()
        .flat_map(|pr| pr.invalid_ids_type2())
        .collect::<Vec<_>>();

    let sum = invalid_ids.iter().sum::<u128>();
    println!("Part 2: {}", sum);

    Ok(())
}

#[derive(Debug)]
struct ProductRange {
    start: u128,
    end: u128,
}

impl ProductRange {
    fn new(start: u128, end: u128) -> Self {
        Self { start, end }
    }

    fn invalid_ids_type1(&self) -> Vec<u128> {
        let mut invalid = Vec::with_capacity(2);

        for id in self.start..=self.end {
            if is_repeated_digits_type1(&id.to_string()) {
                invalid.push(id);
            }
        }

        invalid
    }

    fn invalid_ids_type2(&self) -> Vec<u128> {
        // the ID is invalid if it is made ONLY of some sequence of digits repeated at least twice
        let mut invalid = Vec::with_capacity(2);

        for id in self.start..=self.end {
            if is_repeated_digits_type2(&id.to_string()) {
                invalid.push(id);
            }
        }

        invalid
    }
}

impl FromStr for ProductRange {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').ok_or(eyre::eyre!("Invalid range"))?;
        Ok(Self::new(start.parse()?, end.parse()?))
    }
}

fn is_repeated_digits_type1(s: &str) -> bool {
    if !s.len().is_multiple_of(2) {
        return false;
    }

    let half = s.len() / 2;
    let first_half = &s[..half];
    let second_half = &s[half..];
    first_half == second_half
}

fn is_repeated_digits_type2(s: &str) -> bool {
    let possible_substr_lengths = (1..=s.len() / 2).collect::<Vec<_>>();
    for substr_length in possible_substr_lengths {
        let substr = &s[..substr_length];
        let mut matches = String::new();
        while matches.len() < s.len() {
            matches.push_str(substr);
        }
        if matches == s {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
        11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
        1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
        824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_invalid_ids_type1() {
        assert_eq!(is_repeated_digits_type1("1111"), true);
        assert_eq!(is_repeated_digits_type1("1212"), true);
        assert_eq!(is_repeated_digits_type1("123123"), true);
        assert_eq!(is_repeated_digits_type1("1231234"), false);
    }

    #[test]
    fn test_invalid_ids_type2() {
        assert_eq!(is_repeated_digits_type2("1111"), true);
        assert_eq!(is_repeated_digits_type2("1212"), true);
        assert_eq!(is_repeated_digits_type2("123123"), true);
        assert_eq!(is_repeated_digits_type2("1231234"), false);

        assert_eq!(is_repeated_digits_type2("11111"), true);
        assert_eq!(is_repeated_digits_type2("123123123"), true);
    }

    #[test]
    fn test_find_invalid_ids_type1() {
        println!("TEST_INPUT: {}", TEST_INPUT);

        let product_ranges = TEST_INPUT
            .split(',')
            .map(|s| s.parse::<ProductRange>().unwrap())
            .collect::<Vec<_>>();

        let invalid_ids = product_ranges
            .iter()
            .flat_map(|pr| pr.invalid_ids_type1())
            .collect::<Vec<_>>();

        assert_eq!(
            invalid_ids,
            vec![11, 22, 99, 1010, 1188511885, 222222, 446446, 38593859]
        );
    }

    #[test]
    fn test_find_invalid_ids_type2() {
        let product_ranges = TEST_INPUT
            .split(',')
            .map(|s| s.parse::<ProductRange>().unwrap())
            .collect::<Vec<_>>();

        let invalid_ids = product_ranges
            .iter()
            .flat_map(|pr| pr.invalid_ids_type2())
            .collect::<Vec<_>>();

        assert_eq!(
            invalid_ids,
            vec![
                11, 22, 99, 111, 999, 1010, 1188511885, 222222, 446446, 38593859, 565656,
                824824824, 2121212121
            ]
        );
    }
}
