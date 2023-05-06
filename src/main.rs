#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod board;
mod magic;

use magic::*;
use board::*;

fn main() {
    let rook_dict = create_r_move_dict();
    println!("Rook dict created");
    let bishop_dict = create_b_move_dict();
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
        [' ', ' ', ' ', ' ', ' ', ' ', 'x', ' '],
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