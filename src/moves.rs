use crate::masks::*;
use std::cmp;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref KNIGHT_TABLE: [u64; 64] = create_knight_table();
    pub static ref KING_TABLE: [u64; 64] = create_king_table();
}

fn create_knight_table() -> [u64; 64] {
    let mut table: [u64; 64] = [0; 64];
    let mut pos: u64 = 1;
    for i in 0..64 {
        table[i] = get_n_moves(pos, 0);
        pos <<= 1;
    }
    table
}

fn create_king_table() -> [u64; 64] {
    let mut table: [u64; 64] = [0; 64];
    let mut pos: u64 = 1;
    for i in 0..64 {
        table[i] = get_k_moves(pos, 0);
        pos <<= 1;
    }
    table
}

pub fn get_n_moves_precomp(origin: u64, occupied: u64) -> u64 {
    KNIGHT_TABLE[origin.trailing_zeros() as usize] & !occupied
}

pub fn get_k_moves_precomp(origin: u64, occupied: u64) -> u64 {
    KING_TABLE[origin.trailing_zeros() as usize] & !occupied
}

pub fn get_wp_captures(pawns: u64, capturable: u64) -> u64 {
    capturable & (pawns >> 7 & !FILE_A | pawns >> 9 & !FILE_H)
}

pub fn get_bp_captures(pawns: u64, capturable: u64) -> u64 {
    capturable & (pawns << 7 & !FILE_H | pawns << 9 & !FILE_A)
}

pub fn get_wp_moves(pawns: u64, occupied: u64) -> u64 {
    let spp: u64 = !occupied & (pawns >> 8);
    if spp & RANK_3 == 0 {
        spp
    }
    else {
        spp | get_wp_moves(spp, occupied)
    }
}

pub fn get_bp_moves(pawns: u64, occupied: u64) -> u64 {
    let spp: u64 = !occupied & (pawns << 8);
    if spp & RANK_6 == 0 {
        spp
    }
    else {
        spp | get_bp_moves(spp, occupied)
    }
}

pub fn get_n_moves(knights: u64, occupied: u64) -> u64 {
    !occupied & ((knights >> 15 & !FILE_A) |
        (knights >> 17 & !FILE_H) |
        (knights >> 6 & !FILE_A_B) |
        (knights >> 10 & !FILE_G_H) |
        (knights << 15 & !FILE_H) |
        (knights << 17 & !FILE_A) |
        (knights << 6 & !FILE_G_H) |
        (knights << 10 & !FILE_A_B))
}

pub fn get_k_moves(kings: u64, occupied: u64) -> u64 {
    !occupied & ((kings >> 1 & !FILE_H) |
        (kings >> 7 & !FILE_A) |
        kings >> 8 |
        (kings >> 9 & !FILE_H) |
        (kings << 1 & !FILE_A) |
        (kings << 9 & !FILE_A) |
        kings << 8 |
        (kings << 7 & !FILE_H))
}

pub fn get_r_moves(origin: u64, occupied: u64) -> u64 {
    let mut res: u64 = 0;
    let leading_zeros: u32 = origin.leading_zeros();
    let trailing_zeros: u32 = 64 - leading_zeros - 1;
    let n_distance: u32 = trailing_zeros / 8;
    let s_distance: u32 = leading_zeros / 8;
    let e_distance: u32 = leading_zeros % 8;
    let w_distance: u32 = trailing_zeros % 8;
    let mut is_blocked: bool = false;
    for i in 0..n_distance {
        let pos = origin >> (8 * (i+1));
        if occupied & pos != 0 {
            is_blocked = true;
        }
        if !is_blocked {
            res |= pos;
        }
    }
    is_blocked = false;
    for i in 0..s_distance {
        let pos = origin << (8 * (i+1));
        if occupied & pos != 0 {
            is_blocked = true;
        }
        if !is_blocked {
            res |= pos;
        }
    }
    is_blocked = false;
    for i in 0..e_distance {
        let pos = origin << (i+1);
        if occupied & pos != 0 {
            is_blocked = true;
        }
        if !is_blocked {
            res |= pos;
        }
    }
    is_blocked = false;
    for i in 0..w_distance {
        let pos = origin >> (i+1);
        if occupied & pos != 0 {
            is_blocked = true;
        }
        if !is_blocked {
            res |= pos;
        }
    }
    res
}

pub fn get_b_moves(origin: u64, occupied: u64) -> u64 {
    let mut res: u64 = 0;
    let leading_zeros: u32 = origin.leading_zeros();
    let trailing_zeros: u32 = 64 - leading_zeros - 1;
    let n_distance: u32 = trailing_zeros / 8;
    let s_distance: u32 = leading_zeros / 8;
    let e_distance: u32 = leading_zeros % 8;
    let w_distance: u32 = trailing_zeros % 8;
    let mut is_blocked: bool = false;
    for i in 0..cmp::min(n_distance, e_distance) {
        let pos = origin >> (7 * (i+1));
        if occupied & pos != 0 {
            is_blocked = true;
        }
        if !is_blocked {
            res |= pos;
        }
    }
    is_blocked = false;
    for i in 0..cmp::min(s_distance, w_distance) {
        let pos = origin << (7 * (i+1));
        if occupied & pos != 0 {
            is_blocked = true;
        }
        if !is_blocked {
            res |= pos;
        }
    }
    is_blocked = false;
    for i in 0..cmp::min(s_distance, e_distance) {
        let pos = origin << (9 * (i+1));
        if occupied & pos != 0 {
            is_blocked = true;
        }
        if !is_blocked {
            res |= pos;
        }
    }
    is_blocked = false;
    for i in 0..cmp::min(n_distance, w_distance) {
        let pos = origin >> (9 * (i+1));
        if occupied & pos != 0 {
            is_blocked = true;
        }
        if !is_blocked {
            res |= pos;
        }
    }
    res
}