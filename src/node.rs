use std::f32;

use crate::defines::*;
use crate::board::Board;

#[derive(Debug, Copy, Clone)]
pub struct NodeId {
    index: usize,
}

#[derive(Debug)]
pub struct NodeData {
    move_: Option<usize>,
    wins: i32,
    visits: i32,
    untried_moves: Vec<usize>,
    player_just_moved: Mark,
}

impl NodeData {
    pub fn default() -> Self {
        Self { 
            move_: None, 
            wins: 0, 
            visits: 0, 
            untried_moves: Vec::new(), 
            player_just_moved: Mark::O
        }
    }

    pub fn new(move_: Option<usize>, state: &Board) -> Self {
        Self {
            move_: move_,
            wins: 0,
            visits: 0,
            untried_moves: state.get_moves(),
            player_just_moved: state.player_just_moved,
        }
    }
}

#[derive(Debug)]
pub struct Node {
    id: NodeId,
    parent: Option<NodeId>,    
    children: Vec<NodeId>,
    data: NodeData,
}

#[derive(Debug)]
pub struct Arena {
    nodes: Vec<Node>,
}

impl Arena {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn new_node(&mut self, data: NodeData) -> NodeId {
        // Get the next index in the nodes
        let next_index = self.nodes.len();
        let new_node_id = NodeId { index: next_index };

        // Push the new node into the arena
        self.nodes.push(Node {
            id: new_node_id,
            parent: None,
            children: Vec::new(),
            data: data,
        });

        // Return the node identifier
        new_node_id
    }

    pub fn add_child(&mut self, parent_id: NodeId, child_id: NodeId) {
        if let Some(parent) = self.nodes.get_mut(parent_id.index) {
            parent.children.push(child_id);

            if let Some(child) = self.nodes.get_mut(child_id.index) {
                child.parent = Some(parent_id);
            } else {
                panic!("Couldn't find child!")
            }
        } else {
            panic!("Couldn't find parent!")
        }
    }
}

impl Node {
    pub fn new_root(state: &Board, arena: &mut Arena) -> NodeId {
        let origin_move = None;
        let node_data = NodeData::new(origin_move, state);
        arena.new_node(node_data)
    }

    fn new_child(&self, arena: &mut Arena, move_: usize, state: &Board) -> NodeId {
        // Create a new node in arena using NodeData from provided state and move values
        // Then add the parent <-> child information to the corresponding Nodes inside arena
        // NOTE: this function should only be called from add_child() 
        let new_child_id = arena.new_node(NodeData::new(Some(move_), state));
        arena.add_child(self.id, new_child_id);
        new_child_id
    }

    pub fn update(&mut self, result: i32) {
        self.data.visits += 1;
        self.data.wins += result;
    }

    pub fn add_child(&mut self, arena: &mut Arena, move_: usize, state: &Board) -> NodeId {
        let node_id = self.new_child(arena, move_, state);

        // Below does the same as : self.untried_moves.remove_item(&move_).unwrap();
        let index = self.data.untried_moves.iter().position(|x| *x == move_).unwrap();
        self.data.untried_moves.remove(index);

        node_id
    }

    fn ucb1(&self, node: &Node) -> f32 {
        // Implements UCB -> upper confidence boundary that helps select the most 
        // promising childe nodes
        // Vi + sqrt( ln(N) / Ni ), where Vi is the estimated value of the node
        // Ni is the number of times the node has been visited,
        // N is the total number of times its parent has been visited
        (node.data.wins / node.data.visits) as f32 + 
        (2.0 * (self.data.visits as f32).ln() / node.data.visits as f32).sqrt()
    }

    pub fn select_child(&self, arena: &Arena) -> NodeId {
        if self.children.len() == 0 {
            panic!("No children to select from")
        }

        let mut best_child_id = self.children[0];
        let mut best_child_ucb = self.ucb1(&arena.nodes[best_child_id.index]);

        for child in self.children.iter() {
            let child_ucb = self.ucb1(&arena.nodes[child.index]);
            if child_ucb > best_child_ucb {
                best_child_ucb = child_ucb;
                best_child_id = *child;
            }
        }

        best_child_id
    }
}