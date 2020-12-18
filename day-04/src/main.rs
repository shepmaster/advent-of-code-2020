use snafu::{OptionExt, Snafu};
use std::{convert::TryFrom, mem};

#[derive(Debug, Snafu)]
enum Error {
    MissingKey,

    MissingValue { key: String },

    UnknownKey { key: String },
}
type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Default, PartialEq)]
struct Passport<'a> {
    byr: Option<&'a str>,
    cid: Option<&'a str>,
    ecl: Option<&'a str>,
    eyr: Option<&'a str>,
    hcl: Option<&'a str>,
    hgt: Option<&'a str>,
    iyr: Option<&'a str>,
    pid: Option<&'a str>,
}

impl<'a> Passport<'a> {
    fn extend(&mut self, data: &'a str) -> Result<()> {
        for datum in data.split_whitespace() {
            let datum = datum.trim();
            let mut parts = datum.splitn(2, ":");
            let key = parts.next().context(MissingKey)?;
            let value = parts.next().context(MissingValue { key })?;

            match key {
                "byr" => self.byr = Some(value),
                "cid" => self.cid = Some(value),
                "ecl" => self.ecl = Some(value),
                "eyr" => self.eyr = Some(value),
                "hcl" => self.hcl = Some(value),
                "hgt" => self.hgt = Some(value),
                "iyr" => self.iyr = Some(value),
                "pid" => self.pid = Some(value),
                _ => UnknownKey { key }.fail()?,
            }
        }
        Ok(())
    }

    fn is_valid(&self) -> bool {
        self.byr.is_some()
            && self.ecl.is_some()
            && self.eyr.is_some()
            && self.hcl.is_some()
            && self.hgt.is_some()
            && self.iyr.is_some()
            && self.pid.is_some()
    }
}

struct Batch<'a>(Vec<Passport<'a>>);

impl Batch<'_> {
    #[cfg(test)]
    fn len(&self) -> usize {
        self.0.len()
    }

    fn valid(&self) -> usize {
        self.0.iter().filter(|p| p.is_valid()).count()
    }
}

impl<'a> TryFrom<&'a str> for Batch<'a> {
    type Error = Error;

    fn try_from(other: &'a str) -> Result<Self> {
        let mut batch = vec![];
        let mut passport = Passport::default();

        for l in other.lines() {
            let l = l.trim();
            if l.is_empty() {
                batch.push(mem::take(&mut passport));
            } else {
                passport.extend(l)?;
            }
        }

        if passport != Passport::default() {
            batch.push(passport);
        }

        Ok(Self(batch))
    }
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    let b = Batch::try_from(input)?;
    println!("{}", b.valid());
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
                                byr:1937 iyr:2017 cid:147 hgt:183cm

                                iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
                                hcl:#cfa07d byr:1929

                                hcl:#ae17e1 iyr:2013
                                eyr:2024
                                ecl:brn pid:760753108 byr:1931
                                hgt:179cm

                                hcl:#cfa07d eyr:2025 pid:166559648
                                iyr:2011 ecl:brn hgt:59in"#;

    #[test]
    fn demo() -> Result<()> {
        let b = Batch::try_from(TEST_INPUT)?;
        assert_eq!(b.len(), 4);
        assert_eq!(b.valid(), 2);
        Ok(())
    }
}
