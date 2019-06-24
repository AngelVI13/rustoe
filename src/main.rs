const ROWS: usize = 3;
const BOARD_SIZE: usize = ROWS*ROWS;

#[derive(Debug, Copy, Clone)]
enum Mark {
    X = 1,
    O = -1,
    NoPlayer = 0,
}

#[derive(Debug)]
struct Board {
    pos: [Mark; BOARD_SIZE],
    player_just_moved: Mark,
    // fixed size array (instead of Vec) and a counter to keep 
    // track of the move number will be more performant
    history: Vec<usize>,
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

    fn take_move(&mut self) {
        if let Some(move_int) = self.history.pop() {
            self.pos[move_int] = Mark::NoPlayer;
        } else {
            println!("History is empty");
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
}


fn main() {
    // tttoe example here:
    // https://github.com/flofriday/tictactoe/blob/master/src/main.rs
    let mut b = Board::new();

    let moves = b.get_moves();
    println!("{:?}", moves);
    println!("{:?}", b);
    b.take_move();
    b.make_move(4);
    b.make_move(0);
    let moves = b.get_moves();
    println!("{:?}", moves);
 
    println!("{:?}", b);
    b.take_move();
    println!("{:?}", b);
}