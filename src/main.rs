mod board;
mod defines;
mod node;
mod node_1;
mod uct;
use board::Board;
use defines::{LOSS, WIN, DRAW};
use std::io;
// use node::{Arena, NodeData};
use node_1::{Tree, Node};
use uct::uct;

fn main() {
    // Add generic trait implementation required to use the UCT algorithm

    // tttoe example here:
    // https://github.com/flofriday/tictactoe/blob/master/src/main.rs
    // let node = Node::new_root(&b);
    // let b = Board::new();
    // let mut tree = Tree::new(&b);

    play_user_game();
    // let node1 = arena.new_node(NodeData::default());
    // let node2 = arena.new_node(NodeData::default());

    // arena.add_child(node1, node2);

    // println!("{:?}", tree);
}

fn play_user_game() {
    let mut b = Board::new();

    while let None = b.get_result(b.player_just_moved) {
        uct(&b, 10000);

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
}
