use std::cmp;
use rand::seq::SliceRandom;
use crate::position::{Piece, Position};

pub struct Move {
    pub piece: Piece,
    pub origin: u64,
    pub dest: u64,
    pub promotion: Option<Piece>
}

pub struct Node<'a> {
    pub parent: Option<&'a Node<'a>>,
    pub children: Vec<&'a Node<'a>>,
    pub move_: Option<&'a Move>,
    pub cum_val: f64,
    pub visits: u64
}

impl<'a> Node<'a> {
    fn new(move_: &Move, parent: &Node) -> Node<'a> {
        Node {
            parent: Some(parent),
            children: Vec::new(),
            move_: Some(move_),
            cum_val: 0.0,
            visits: 0,
        }
    }
}

impl<'a> Default for Node<'a> {
    fn default() -> Node<'a> {
        Node {
            parent: None,
            children: Vec::new(),
            move_: None,
            cum_val: 0.0,
            visits: 0
        }
    }
}

impl<'a> FromIterator<Node<'a>> for &mut Vec<&Node<'a>> {
    fn from_iter<I: IntoIterator<Item=Node<'a>>>(iter: I) -> Self {
        let mut v = Vec::new();
        for i in iter {
            v.push(&i);
        }
        &mut v
    }
}

fn mcts(mut position: Position, num_iterations: u64, num_rollout: u64) {
    let mut root: Node = Node::default();
    root.children.append(position.moves().iter().map(|m| Node::new(m, &root)).collect());
    for i in 0..num_iterations {
        let leaf: &mut Node = select(&mut root, &mut position);
        let expanded: &mut Node = expand(leaf, &mut position);
        let reward: f64 = rollout(expanded, num_rollout, &mut position);
        backprop(expanded, reward);
    }
}

fn select<'a>(root: &'a mut Node<'a>, pos: &mut Position) -> &'a mut Node<'a> {
    let mut selected_node = root;
    while !selected_node.children.is_empty() {
        let child = selected_node
            .children
            .iter_mut()
            .max_by(|a, b| calc_ucb_score(*a).partial_cmp(&calc_ucb_score(*b)).unwrap())
            .unwrap();
        selected_node = child;
        pos.make_move(selected_node.move_.unwrap());
    }
    selected_node
}

fn expand<'a>(leaf: &'a mut Node<'a>, pos: &mut Position) -> &'a mut Node<'a> {
    let moves: Vec<Move> = pos.moves();
    leaf.children.append(moves.iter().map(|m| Node::new(m, leaf)).collect());
    let expanded: &mut Node = &mut leaf.children.choose(&mut rand::thread_rng()).unwrap();
    pos.make_move(expanded.move_.unwrap());
    expanded
}

fn rollout(expanded: &mut Node, num_rollout: u64, pos: &mut Position) -> f64 {
    let mut reward: f64 = 0.0;
    for _ in 0..num_rollout {
        reward += simulate(expanded);
    }
    reward / num_rollout as f64
}

const fn calc_ucb_score(node: &Node) -> f64 {
    let exploitation_score = node.cum_val / node.visits as f64;
    let exploration_score = 2.0 * ((node.parent.unwrap().visits as f64).ln() / node.visits as f64).sqrt();
    exploitation_score + exploration_score
}