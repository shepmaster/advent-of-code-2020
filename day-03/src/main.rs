use std::{collections::BTreeSet, convert::Infallible, str::FromStr};

type Result<T, E = Infallible> = std::result::Result<T, E>;

#[derive(Copy, Clone)]
struct Right(usize);

#[derive(Copy, Clone)]
struct Down(usize);

type Coord = (usize, usize);
struct Map {
    grid: BTreeSet<Coord>,
    width: usize,
    height: usize,
}

impl Map {
    const SLOPES: &'static [(Right, Down)] = &[
        (Right(1), Down(1)),
        (Right(3), Down(1)),
        (Right(5), Down(1)),
        (Right(7), Down(1)),
        (Right(1), Down(2)),
    ];

    fn intersections(&self, right: Right, down: Down) -> usize {
        let mut x = 0;
        let mut intersections = 0;

        for y in (0..self.height).step_by(down.0) {
            if self.grid.contains(&(x, y)) {
                intersections += 1;
            }

            x = (x + right.0) % self.width;
        }

        intersections
    }

    fn intersection_product(&self) -> usize {
        Self::SLOPES
            .iter()
            .map(|&(right, down)| self.intersections(right, down))
            .product()
    }
}

impl FromStr for Map {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.trim().chars().enumerate().filter_map(move |(x, c)| {
                    if c == '#' {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .collect();

        let height = s.lines().count();
        let width = s.lines().next().map_or(0, |l| l.trim().len());

        Ok(Self {
            grid,
            height,
            width,
        })
    }
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    let m = Map::from_str(input)?;
    println!("{}", m.intersections(Right(3), Down(1)));
    println!("{}", m.intersection_product());
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"..##.......
                                #...#...#..
                                .#....#..#.
                                ..#.#...#.#
                                .#...##..#.
                                ..#.##.....
                                .#.#.#....#
                                .#........#
                                #.##...#...
                                #...##....#
                                .#..#...#.#"#;

    #[test]
    fn demo() -> Result<()> {
        let m = Map::from_str(TEST_INPUT)?;
        assert_eq!(m.intersections(Right(3), Down(1)), 7);
        Ok(())
    }

    fn demo_product() -> Result<()> {
        let m = Map::from_str(TEST_INPUT)?;
        assert_eq!(m.intersection_product(), 336);
        Ok(())
    }
}
