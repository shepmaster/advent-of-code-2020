use snafu::{ResultExt, Snafu};
use std::collections::BTreeSet;

#[derive(Debug, Snafu)]
enum Error {
    UnableToParseInput { source: std::num::ParseIntError },

    NoResult,
}
type Result<T, E = Error> = std::result::Result<T, E>;

fn do_it(s: &str) -> Result<u128> {
    let d = s.lines().map(str::trim).map(str::parse::<u128>);
    let nums: BTreeSet<_> = itertools::process_results(d, |d| d.filter(|&v| v < 2020).collect())
        .context(UnableToParseInput)?;

    for n in &nums {
        let comp = 2020 - n;
        if nums.contains(&comp) {
            return Ok(n * comp);
        }
    }

    NoResult.fail()
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    println!("{}", do_it(input)?);
    Ok(())
}

#[test]
fn demo() -> Result<()> {
    let input = r#"1721
        979
        366
        299
        675
        1456"#;

    assert_eq!(do_it(input)?, 514579);
    Ok(())
}
