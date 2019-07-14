mod board;
mod defines;
mod uct;
// pub use board::board;
use board::Board;
use defines::Mark;
use uct::Node;


fn main() {
    // Add generic trait implementation required to use the UCT algorithm 

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

    let node = Node::new(&b);
}
