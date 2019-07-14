use crate::defines::*;
use crate::board::Board;


pub struct Node {
    move_: Option<usize>,
    parentNode: Option<&Board>,
    childNodes: Vec<&Board>,
    wins: i32,
    visits: i32,
    untriedMoves: Vec<usize>,
    playerJustMoved: Mark,
}

impl Node {
    pub fn new(state: &Board) -> Node {
        Node {
            move_: None,
            parentNode: None,
            childNodes: Vec::new(),
            wins: 0,
            visits: 0,
            untriedMoves: state.get_moves(),
            playerJustMoved: state.player_just_moved,
        }
    }
}