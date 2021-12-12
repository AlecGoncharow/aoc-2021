use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = s.split(",");
        let x = numbers.next().unwrap().parse().unwrap();
        let y = numbers.next().unwrap().parse().unwrap();

        Ok(Point { x, y })
    }
}

#[derive(Debug, Clone, Copy)]
enum Orienation {
    Horizontal,
    Vertical,
    MainDiagonal,
    AntiDiagonal,
}

#[derive(Debug, Clone, Copy)]
struct LineSegment {
    orientation: Orienation,
    source: Point,
    target: Point,
}

impl FromStr for LineSegment {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = s.split(" -> ");

        let p1: Point = points.next().unwrap().parse()?;
        let p2: Point = points.next().unwrap().parse()?;

        let source: Point;
        let target: Point;
        let orientation: Orienation;
        if p1.x == p2.x {
            orientation = Orienation::Vertical;
            if p1.y > p2.y {
                source = p2;
                target = p1;
            } else {
                source = p1;
                target = p2;
            }
        } else if p1.y == p2.y {
            orientation = Orienation::Horizontal;
            if p1.x > p2.x {
                source = p2;
                target = p1;
            } else {
                source = p1;
                target = p2;
            }
        } else {
            if p1.x > p2.x {
                // p1 is right of p2
                source = p2;
                target = p1;
                if p1.y > p2.y {
                    orientation = Orienation::AntiDiagonal;
                } else {
                    orientation = Orienation::MainDiagonal;
                }
            } else {
                // p2 is right of p1
                source = p1;
                target = p2;
                if p1.y > p2.y {
                    orientation = Orienation::MainDiagonal;
                } else {
                    orientation = Orienation::AntiDiagonal;
                }
            }
        }

        Ok(LineSegment {
            orientation,
            source,
            target,
        })
    }
}

impl LineSegment {}

const ROWS: usize = 1_000;
const COLS: usize = 1_000;
type Grid = [u8; ROWS * COLS];

struct Context {
    grid: Grid,
    overlaps: usize,
    diagonals_on: bool,
}

impl Context {
    pub fn draw_segment(&mut self, segment: LineSegment) {
        match segment.orientation {
            Orienation::Horizontal => {
                let source_index = (segment.source.y * COLS) + segment.source.x;
                let target_index = (segment.target.y * COLS) + segment.target.x;

                for i in source_index..=target_index {
                    self.grid[i] += 1;

                    if self.grid[i] == 2 {
                        self.overlaps += 1;
                    }
                }
            }
            Orienation::Vertical => {
                let source_index = (segment.source.y * COLS) + segment.source.x;

                for i in 0..=(segment.target.y - segment.source.y) {
                    self.grid[source_index + (i * COLS)] += 1;

                    if self.grid[source_index + (i * COLS)] == 2 {
                        self.overlaps += 1;
                    }
                }
            }
            Orienation::MainDiagonal if self.diagonals_on => {
                let source_index = (segment.source.y * COLS) + segment.source.x;
                let delta_x = segment.target.x - segment.source.x;

                for i in 0..=delta_x {
                    self.grid[source_index + i - (i * COLS)] += 1;

                    if self.grid[source_index + i - (i * COLS)] == 2 {
                        self.overlaps += 1;
                    }
                }
            }
            Orienation::AntiDiagonal if self.diagonals_on => {
                let source_index = (segment.source.y * COLS) + segment.source.x;
                let delta_x = segment.target.x - segment.source.x;

                for i in 0..=delta_x {
                    self.grid[source_index + i + (i * COLS)] += 1;

                    if self.grid[source_index + i + (i * COLS)] == 2 {
                        self.overlaps += 1;
                    }
                }
            }
            _ => {}
        }
    }
}

fn main() -> std::io::Result<()> {
    let input: Vec<LineSegment> = common::get_input_vec()?;

    let grid: Grid = [0; ROWS * COLS];

    println!("line seg 0 {:?}", input[0]);

    let mut ctx = Context {
        grid,
        overlaps: 0,
        diagonals_on: false,
    };

    for line in &input {
        ctx.draw_segment(*line);
    }

    println!("[Part 1] Overlaps {}", ctx.overlaps);

    let grid: Grid = [0; ROWS * COLS];

    let mut ctx = Context {
        grid,
        overlaps: 0,
        diagonals_on: true,
    };

    for line in input {
        ctx.draw_segment(line);
    }

    println!("[Part 2] Overlaps {}", ctx.overlaps);

    Ok(())
}
