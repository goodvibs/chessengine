use crate::position::{Piece, Position};

pub struct Move {
    pub piece: Piece,
    pub origin: u64,
    pub dest: u64,
    pub promotion: Option<Piece>
}

pub struct Node {
    pub parent: Option<Box<Node>>,
    pub children: Vec<Node>,
    pub move_: Option<Move>,
    pub ucb_score: f64,
    pub visits: u64,
}

impl Node {
    fn new(move_: Move, parent: Box<Node>) -> Node {
        Node {
            parent: Some(parent),
            children: Vec::new(),
            move_: Some(move_),
            ucb_score: 0.0,
            visits: 0,
        }
    }
}

impl Default for Node {
    fn default() -> Node {
        Node {
            parent: None,
            children: Vec::new(),
            move_: None,
            ucb_score: 0.0,
            visits: 0
        }
    }
}

fn mcts(position: Position, num_iterations: u64, num_rollout: u64) {
    let mut root: Node = Node::default();
    root.children.extend(position.generate_moves().iter().map(|m| Node::new()));
    for i in 0..num_iterations {


    }
}