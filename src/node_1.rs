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

    // TODO: add logic for adding root, child & selecting child to explore. Probably should be
    // handled in the arena impl instead of the node itself
}

#[derive(Debug, Default)]
pub struct Tree {
    arena: Vec<Node>,
}

impl Tree {
    pub fn new() -> Self {
        Tree::default()
    }

    pub fn get(&self, node_id: usize) -> &Node {
        &self.arena[node_id]
    }

    pub fn add(&mut self, parent: Option<usize>, move_: Option<usize>, state: &Board) -> usize {
        let new_node_index = self.arena.len();
        let new_node = Node::new(new_node_index, parent, move_, state);

        self.arena.push(new_node);

        // register child to parent node (if parent index was provided)
        if let Some(parent_id) = parent {
            self.arena[parent_id].children.push(new_node_index);
        }

        new_node_index
    }

    
    pub fn select_child(&self, parent: Option<usize>) -> usize {
        if Some(parent_index) = parent {
            let parent_node = self.arena[parent_index]
        } else {
            let parent_node = self.arena[0]
        }

        if parent_node.children.len() == 0 {
            panic!("No children to select from")
        }

        let mut best_child_id = parent_node.children[0];
        let mut best_child_ucb = parent_node.ucb1(self.arena[best_child_id]);

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
