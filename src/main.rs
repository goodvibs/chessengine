#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod board;
mod magic;
mod position;
mod masks;
mod moves;

use magic::*;
use board::*;
use masks::*;
use position::*;
// use moves::*;

fn main() {
    let pos: Position = Position {
        board: Board::default(),
        player: Color::White,
        turn: Color::Black,
        context: Context::default(),
        halfmove: 1
    };
    for bb in pos.b_knight_moves() {
        bb.board.print_pretty();
    }
    // let origin_cb = [
    //     [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    //     [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    //     [' ', ' ', ' ', 'x', ' ', ' ', ' ', ' '],
    //     [' ', ' ', ' ', ' ', 'x', ' ', ' ', ' '],
    //     [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    //     [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    //     [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    //     [' ', ' ', ' ', ' ', ' ', ' ', ' ', 'x']
    // ];
    // let og: u64 = charboard_to_bitboard(origin_cb);
    // for bb in unpack_bb(og) {
    //     print_bitboard(bb);
    // }
}