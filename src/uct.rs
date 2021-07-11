use rand::seq::SliceRandom;

use crate::defines::*;
use crate::board::Board;
// use crate::node::{Arena, Node, NodeId, NodeData};

pub fn uct(rootstate: Board, itermax: i32) -> (f32, f32) {
    (0.0, 0.0)
}

// Add stuct for return result
// pub fn uct(rootstate: Board, arena: &mut Arena, itermax: i32) -> (f32, f32) {
//     let rootnode_id = Node::new_root(&rootstate, arena);
// 
//     let mut state = rootstate;
//     for _i in 0..itermax {
//         let mut node = &mut arena.nodes[rootnode_id.index];
//         let mut moves_to_root = 0;
// 
//         // Select state
//         // node is fully expanded and non-terminal
//         while node.data.untried_moves.len() == 0 && node.children.len() > 0 {
//             let selected_child_id = node.select_child(arena);
//             node = arena.get(&selected_child_id);
//             state.make_move(node.data.move_.expect("Move missing!"));
//             moves_to_root += 1;
//         }
// 
//         // Expand
//         // If we can expand (i.e. state/node is non-terminal)
//         if node.data.untried_moves.len() > 0 {
//             let move_ = node.data.untried_moves.choose(&mut rand::thread_rng());
//             let move_ = *move_.expect("Move missing!"); // unpack move from Option
//             state.make_move(move_);
//             moves_to_root += 1;
//             let new_child_id = node.add_child(arena, move_, &state); 
//             node = arena.get(&new_child_id);
//         }
// 
//         // Rollout
//         // While state is non-terminal
//         while state.get_result(state.player_just_moved).is_none() {
//             let m = node.data.untried_moves.choose(&mut rand::thread_rng());
//             let m = *m.expect("Move missing!"); // unpack move from Option
//             state.make_move(m);
//             moves_to_root += 1;
//         }
// 
//         // Backpropagate
//         // Backpropagate from the expanded node and work back to the root node
//         while node.parent.is_some() {
//             let game_result = state.get_result(node.data.player_just_moved).
//                                         expect("No game result!");
//             node.update(game_result);
//             let parent_id = node.parent.expect("No parent id!");
//             node = arena.get(&parent_id);
//         }
// 
//         // Undo moves made during this iteration
//         for _i in 0..moves_to_root {
//             state.take_move();
//         }
//     }
// 
//     let rootnode = arena.get(&rootnode_id);
//     for child_id in rootnode.children.iter() {
//         let child = arena.get(child_id);
//         println!("Move {}, Score {}/{} -> {}", child.data.move_.expect("No move!"), child.data.wins, child.data.visits, child.data.wins/child.data.visits);
//     }
//     (0.0, 0.0)
// }
