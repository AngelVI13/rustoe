use std::f32;

use crate::defines::*;
use crate::board::Board;

// TODO: maybe split to 2 separate structs
#[derive(Debug)]
pub struct Node {
    pub index: usize,
    pub parent: Option<usize>,
    pub children: Vec<usize>,

    // data
    pub move_: Option<usize>,
    pub wins: f32,
    pub visits: f32,
    pub untried_moves: Vec<usize>,
    pub player_just_moved: Mark,
}

impl Node {
    pub fn new(index: usize, parent: Option<usize>, move_: Option<usize>, state: &Board) -> Self {
        Self {
            index: index,
            parent: parent,
            children: Vec::new(),

            move_: move_,
            wins: 0.0,
            visits: 0.0,
            untried_moves: state.get_moves(),
            player_just_moved: state.player_just_moved,
        }
    }

    pub fn update(&mut self, result: f32) {
        self.visits += 1.0;
        self.wins += result;
    }

    
    pub fn ucb1(&self, child_node: &Node) -> f32 {
        // Implements UCB -> upper confidence boundary that helps select the most 
        // promising child nodes
        // Vi + sqrt( ln(N) / Ni ), where Vi is the estimated value of the node
        // Ni is the number of times the node has been visited,
        // N is the total number of times its parent has been visited
        (child_node.wins / child_node.visits) + 
        (2.0 * (self.visits as f32).ln() / child_node.visits as f32).sqrt()
    }
}

#[derive(Debug, Default)]
pub struct Tree {
    arena: Vec<Node>,
}

impl Tree {
    pub fn new(state: &Board) -> Tree {
        let mut tree_root = Tree::default();

        let parent_index = None;
        let origin_move = None;
        tree_root.add_child(parent_index, origin_move, state);

        tree_root
    }

    pub fn get(&self, node_id: usize) -> &Node {
        &self.arena[node_id]
    }

    pub fn add_child(&mut self, parent: Option<usize>, move_: Option<usize>, state: &Board) -> usize {
        let new_node_index = self.arena.len();
        let new_node = Node::new(new_node_index, parent, move_, state);

        self.arena.push(new_node);

        // register child to parent node (if parent index was provided)
        if let Some(parent_id) = parent {
            let parent_node = &mut self.arena[parent_id];
            parent_node.children.push(new_node_index);

            // If a selected move is provided -> remove it from the parent node's untried moves
            if let Some(selected_move) = move_ {
                // Below does the same as : self.untried_moves.remove_item(&move_).unwrap();
                let index = parent_node.untried_moves.iter().position(|x| *x == selected_move).unwrap();
                parent_node.untried_moves.remove(index);
            }
        }

        new_node_index
    }

    
    pub fn select_child(&self, parent: Option<usize>) -> usize {
        let parent_node: &Node;

        if let Some(parent_index) = parent {
            parent_node = &self.arena[parent_index];
        } else {
            parent_node = &self.arena[0];
        }

        if parent_node.children.len() == 0 {
            panic!("No children to select from");
        }

        let mut best_child_id = parent_node.children[0];
        let mut best_child_ucb = parent_node.ucb1(&self.arena[best_child_id]);

        for child in parent_node.children.iter() {
            let child_ucb = parent_node.ucb1(&self.arena[*child]);
            if child_ucb > best_child_ucb {
                best_child_ucb = child_ucb;
                best_child_id = *child;
            }
        }

        best_child_id
    }
}
