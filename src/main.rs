mod board;
mod defines;
mod uct;
// pub use board::board;
use board::Board;
use defines::{LOSS, WIN, DRAW};
use std::io;
// use uct::Node;


fn main() {
    // Add generic trait implementation required to use the UCT algorithm

    // tttoe example here:
    // https://github.com/flofriday/tictactoe/blob/master/src/main.rs
    let mut b = Board::new();

    while let None = b.get_result(b.player_just_moved) {
        let moves = b.get_moves();
        println!("{}", b);
        println!("Enter move (available: {:?})", moves);
        let mut input_move = String::new();
        io::stdin().read_line(&mut input_move).expect("Failed to read line");
        let move_: u8 = input_move.trim().parse().expect("Please type a positive number!");
        b.make_move_safe(move_ as usize).unwrap();
    }

    println!("{}", b);
    if let Some(winner) = b.get_result(b.player_just_moved) {
        if winner == DRAW { println!("Draw") }
        if winner == WIN { println!("Winner is {:?}", b.player_just_moved) }
        if winner == LOSS { println!("Winner is {:?}", b.update_player_jm(b.player_just_moved)) }
    }
    // let node = Node::new_root(&b);
}
