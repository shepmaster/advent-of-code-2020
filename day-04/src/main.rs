use snafu::{OptionExt, Snafu};
use std::{convert::TryFrom, mem, ops::RangeInclusive};

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

    fn is_deep_valid(&self) -> bool {
        let year_within = |r: RangeInclusive<u16>| {
            move |yr: Option<&str>| {
                yr.map_or(false, |yr| {
                    yr.parse::<u16>().map_or(false, |yr| r.contains(&yr))
                })
            }
        };

        let byr = year_within(1920..=2002)(self.byr);
        let iyr = year_within(2010..=2020)(self.iyr);
        let eyr = year_within(2020..=2030)(self.eyr);

        let hgt = self.hgt.map_or(false, |hgt| {
            if let Some(v) = hgt.strip_suffix("cm") {
                v.parse().map_or(false, |v| (150..=193).contains(&v))
            } else if let Some(v) = hgt.strip_suffix("in") {
                v.parse().map_or(false, |v| (59..=76).contains(&v))
            } else {
                false
            }
        });

        let hcl = self.hcl.map_or(false, |hcl| {
            if let Some(v) = hcl.strip_prefix("#") {
                v.len() == 6
                    && v.chars().all(|c| {
                        matches!(
                            c,
                            '0' | '1'
                                | '2'
                                | '3'
                                | '4'
                                | '5'
                                | '6'
                                | '7'
                                | '8'
                                | '9'
                                | 'a'
                                | 'b'
                                | 'c'
                                | 'd'
                                | 'e'
                                | 'f'
                        )
                    })
            } else {
                false
            }
        });

        let ecl = self.ecl.map_or(false, |ecl| {
            matches!(ecl, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
        });

        let pid = self.pid.map_or(false, |pid| {
            pid.len() == 9 && pid.chars().all(|c| c.is_ascii_digit())
        });

        byr && iyr && eyr && hgt && hcl && ecl && pid
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

    fn deep_valid(&self) -> usize {
        self.0.iter().filter(|p| p.is_deep_valid()).count()
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
    let a1 = b.valid();
    let a2 = b.deep_valid();
    println!("{}", a1);
    println!("{}", a2);
    assert_eq!(200, a1);
    assert_eq!(116, a2);
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

    const TEST_DEEP_INVALID: &str = r#"eyr:1972 cid:100
                                       hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

                                       iyr:2019
                                       hcl:#602927 eyr:1967 hgt:170cm
                                       ecl:grn pid:012533040 byr:1946

                                       hcl:dab227 iyr:2012
                                       ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

                                       hgt:59cm ecl:zzz
                                       eyr:2038 hcl:74454a iyr:2023
                                       pid:3556412378 byr:2007"#;

    #[test]
    fn demo_deep_invalid() -> Result<()> {
        let b = Batch::try_from(TEST_DEEP_INVALID)?;
        assert_eq!(b.len(), 4);
        assert_eq!(b.deep_valid(), 0);
        Ok(())
    }

    const TEST_DEEP_VALID: &str = r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
                                     hcl:#623a2f

                                     eyr:2029 ecl:blu cid:129 byr:1989
                                     iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

                                     hcl:#888785
                                     hgt:164cm byr:2001 iyr:2015 cid:88
                                     pid:545766238 ecl:hzl
                                     eyr:2022

                                     iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"#;

    #[test]
    fn demo_deep_valid() -> Result<()> {
        let b = Batch::try_from(TEST_DEEP_VALID)?;
        assert_eq!(b.len(), 4);
        assert_eq!(b.deep_valid(), 4);
        Ok(())
    }
}
