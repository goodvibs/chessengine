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
    // let attack_masks = generate_attack_masks(&occupancies, get_r_moves);
    // let mut attack_mask_indices = Vec::new();
    // for blocking_board in &occupancies[0] {
    //     attack_mask_indices.push(attack_masks[0].binary_search(&get_r_moves(1, *blocking_board)).unwrap());
    // }
    // let magic_number = find_magic_number(&occupancies[0], &attack_mask_indices);
    // println!("{:?}", magic_number);

    let origin_cb = [
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', 'x', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ']
    ];
    let occupancy_cb = [
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', 'x', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', 'x', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', 'x', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', 'x', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', 'x', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', 'x', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', 'x', ' ', ' ']
    ];
    let origin: u64 = charboard_to_bitboard(origin_cb);
    let occupancy: u64 = charboard_to_bitboard(occupancy_cb);
    print_bitboard(get_b_moves(origin, occupancy));
}