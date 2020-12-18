use snafu::{OptionExt, ResultExt, Snafu};
use std::{convert::TryFrom, ops::RangeInclusive};

#[derive(Debug, Snafu)]
enum Error {
    WrongNumberOfElements { actual: usize },

    MinWasInvalid { source: std::num::ParseIntError },

    MaxWasInvalid { source: std::num::ParseIntError },

    CharacterMissing,
}
type Result<T, E = Error> = std::result::Result<T, E>;

struct Line<'a> {
    r: RangeInclusive<usize>,
    c: char,
    password: &'a str,
}

impl Line<'_> {
    fn is_sled_valid(&self) -> bool {
        let occurrences = self.password.matches(self.c).count();
        self.r.contains(&occurrences)
    }

    fn is_toboggan_valid(&self) -> bool {
        let location_1 = *self.r.start();
        let location_2 = *self.r.end();

        let a = self.password.chars().nth(location_1) == Some(self.c);
        let b = self.password.chars().nth(location_2) == Some(self.c);

        a ^ b
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
        let c = c.chars().next().context(CharacterMissing)?;
        let r = min..=max;
        Ok(Self { r, c, password })
    }
}

fn process_sled_rules(s: &str) -> Result<usize> {
    itertools::process_results(s.lines().map(str::trim).map(Line::try_from), |l| {
        l.filter(Line::is_sled_valid).count()
    })
}

fn process_toboggan_rules(s: &str) -> Result<usize> {
    itertools::process_results(s.lines().map(str::trim).map(Line::try_from), |l| {
        l.filter(Line::is_toboggan_valid).count()
    })
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    println!("{}", process_sled_rules(input)?);
    println!("{}", process_toboggan_rules(input)?);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"1-3 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc"#;

    #[test]
    fn demo_sled() -> Result<()> {
        assert_eq!(process_sled_rules(TEST_INPUT)?, 2);
        Ok(())
    }

    #[test]
    fn demo_toboggan() -> Result<()> {
        assert_eq!(process_toboggan_rules(TEST_INPUT)?, 1);
        Ok(())
    }
}
