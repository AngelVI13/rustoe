
use crate::defines::*;
use std::fmt;

#[derive(Debug)]
pub struct Board {
    pos: [Mark; BOARD_SIZE],
    player_just_moved: Mark,
    // fixed size array (instead of Vec) and a counter to keep
    // track of the move number will be more performant
    history: Vec<usize>,
    result_lines: Vec<Vec<Vec<usize>>>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board_string = String::new();

        for row_line in self.result_lines[1].iter() {
            let mut line = String::new();
            for idx in row_line.iter() {
                let mark = match self.pos[*idx] {
                    Mark::NoPlayer => String::from("-"),
                    Mark::X => String::from("X"),
                    Mark::O => String::from("O"),
                };

                let var = format!("| {} ", mark);
                line.push_str(&var);
            }
            board_string.push_str(&format!("\t{}|\n", line));
        }

        let player_to_move = match self.player_just_moved {
            Mark::O => Mark::X,
            Mark::X => Mark::O,
            _ => Mark::NoPlayer,
        };

        write!(f, "\n\tPlayer to move {:?}\n\n{}", player_to_move, board_string)
    }
}

impl Board {
    pub fn new() -> Board {
        // Returns a new board initialized to "0"/default values
        Board {
            pos: [Mark::NoPlayer; BOARD_SIZE],
            player_just_moved: Mark::O,
            history: Vec::new(),
            result_lines: get_result_lines(),
        }
    }

    // only used for valid/verified input, any other input goes through make_move_safe
    pub fn make_move(&mut self, move_int: usize) {
        self.player_just_moved = match self.player_just_moved {
            Mark::X => Mark::O,
            Mark::O => Mark::X,
            // for all other cases set to NoPlayer (should not happen)
            _ => Mark::NoPlayer,
        };

        self.pos[move_int] = self.player_just_moved;
        self.history.push(move_int);
    }

    // Used for parsing user input move
    pub fn make_move_safe(&mut self, move_int: usize) -> Result<(), String> {
        if !self.get_moves().contains(&move_int) {
            return Err(format!("Invalid move value: {}", move_int));
        }

        self.make_move(move_int);

        Ok(())  // return empty result if everything went okay
    }

    pub fn take_move(&mut self) {
        if let Some(move_int) = self.history.pop() {
            self.pos[move_int] = Mark::NoPlayer;
        } else {
            println!("History is empty");
            // todo panic instead of print in order to catch logic errors
        }
    }

    pub fn get_moves(&self) -> Vec<usize> {
        // Implementation might be faster if Board is initialized with
        // fixed size array and values are removed/added on every
        // make/take_move
        let mut possible_moves: Vec<usize> = Vec::new();

        for (idx, value) in self.pos.iter().enumerate() {
            match value {
                Mark::NoPlayer => possible_moves.push(idx),
                _ => continue,
            }
        }
        return possible_moves;
    }

    fn evaluate_lines(&self, lines: &Vec<Vec<usize>>, player_jm: &Mark) -> Option<f32> {
        for line in lines.iter() {
            // let mut result: Vec<Mark> = Vec::new();
            let result = line.iter().map(|x| self.pos[*x] as i8).collect::<Vec<i8>>();
            // the first element of result vec would also be the winner mark if line has result
            let potential_winner = result[0];
            // sum all elements in a row to find if there is a winner
            let result: i8 = result.iter().sum();
            if result.abs() == ROWS as i8 {
                return if potential_winner == *player_jm as i8 { Some(WIN) } else { Some(LOSS) };
            }
        }

        return None;
    }

    pub fn get_result(&self, player_jm: Mark) -> Option<f32> {
        for line in self.result_lines.iter() {
            if let Some(winner) = self.evaluate_lines(&line, &player_jm) {
                return Some(winner);
            }
        }

        // If no result and no moves left => DRAW
        if self.get_moves().len() == 0 {
            return Some(DRAW);
        }

        return None;
    }
}