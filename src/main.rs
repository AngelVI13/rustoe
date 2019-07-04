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
    result_lines: Vec<Vec<Vec<usize>>>,
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
            result_lines: get_result_lines(),
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

    fn get_result(&self, player_jm: Mark) -> Option<f32> {
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
    b.make_move(4);
    b.make_move(8);
    b.make_move(1);
    b.make_move(7);
    b.make_move(2);
    b.make_move(6);
    // b.make_move(5);
    // b.make_move(3);

    // let _result = b.make_move_safe(4).unwrap();

    println!("{:?}", b);
    let result = b.get_result(Mark::X);
    println!("{:?}", result);
    let moves = b.get_moves();
    println!("{:?}", moves);

    b.take_move();
    println!("{:?}", b);
}

fn get_row_vector(col_vec: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut row_vec: Vec<Vec<usize>> = vec![vec![0; ROWS]; ROWS];

    for i in 0..ROWS {
        for j in 0..ROWS {
            // performs the equivalent of zip on a list of lists in python
            // [[1, 2], [3, 4]] -> [[1, 3], [2, 4]]
            row_vec[i][j] = col_vec[j][i];
        }
    }

    row_vec
}

fn get_column_vector() -> Vec<Vec<usize>> {
    let mut col_vec: Vec<Vec<usize>> = vec![vec![0; ROWS]; ROWS];

    // take every nth element from board starting from each of the first row columns
    // i.e. 0-3-6, 1-4-7 etc.
    for i in 0..ROWS {
        let v_temp = (i..BOARD_SIZE).
            step_by(ROWS).
            collect::<Vec<usize>>();

        col_vec[i] = v_temp;
    }

    col_vec
}


fn get_diagonals(row_vec: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut left_diagonal: Vec<usize> = vec![0; ROWS];
    let mut right_diagonal: Vec<usize> = vec![0; ROWS];

    for i in 0..ROWS {
        left_diagonal[i] = row_vec[i][i]; // left diagonal -> 0,0| 1,1| 2,2
        right_diagonal[i] = row_vec[i][ROWS-i-1] // right diagonal 0,2 | 1,1| 2,0 .
        // Additional -1 is needed to convert from size to index
    }

    let diagonals = vec![left_diagonal, right_diagonal];
    diagonals
}


fn get_result_lines() -> Vec<Vec<Vec<usize>>> {
    let col_vec = get_column_vector();
    let row_vec = get_row_vector(&col_vec);
    let diagonal_vec = get_diagonals(&row_vec);

    let result_lines = vec![col_vec, row_vec, diagonal_vec];
    result_lines
}
