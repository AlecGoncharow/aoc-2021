use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum SubDirection {
    Fwd,
    Down,
    Up,
}

#[derive(Debug, PartialEq)]
struct SubCommand {
    direction: SubDirection,
    scalar: usize,
}

impl FromStr for SubCommand {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let direction = match split.next() {
            Some(inner) => match inner {
                "forward" => Ok(SubDirection::Fwd),
                "up" => Ok(SubDirection::Up),
                "down" => Ok(SubDirection::Down),
                _ => Err(Self::Err::new(
                    std::io::ErrorKind::Other,
                    format!("Oh No! {} is not a direction", inner),
                )),
            },
            None => Err(Self::Err::new(
                std::io::ErrorKind::Other,
                "Oh No! No Direction",
            )),
        }?;

        let scalar = match split.next() {
            Some(inner) => match inner.parse::<usize>() {
                Ok(n) => Ok(n),
                Err(_) => Err(Self::Err::new(
                    std::io::ErrorKind::Other,
                    format!("Oh No! {} is not a usize", inner),
                )),
            },
            None => Err(Self::Err::new(
                std::io::ErrorKind::Other,
                "Oh No! No Scalar",
            )),
        }?;

        Ok(SubCommand { direction, scalar })
    }
}

#[derive(Debug, PartialEq)]
pub struct SubPosition {
    distance: usize,
    depth: usize,
}

impl std::ops::Add<SubCommand> for SubPosition {
    type Output = Self;

    fn add(self, command: SubCommand) -> Self::Output {
        match command.direction {
            SubDirection::Fwd => Self {
                distance: self.distance + command.scalar,
                depth: self.depth,
            },
            SubDirection::Up => Self {
                distance: self.distance,
                depth: self.depth - command.scalar,
            },
            SubDirection::Down => Self {
                distance: self.distance,
                depth: self.depth + command.scalar,
            },
        }
    }
}

impl std::ops::AddAssign<&SubCommand> for SubPosition {
    fn add_assign(&mut self, command: &SubCommand) {
        match command.direction {
            SubDirection::Fwd => self.distance += command.scalar,

            SubDirection::Up => self.depth -= command.scalar,
            SubDirection::Down => self.depth += command.scalar,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SubAimedPosition {
    aim: isize,
    distance: usize,
    depth: usize,
}

impl std::ops::AddAssign<&SubCommand> for SubAimedPosition {
    fn add_assign(&mut self, command: &SubCommand) {
        match command.direction {
            SubDirection::Fwd => {
                self.distance += command.scalar;
                self.depth += command.scalar * self.aim as usize;
            }
            SubDirection::Up => self.aim -= command.scalar as isize,
            SubDirection::Down => self.aim += command.scalar as isize,
        }
    }
}

fn main() -> std::io::Result<()> {
    let input: Vec<SubCommand> = common::get_input_vec()?;

    println!("{:?}", input[0]);

    let mut position = SubPosition {
        distance: 0,
        depth: 0,
    };

    input.iter().for_each(|command| position += command);

    println!(
        "[Part 1] position: {:?}, solution: {}",
        position,
        position.depth * position.distance
    );

    let mut aimed_position = SubAimedPosition {
        aim: 0,
        distance: 0,
        depth: 0,
    };

    input.iter().for_each(|command| aimed_position += command);
    println!(
        "[Part 2] position: {:?}, solution: {}",
        aimed_position,
        aimed_position.depth * aimed_position.distance
    );

    Ok(())
}
