#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use std::collections::HashSet;
use rand::{Rng, RngCore};
use std::fs::File;
use crate::utils;

const ROOK_MAGIC_FILE: &str = "rook_magic";
const BISHOP_MAGIC_FILE: &str = "bishop_magic";

#[derive(Clone)]
pub struct Magic {
    pub magic_number: u64,
    pub mask: u64,
    pub attack_masks: Vec<u64>,
    pub shift: u32
}

// pub fn init_rook_magic() {
//     let mut rook_file = File::create(ROOK_MAGIC_FILE).unwrap();
//     let rook_occupancies: Vec<u64> = generate_occupancies(utils::get_cropped_orthogonals);
//     let rook_magic_numbers: Vec<u64> = generate_magic_numbers(rook_occupancies);
// }

pub fn generate_rook_magic() -> Vec<Magic> {
    let mut rook_magic: Vec<Magic> = vec![
        Magic {
            magic_number: 0,
            mask: 0,
            attack_masks: Vec::new(),
            shift: 0
        };
        64
    ];
    let occupancies: Vec<Vec<u64>> = generate_occupancies(utils::get_orthogonals);
    let attack_masks: Vec<Vec<u64>> = generate_attack_masks(&occupancies, utils::get_r_moves);
    let mut rng = rand::thread_rng();
    let mut pos: u64 = 1; // start with bitboard with only the first bit set
    for i in 0..64usize {
        for blocking_masks in &occupancies {
            let mut attack_mask_indices_for_pos: Vec<usize> = Vec::new();
            for blocking_mask in blocking_masks {
                let attack_mask: u64 = utils::get_r_moves(pos, *blocking_mask);
                let attack_mask_index: usize = attack_masks[i].binary_search(&attack_mask).unwrap();
                attack_mask_indices_for_pos.push(attack_mask_index);
            }
            let mut magic_number: u64;
            loop {
                magic_number = rng.gen::<u64>() & rng.gen::<u64>() & rng.gen::<u64>();
                let mut fail: bool = false;
                for j in 0..blocking_masks.len() {
                    let attack_mask_index: usize = attack_mask_indices_for_pos[j];
                    let shift: u32 = 0;
                    if (magic_number * blocking_masks[j] >> shift) as usize != attack_mask_index {
                        fail = true;
                        break;
                    }
                }
                if !fail {
                    break;
                }
            }
            rook_magic[i].magic_number = magic_number;
            //rook_magic[i].mask = blocking_masks[0];
            rook_magic[i].attack_masks = attack_masks[i].clone();
            //rook_magic[i].shift = 64 - blocking_masks[0].count_ones();
        }
        pos <<= 1; // shift bitboard left by 1
    }
    rook_magic
}

pub fn find_magic_number(blocking_boards: &Vec<u64>, attack_mask_indices: &Vec<usize>) -> u64 {
    assert_eq!(blocking_boards.len(), attack_mask_indices.len());
    let mut rng = rand::thread_rng();
    let mut magic_number: u64;
    loop {
        magic_number = rng.gen::<u64>() & rng.gen::<u64>() & rng.gen::<u64>();
        let mut fail: bool = false;
        for i in 0..blocking_boards.len() {
            let shift: u32 = 0;
            if (magic_number * blocking_boards[i] >> shift) as usize != attack_mask_indices[i] {
                fail = true;
                break;
            }
        }
        if !fail {
            break;
        }
    }
    magic_number
}

/// Generates a vector of vectors of all the possible attack masks for each square on the board.
/// 64 squares, so 64 subvectors. All attack masks in a subector are unique and sorted.
pub fn generate_attack_masks(occupancies: &Vec<Vec<u64>>, get_attack_mask: fn(u64, u64) -> u64) -> Vec<Vec<u64>> {
    let mut attack_masks: Vec<Vec<u64>> = vec![Vec::new(); 64];
    let mut pos: u64 = 1; // start with bitboard with only the first bit set
    for i in 0..64usize {
        for blocking_mask in occupancies[i].iter() {
            let attack_mask: u64 = get_attack_mask(pos, *blocking_mask);
            attack_masks[i].push(attack_mask);
        }
        attack_masks[i].sort();
        attack_masks[i].dedup();
        pos <<= 1; // shift bitboard left by 1
    }
    attack_masks
}

/// Generates a vector of vectors of all the possible occupancies for each square on the board.
/// 64 squares, so 64 subvectors. All occupancies in a subvector are unique.
pub fn generate_occupancies(get_blocking_mask: fn(u64) -> u64) -> Vec<Vec<u64>> {
    let mut occupancies: Vec<Vec<u64>> = vec![Vec::new(); 64];
    let mut pos: u64 = 1; // start with bitboard with only the first bit set
    for i in 0..64usize {
        let blocking_mask: u64 = get_blocking_mask(pos) & !pos;
        let vec: Vec<u64> = fixed_zeros_perms(blocking_mask);
        occupancies[i] = vec;
        pos <<= 1; // shift bitboard left by 1
    }
    occupancies
}

/// Returns a vector of all the possible permutations of the bits of x where the current 0 bits of x
/// remain in place. For example, if x = 0b101, then the vector returned will be [0b000, 0b100, 0b001, 0b101].
pub fn fixed_zeros_perms(x: u64) -> Vec<u64> {
    let mut res = Vec::new(); // create vector
    let pos_bit_count = x.count_ones(); // count number of ones in x
    let perms: u64 = 2u64.pow(pos_bit_count); // calculate number of permutations
    for i in 0..perms {
        let mut x_bit_index: u32 = 0;
        let mut pos_bit_perm: u64 = 0;
        for j in 0..64 {
            if (1u64 << j) & x != 0 { // if jth bit of x is 1
                // set jth bit of pos_bit_perm to the x_bit_indexth bit of i
                pos_bit_perm = set_bit(j, x_bit_index, pos_bit_perm, i);
                x_bit_index += 1;
            }
        }
        res.push(pos_bit_perm); // add permutation to vector
    }
    res
}

/// Sets the nth bit of target to the kth bit of source
fn set_bit(n: u64, k: u32, mut target: u64, source: u64) -> u64 {
    let bit = (source >> k) & 1; // extract kth bit from source
    target |= bit << n; // set nth bit in target to the extracted bit
    target // return the updated target
}