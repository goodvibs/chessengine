#![allow(dead_code)]
#![allow(unused_variables)]

mod board;
mod magic;
mod board_state;
mod masks;
mod moves;

use magic::*;
use board::*;
use masks::*;
// use moves::*;
// use board_state::*;

fn main() {
    let board: Board = Board::default();
    board.print_pretty();
}

fn test_magics() {
    let rook_dict = MagicDict::new_rook_dict();
    println!("Rook dict created");
    let bishop_dict = MagicDict::new_bishop_dict();
    println!("Bishop dict created");
    let origin_cb = [
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', 'x', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ']
    ];
    let occupancy_cb = [
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', 'x', ' ', ' ', ' ', 'x', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', 'x', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', 'x', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ']
    ];
    let origin: u64 = charboard_to_bitboard(origin_cb);
    println!("{}", origin);
    let occupancy: u64 = charboard_to_bitboard(occupancy_cb);

    let rook_moves = rook_dict.get_moves(origin, occupancy & !origin & get_orthogonals(origin));
    print_bitboard(rook_moves);
    println!("{}", rook_moves);

    let bishop_moves = bishop_dict.get_moves(origin, occupancy & !origin & get_diagonals(origin));
    print_bitboard(bishop_moves);
    println!("{}", bishop_moves);
}