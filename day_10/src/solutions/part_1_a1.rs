use core::panic;
use std::{fmt::Error, str::FromStr};

#[derive(Debug, Clone)]
struct Maze {
    grid: Vec<Vec<((i32, i32), (i32, i32))>>,
    width: usize,
    height: usize,
    start: (i32, i32),
}

impl Maze {
    pub fn length(&self) -> u32 {
        // println!("Start: {:?}", self.start);
        let mut current = self.check_start_sides();
        let mut previous = self.start;
        let mut distance = 1;
        let mut not_looped = true;
        while not_looped {
            let (dir1, dir2) = self.grid[current.0 as usize][current.1 as usize];
            // println!("x: {} y: {}", current.0, current.1);
            // println!("{:?}", self.grid[current.0 as usize][current.1 as usize]);

            // Check if dir1 is the previous tile
            if current.0 + dir1.0 == previous.0 && current.1 + dir1.1 == previous.1 {
                previous = current;
                current = (current.0 + dir2.0, current.1 + dir2.1)
            } else {
                previous = current;
                current = (current.0 + dir1.0, current.1 + dir1.1)
            }

            distance += 1;

            if current == self.start {
                not_looped = false;
            }
        }
        distance / 2
    }

    fn check_start_sides(&self) -> (i32, i32) {
        let x = self.start.0 as usize;
        let y = self.start.1 as usize;
        // println!("{:?}", self.grid);
        if x > 0 && (self.grid[x - 1][y].0 .0 == 1 || self.grid[x - 1][y].1 .0 == 1) {
            return (x as i32 - 1, y as i32);
        } else if x < self.width && self.grid[x + 1][y].0 .0 == -1 {
            return (x as i32 + 1, y as i32);
        } else if y > 0 && (self.grid[x][y - 1].0 .1 == 1 || self.grid[x][y - 1].1 .1 == 1) {
            return (y as i32 - 1, y as i32);
        } else if y < self.height
            && (self.grid[x][y + 1].0 .1 == -1 || self.grid[x][y + 1].1 .1 == -1)
        {
            return (y as i32 + 1, y as i32);
        }
        panic!()
    }
}

impl FromStr for Maze {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines();
        let height = s.lines().into_iter().next().unwrap().len();
        let width = s.len() / height;
        let mut grid: Vec<Vec<((i32, i32), (i32, i32))>> =
            vec![vec!(((0, 0), (0, 0)); width); height];
        let mut start: Option<(i32, i32)> = None;

        for (j, line) in lines.enumerate() {
            for (i, byte) in line.bytes().enumerate() {
                match byte {
                    b'|' => grid[i][j] = ((0, -1), (0, 1)),
                    b'-' => grid[i][j] = ((-1, 0), (1, 0)),
                    b'L' => grid[i][j] = ((0, -1), (1, 0)),
                    b'J' => grid[i][j] = ((-1, 0), (0, -1)),
                    b'7' => grid[i][j] = ((-1, 0), (0, 1)),
                    b'F' => grid[i][j] = ((0, 1), (1, 0)),
                    b'S' => {
                        start = Some((i as i32, j as i32));
                        grid[i][j] = ((0, 0), (0, 0))
                    }
                    _ => grid[i][j] = ((0, 0), (0, 0)),
                }
            }
        }

        Ok(Maze {
            grid,
            width,
            height,
            start: start.unwrap(),
        })
    }
}

pub fn solution(input: &str) -> u32 {
    let maze = input.parse::<Maze>().unwrap();
    maze.length()
}
