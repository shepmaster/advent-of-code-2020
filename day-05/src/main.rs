use snafu::{ensure, Snafu};
use std::{cell::RefCell, str::FromStr};

#[derive(Debug, Snafu)]
enum PassError {
    InvalidDigitCount,

    #[snafu(context(false))]
    InvalidColumn {
        source: ColError,
    },

    #[snafu(context(false))]
    InvalidRow {
        source: RowError,
    },
}

struct Pass {
    rows: Vec<Row>,
    cols: Vec<Col>,
    row: RefCell<Option<u8>>,
    col: RefCell<Option<u8>>,
}

impl Pass {
    fn row(&self) -> u8 {
        let rs = self.rows.iter().copied().map(Direction::from);
        *self
            .row
            .borrow_mut()
            .get_or_insert_with(|| winnow(rs, 0, 127))
    }

    fn col(&self) -> u8 {
        let rs = self.cols.iter().copied().map(Direction::from);
        *self
            .col
            .borrow_mut()
            .get_or_insert_with(|| winnow(rs, 0, 7))
    }

    #[cfg(test)]
    fn seat(&self) -> (u8, u8) {
        (self.row(), self.col())
    }

    fn id(&self) -> u16 {
        u16::from(self.row()) * 8 + u16::from(self.col())
    }
}

impl FromStr for Pass {
    type Err = PassError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ensure!(s.len() == 10, InvalidDigitCount);

        let mut parts = s.split("").filter(|s| !s.is_empty());
        let rows = parts
            .by_ref()
            .take(7)
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        let cols = parts.map(str::parse).collect::<Result<_, _>>()?;

        Ok(Self {
            rows,
            cols,
            row: Default::default(),
            col: Default::default(),
        })
    }
}

#[derive(Copy, Clone)]
enum Direction {
    B,
    S,
}

fn winnow(dir: impl IntoIterator<Item = Direction>, mut min: u8, mut max: u8) -> u8 {
    for d in dir {
        use Direction::*;
        let mid = (min + max) / 2;
        match d {
            B => max = mid,
            S => min = mid,
        }
    }

    max
}

impl From<Row> for Direction {
    fn from(other: Row) -> Self {
        match other {
            Row::F => Direction::B,
            Row::B => Direction::S,
        }
    }
}

impl From<Col> for Direction {
    fn from(other: Col) -> Self {
        match other {
            Col::L => Direction::B,
            Col::R => Direction::S,
        }
    }
}

#[derive(Debug, Snafu)]
struct RowError {
    s: String,
}

#[derive(Copy, Clone)]
enum Row {
    F,
    B,
}

impl FromStr for Row {
    type Err = RowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "F" => Self::F,
            "B" => Self::B,
            s => RowContext { s }.fail()?,
        })
    }
}

#[derive(Debug, Snafu)]
struct ColError {
    s: String,
}

#[derive(Copy, Clone)]
enum Col {
    L,
    R,
}

impl FromStr for Col {
    type Err = ColError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "L" => Self::L,
            "R" => Self::R,
            s => ColContext { s }.fail()?,
        })
    }
}

fn main() -> Result<(), PassError> {
    let passes = include_str!("../input.txt");

    let a1 = itertools::process_results(passes.lines().map(str::trim).map(Pass::from_str), |ps| {
        ps.map(|p| p.id()).max()
    })?;

    println!("{:?}", a1);
    assert_eq!(a1, Some(906));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &[(&str, (u8, u8), u16)] = &[
        ("FBFBBFFRLR", (44, 5), 357),
        ("BFFFBBFRRR", (70, 7), 567),
        ("FFFBBBFRRR", (14, 7), 119),
        ("BBFFBBFRLL", (102, 4), 820),
    ];

    #[test]
    fn demo() -> Result<(), Box<dyn std::error::Error>> {
        for (i, &(input, seat, id)) in TEST_INPUT.iter().enumerate() {
            let p = Pass::from_str(input)?;
            assert_eq!(seat, p.seat(), "TEST_INPUT index {} failed", i);
            assert_eq!(id, p.id(), "TEST_INPUT index {} failed", i);
        }
        Ok(())
    }
}
