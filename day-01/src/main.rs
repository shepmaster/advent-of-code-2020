use snafu::{ResultExt, Snafu};
use std::collections::BTreeSet;

#[derive(Debug, Snafu)]
enum Error {
    UnableToParseInput { source: std::num::ParseIntError },

    NoResult,
}
type Result<T, E = Error> = std::result::Result<T, E>;

fn find_pair(s: &str) -> Result<u128> {
    let nums = parse(s)?;

    for n in &nums {
        let comp = 2020 - n;
        if nums.contains(&comp) {
            return Ok(n * comp);
        }
    }

    NoResult.fail()
}

fn find_triple(s: &str) -> Result<u128> {
    let nums = parse(s)?;

    for a in &nums {
        for b in nums.range(a..) {
            let c = 2020 - a - b;
            if nums.contains(&c) {
                return Ok(a * b * c);
            }
        }
    }

    NoResult.fail()
}

fn parse(s: &str) -> Result<BTreeSet<u128>> {
    let d = s.lines().map(str::trim).map(str::parse::<u128>);
    itertools::process_results(d, |d| d.filter(|&v| v < 2020).collect()).context(UnableToParseInput)
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    println!("{}", find_pair(input)?);
    println!("{}", find_triple(input)?);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"1721
        979
        366
        299
        675
        1456"#;

    #[test]
    fn test_pair() -> Result<()> {
        assert_eq!(find_pair(TEST_INPUT)?, 514579);
        Ok(())
    }

    #[test]
    fn test_triple() -> Result<()> {
        assert_eq!(find_triple(TEST_INPUT)?, 241861950);
        Ok(())
    }
}
