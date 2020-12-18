use snafu::{ResultExt, Snafu};
use std::{convert::TryFrom, ops::RangeInclusive};

#[derive(Debug, Snafu)]
enum Error {
    WrongNumberOfElements { actual: usize },

    MinWasInvalid { source: std::num::ParseIntError },

    MaxWasInvalid { source: std::num::ParseIntError },
}
type Result<T, E = Error> = std::result::Result<T, E>;

struct Line<'a> {
    r: RangeInclusive<usize>,
    c: &'a str,
    password: &'a str,
}

impl Line<'_> {
    fn is_valid(&self) -> bool {
        let occurrences = self.password.matches(self.c).count();
        self.r.contains(&occurrences)
    }
}

impl<'a> TryFrom<&'a str> for Line<'a> {
    type Error = Error;

    fn try_from(other: &'a str) -> Result<Self> {
        let parts: Vec<_> = other.splitn(4, &['-', ' ', ':'][..]).collect();
        let [min, max, c, password] = match <[_; 4]>::try_from(parts) {
            Ok(a) => a,
            Err(v) => WrongNumberOfElements { actual: v.len() }.fail()?,
        };
        let min = min.parse().context(MinWasInvalid)?;
        let max = max.parse().context(MaxWasInvalid)?;
        let r = min..=max;
        Ok(Self { r, c, password })
    }
}

fn process(s: &str) -> Result<usize> {
    itertools::process_results(s.lines().map(str::trim).map(Line::try_from), |l| {
        l.filter(Line::is_valid).count()
    })
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    println!("{}", process(input)?);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"1-3 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc"#;

    #[test]
    fn demo() -> Result<()> {
        assert_eq!(process(TEST_INPUT)?, 2);
        Ok(())
    }
}
