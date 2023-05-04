#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

mod utils;
mod magic;

use magic::*;
use utils::*;

fn main() {
    // let occupancies = generate_occupancies(get_orthogonals);
    // println!("{}", occupancies.len());
    // for blocking_masks in occupancies {
    //     println!("{}", blocking_masks.len());
    //     print_bitboard(blocking_masks[16383]);
    //     for blocking_mask in blocking_masks {
    //         // print_bitboard(blocking_mask);
    //         // println!();
    //     }
    // }

    let origin_cb = [
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ']
    ];
    let occupancy_cb = [
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', 'x', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ']
    ];
    let origin = charboard_to_bitboard(origin_cb);
    let occupancy = charboard_to_bitboard(occupancy_cb);
    let rook_moves = get_r_moves(origin, occupancy);
    print_bitboard(rook_moves);
}