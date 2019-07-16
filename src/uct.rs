use crate::defines::*;
use crate::board::Board;


pub struct Node<'a> {
    move_: Option<usize>,
    parent_node: Option<&'a Self>,
    child_nodes: Vec<&'a Self>,
    wins: i32,
    visits: i32,
    untried_moves: Vec<usize>,
    player_just_moved: Mark,
}

impl<'a> Node<'a> {
    pub fn new_root(state: &Board) -> Node {  // return reference ?
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

    pub fn new_child(move_: usize, parent: &'a Node, state: &'a Board) -> Node<'a> {
        Node {
            move_: Some(move_),
            parent_node: Some(parent),
            child_nodes: Vec::new(),
            wins: 0,
            visits: 0,
            untried_moves: state.get_moves(),
            player_just_moved: state.player_just_moved,
        }
    }

    pub fn update(&mut self, result: i32) {
        self.visits += 1;
        self.wins += result;
    }

    pub fn add_child(&mut self, move_: usize, state: &'a Board) -> &Node {
        let node = Self::new_child(move_, self, state);

        // Below does the same as : self.untried_moves.remove_item(&move_).unwrap();
        let index = self.untried_moves.iter().position(|x| *x == move_).unwrap();
        self.untried_moves.remove(index);

        self.child_nodes.push(&node);
        &node
    }
}