use std::fmt;

const ROWS: usize = 3;
const BOARD_SIZE: usize = ROWS*ROWS;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Mark {
    X = 1,
    O = -1,
    NoPlayer = 0,
}

// Game result scores
const LOSS: f32 = 0.0;
const DRAW: f32 = 0.5;
const WIN: f32 = 1.0;


#[derive(Debug)]
struct Board {
    pos: [Mark; BOARD_SIZE],
    player_just_moved: Mark,
    // fixed size array (instead of Vec) and a counter to keep
    // track of the move number will be more performant
    history: Vec<usize>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Board(pos={:?}, player_just_moved={:?}, history={:?})",
        self.pos, self.player_just_moved, self.history)
    }
}

impl Board {
    fn new() -> Board {
        // Returns a new board initialized to "0"/default values
        Board {
            pos: [Mark::NoPlayer; BOARD_SIZE],
            player_just_moved: Mark::O,
            history: Vec::new(),
        }
    }

    // only used for valid/verified input, any other input goes through make_move_safe
    fn make_move(&mut self, move_int: usize) {
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
    fn make_move_safe(&mut self, move_int: usize) -> Result<(), String> {
        if !self.get_moves().contains(&move_int) {
            return Err(format!("Invalid move value: {}", move_int));
        }

        self.player_just_moved = match self.player_just_moved {
            Mark::X => Mark::O,
            Mark::O => Mark::X,
            // for all other cases set to NoPlayer (should not happen)
            _ => Mark::NoPlayer,
        };

        self.pos[move_int] = self.player_just_moved;
        self.history.push(move_int);

        Ok(())  // return empty result if everything went okay
    }

    fn take_move(&mut self) {
        if let Some(move_int) = self.history.pop() {
            self.pos[move_int] = Mark::NoPlayer;
        } else {
            println!("History is empty");
            // todo panic instead of print in order to catch logic errors
        }
    }

    fn get_moves(&self) -> Vec<usize> {
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

    fn get_result(&self, player_jm: Mark) -> Option<f32> {
        for idx in 0..ROWS {
            // Checks result column-wise i.e. 0-3-6/1-4-7/2-5-8 indexes
            if self.pos[idx] == self.pos[idx + ROWS] && self.pos[idx] == self.pos[idx + 2*ROWS] && self.pos[idx] != Mark::NoPlayer {
                return if self.pos[idx] == player_jm { Some(WIN) } else { Some(LOSS) };
            }

            let idx = idx * ROWS;

            // Checks result row-wise i.e. 0-1-2/3-4-5/6-7-8
            if self.pos[idx] == self.pos[idx + 1] && self.pos[idx] == self.pos[idx + 2] && self.pos[idx] != Mark::NoPlayer {
                return if self.pos[idx] == player_jm { Some(WIN) } else { Some(LOSS) };
            }
        }

        // Check result for left diagonal i.e. 0-4-8
        if self.pos[0] == self.pos[ROWS + 1] && self.pos[0] == self.pos[2*ROWS + 2] && self.pos[0] != Mark::NoPlayer {
            return if self.pos[0] == player_jm { Some(WIN) } else { Some(LOSS) };
        }

        // Check result for left diagonal i.e. 2-4-6
        if self.pos[ROWS - 1] == self.pos[2*ROWS - 2] && self.pos[ROWS - 1] == self.pos[2*ROWS] && self.pos[ROWS - 1] != Mark::NoPlayer {
            return if self.pos[ROWS - 1] == player_jm { Some(WIN) } else { Some(LOSS) };
        }

        // If no result and no moves left => DRAW
        if self.get_moves().len() == 0 {
            return Some(DRAW);
        }

        return None;
    }
}


fn main() {
    // tttoe example here:
    // https://github.com/flofriday/tictactoe/blob/master/src/main.rs
    let mut b = Board::new();

    let moves = b.get_moves();
    println!("{:?}", moves);
    println!("{:?}", b);
    println!("{}", b);
    b.take_move();
    b.make_move(0);
    // b.make_move(4);
    // b.make_move(8);
    // b.make_move(1);
    // b.make_move(7);
    // b.make_move(6);
    // b.make_move(2);
    // b.make_move(5);
    // b.make_move(3);

    let _result = b.make_move_safe(4).unwrap();

    let result = b.get_result(Mark::X);
    println!("{:?}", result);
    let moves = b.get_moves();
    println!("{:?}", moves);

    println!("{:?}", b);
    b.take_move();
    println!("{:?}", b);

    let mut col_arr: Vec<Vec<usize>> = vec![vec![0; ROWS]; ROWS];

    get_column_array(&mut col_arr);

    // how to do zip(col_array) to go from 0-3-6|1-4-7|2-5-8 -> 0-1-2|3-4-5|6-7-8
    // for arbitrary sub vectors into multi dimensional vector
    println!("{:?}", col_arr);
}


fn get_column_array(v: &mut Vec<Vec<usize>>) {
    for i in 0..ROWS {
        let v_temp = (i..BOARD_SIZE).step_by(ROWS).collect::<Vec<usize>>();
        v[i] = v_temp;
    }
}