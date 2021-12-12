#![feature(str_split_as_str)]
#![feature(drain_filter)]

use std::str::FromStr;

#[derive(Debug)]
pub struct BingoBoard {
    mask: u128,
    board: [[(u8, bool); 5]; 5],
}

impl BingoBoard {
    pub fn contains_selected(&self, selected: u8) -> bool {
        self.mask & (1 << selected) > 0
    }

    // returns true if board wins after marking selected else false
    pub fn process(&mut self, selected: u8) -> bool {
        if !self.contains_selected(selected) {
            return false;
        }

        // go set bool
        for index in 0..25 {
            let i = index / 5;
            let j = index % 5;

            if self.board[i][j].0 == selected {
                self.board[i][j] = (selected, true);
                break;
            }
        }

        // check win

        // rows
        let mut index = 0;
        loop {
            let i = index / 5;
            let j = index % 5;

            if self.board[i][j].1 {
                if j == 4 {
                    println!("wins on rows");
                    return true;
                }
            } else {
                if i != 4 {
                    index += 4 - j;
                } else {
                    break;
                }
            }

            index += 1;
            if index > 24 {
                break;
            }
        }

        // columns
        let mut index = 0;
        loop {
            let i = index / 5;
            let j = index % 5;

            if self.board[j][i].1 {
                if j == 4 {
                    println!("wins on cols");
                    return true;
                }
            } else {
                if i != 4 {
                    index += 4 - j;
                } else {
                    break;
                }
            }

            index += 1;
            if index > 24 {
                break;
            }
        }

        // diags
        let mut win = true;
        for i in 0..5 {
            if !self.board[i][i].1 {
                win = false;
                break;
            }
        }
        for i in 0..5 {
            if !self.board[i][4 - i].1 {
                win = false;
                break;
            }
        }

        if win {
            println!("wins on diag");
        }

        win
    }
}

impl FromStr for BingoBoard {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.split("\n");
        let mut mask: u128 = 0;
        let mut board = [[(0u8, false); 5]; 5];

        for i in 0..5 {
            let mut line = lines.next().unwrap().split_whitespace();
            for j in 0..5 {
                let value = match line.next().unwrap().parse::<u8>() {
                    Ok(n) => n,
                    Err(e) => panic!("Oh No! {} is not a u8", e),
                };
                mask |= 1 << value;
                board[i][j] = (value, false);
            }
        }

        Ok(BingoBoard { mask, board })
    }
}

fn main() -> std::io::Result<()> {
    let input_string = common::get_input()?;

    let mut split = input_string.split("\n");

    let selected: Vec<u8> = common::parse_split(split.next().unwrap().split(","))?;
    println!("selected 0: {}, len {}", selected[0], selected.len());

    let mut boards: Vec<BingoBoard> = common::parse_split(split.as_str().trim().split("\n\n"))?;
    println!("board 0: {:?}, mask: {:b}", boards[0], boards[0].mask);

    // part one
    let mut selected_mask = 0u128;
    for current_selected in &selected {
        selected_mask |= 1 << current_selected;

        let mut winning_boards: Vec<&mut BingoBoard> = boards
            .iter_mut()
            .filter_map(|board| {
                if board.process(*current_selected) {
                    Some(board)
                } else {
                    None
                }
            })
            .collect();

        if winning_boards.len() > 0 {
            let winning_board = winning_boards.pop().unwrap();
            println!("We have a winning board: {:?}", winning_board);
            let board_selected_mask = winning_board.mask & selected_mask;
            let not_selected_board_mask = winning_board.mask ^ board_selected_mask;
            println!("winning_board.mask: {:b}", winning_board.mask);
            println!("selected_mask: {:b}", selected_mask);
            println!("not_selected_board_mask: {:b}", not_selected_board_mask);

            let mut sum_unselected = 0usize;
            for i in 0..100 {
                if not_selected_board_mask & (1 << i) > 0 {
                    println!("{}", i);
                    sum_unselected += i;
                }
            }

            println!(
                "[Part One] sum_unselected {}, current_selected {}, solution {}",
                sum_unselected,
                current_selected,
                sum_unselected * *current_selected as usize
            );

            break;
        }
    }

    let mut selected_mask = 0u128;
    let mut boards: Vec<BingoBoard> = common::parse_split(split.as_str().trim().split("\n\n"))?;
    for current_selected in selected {
        selected_mask |= 1 << current_selected;
        if boards.len() == 1 {
            if boards[0].process(current_selected) {
                let winning_board = boards.pop().unwrap();
                println!("We have a winning board: {:?}", winning_board);
                let board_selected_mask = winning_board.mask & selected_mask;
                let not_selected_board_mask = winning_board.mask ^ board_selected_mask;

                let mut sum_unselected = 0usize;
                for i in 0..100 {
                    if not_selected_board_mask & (1 << i) > 0 {
                        println!("{}", i);
                        sum_unselected += i;
                    }
                }

                println!(
                    "[Part Two] sum_unselected {}, current_selected {}, solution {}",
                    sum_unselected,
                    current_selected,
                    sum_unselected * current_selected as usize
                );
                break;
            }
        } else {
            boards = boards
                .drain_filter(|board| !board.process(current_selected))
                .collect();
        }
    }

    Ok(())
}
