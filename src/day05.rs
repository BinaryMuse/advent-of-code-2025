use std::ops::RangeInclusive;

use range_set::range_set;

pub(crate) fn run(input: String) -> eyre::Result<()> {
    let kitchen = parse_kitchen(&input);

    let mut total_fresh = 0;
    for ingredient in &kitchen.ingredients {
        if kitchen.is_fresh(*ingredient) {
            total_fresh += 1;
        }
    }
    println!("Part 1: {total_fresh}");

    let total_fresh = kitchen.total_fresh_ids();
    println!("Part 2: {total_fresh}");

    Ok(())
}

struct Kitchen {
    fresh_ranges: Vec<RangeInclusive<u128>>,
    ingredients: Vec<u128>,
}

impl Kitchen {
    fn new(ranges: Vec<RangeInclusive<u128>>, ingredients: Vec<u128>) -> Self {
        Self {
            fresh_ranges: ranges,
            ingredients,
        }
    }

    fn is_fresh(&self, ingredient: u128) -> bool {
        self.fresh_ranges
            .iter()
            .any(|range| range.contains(&ingredient))
    }

    fn total_fresh_ids(&self) -> u128 {
        let mut set = range_set![];
        for range in &self.fresh_ranges {
            set.insert_range(range.clone());
        }

        set.len() as u128
    }
}

fn parse_kitchen(input: &str) -> Kitchen {
    let (fresh_ranges, ingredients) = input.split_once("\n\n").unwrap();
    let fresh_ranges = fresh_ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            let start = start.parse::<u128>().unwrap();
            let end = end.parse::<u128>().unwrap();
            start..=end
        })
        .collect::<Vec<_>>();
    let ingredients = ingredients
        .lines()
        .map(|line| line.parse::<u128>().unwrap())
        .collect();
    Kitchen::new(fresh_ranges, ingredients)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
    3-5\n\
    10-14\n\
    16-20\n\
    12-18\n\
    \n\
    1\n\
    5\n\
    8\n\
    11\n\
    17\n\
    32\n\
    ";

    #[test]
    fn test_parse_kitchen() {
        let kitchen = parse_kitchen(TEST_INPUT);

        assert!(kitchen.is_fresh(3));
        assert!(kitchen.is_fresh(4));
        assert!(kitchen.is_fresh(5));
        assert!(!kitchen.is_fresh(6));
    }

    #[test]
    fn test_total_fresh() {
        let kitchen = parse_kitchen(TEST_INPUT);
        assert_eq!(kitchen.total_fresh_ids(), 14);
    }
}
