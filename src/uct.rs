use crate::defines::*;
use crate::board::Board;


pub struct Node<'a> {
    move_: Option<usize>,
    parent_node: Option<&'a Board>,
    child_nodes: Vec<&'a Board>,
    wins: i32,
    visits: i32,
    untried_moves: Vec<usize>,
    player_just_moved: Mark,
}

impl<'a> Node<'a> {
    pub fn new(state: &Board) -> Node {
        Node {
            move_: None,
            parent_node: None,
            child_nodes: Vec::new(),
            wins: 0,
            visits: 0,
            untried_moves: state.get_moves(),
            player_just_moved: state.player_just_moved,
        }
    }
}