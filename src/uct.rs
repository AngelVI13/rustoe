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
    pub fn new() -> Self {
        Self { 
            move_: None, 
            wins: 0, 
            visits: 0, 
            untried_moves: Vec::new(), 
            player_just_moved: Mark::O
        }
    }
}

#[derive(Debug)]
pub struct Node {
    parent: Option<NodeId>,
    children: Vec<Option<NodeId>>,
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

        // Push the new node into the arena
        self.nodes.push(Node {
            parent: None,
            children: Vec::new(),
            data: data,
        });

        // Return the node identifier
        NodeId { index: next_index }
    }

    pub fn add_child(&mut self, parent_id: NodeId, child_id: NodeId) {
        if let Some(parent) = self.nodes.get_mut(parent_id.index) {
            parent.children.push(Some(child_id));

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

// impl<'a> Node<'a> {
//     pub fn new_root(state: &Board) -> Node {  // return reference ?
//         Node {
//             move_: None,
//             parent_node: None,
//             child_nodes: Vec::new(),
//             wins: 0,
//             visits: 0,
//             untried_moves: state.get_moves(),
//             player_just_moved: state.player_just_moved,
//         }
//     }

//     pub fn new_child(move_: usize, parent: &'a Node, state: &'a Board) -> Node<'a> {
//         Node {
//             move_: Some(move_),
//             parent_node: Some(parent),
//             child_nodes: Vec::new(),
//             wins: 0,
//             visits: 0,
//             untried_moves: state.get_moves(),
//             player_just_moved: state.player_just_moved,
//         }
//     }

//     pub fn update(&mut self, result: i32) {
//         self.visits += 1;
//         self.wins += result;
//     }

//     pub fn add_child(&mut self, move_: usize, state: &'a Board) -> &Node {
//         let node = Self::new_child(move_, self, state);

//         // Below does the same as : self.untried_moves.remove_item(&move_).unwrap();
//         let index = self.untried_moves.iter().position(|x| *x == move_).unwrap();
//         self.untried_moves.remove(index);

//         self.child_nodes.push(&node);
//         &node
//     }
// }