use crate::const_masks::*;
use std::cmp;
use lazy_static::lazy_static;
use crate::position::Piece;
use crate::magic;

lazy_static! {
    pub static ref KNIGHT_TABLE: [u64; 64] = create_knight_table();
    pub static ref KING_TABLE: [u64; 64] = create_king_table();
}

fn create_knight_table() -> [u64; 64] {
    let mut table: [u64; 64] = [0; 64];
    let mut pos: u64 = 1;
    for i in 0..64 {
        table[i] = n_moves_mask(pos, 0);
        pos <<= 1;
    }
    table
}

fn create_king_table() -> [u64; 64] {
    let mut table: [u64; 64] = [0; 64];
    let mut pos: u64 = 1;
    for i in 0..64 {
        table[i] = k_moves_mask(pos, 0);
        pos <<= 1;
    }
    table
}

pub fn n_moves_mask_precomp(origin: u64, occupied: u64) -> u64 {
    KNIGHT_TABLE[origin.trailing_zeros() as usize] & !occupied
}

pub fn k_moves_mask_precomp(origin: u64, occupied: u64) -> u64 {
    KING_TABLE[origin.trailing_zeros() as usize] & !occupied
}

pub fn wp_captures_mask(pawns: u64, capturable: u64) -> u64 {
    capturable & (pawns >> 7 & !FILE_A | pawns >> 9 & !FILE_H)
}

pub fn bp_captures_mask(pawns: u64, capturable: u64) -> u64 {
    capturable & (pawns << 7 & !FILE_H | pawns << 9 & !FILE_A)
}

pub fn wp_moves_mask(pawns: u64, occupied: u64) -> u64 {
    let spp: u64 = !occupied & (pawns >> 8);
    if spp & RANK_3 == 0 {
        spp
    }
    else {
        spp | wp_moves_mask(spp, occupied)
    }
}

pub fn bp_moves_mask(pawns: u64, occupied: u64) -> u64 {
    let spp: u64 = !occupied & (pawns << 8);
    if spp & RANK_6 == 0 {
        spp
    }
    else {
        spp | bp_moves_mask(spp, occupied)
    }
}

pub fn n_moves_mask(knights: u64, occupied: u64) -> u64 {
    !occupied & ((knights >> 15 & !FILE_A) |
        (knights >> 17 & !FILE_H) |
        (knights >> 6 & !FILE_A_B) |
        (knights >> 10 & !FILE_G_H) |
        (knights << 15 & !FILE_H) |
        (knights << 17 & !FILE_A) |
        (knights << 6 & !FILE_G_H) |
        (knights << 10 & !FILE_A_B))
}

pub fn k_moves_mask(kings: u64, occupied: u64) -> u64 {
    !occupied & ((kings >> 1 & !FILE_H) |
        (kings >> 7 & !FILE_A) |
        kings >> 8 |
        (kings >> 9 & !FILE_H) |
        (kings << 1 & !FILE_A) |
        (kings << 9 & !FILE_A) |
        kings << 8 |
        (kings << 7 & !FILE_H))
}

pub fn r_moves_mask_magic(origin: u64, occupied: u64) -> u64 {
    magic::ROOK_MAGIC_DICT.get_moves(origin, occupied & get_orthogonals(origin))
}

pub fn r_moves_mask(origin: u64, occupied: u64) -> u64 {
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

pub fn b_moves_mask_magic(origin: u64, occupied: u64) -> u64 {
    magic::BISHOP_MAGIC_DICT.get_moves(origin, occupied & get_diagonals(origin))
}

pub fn b_moves_mask(origin: u64, occupied: u64) -> u64 {
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

pub fn q_moves_mask_magic(origin: u64, occupied: u64) -> u64 {
    r_moves_mask_magic(origin, occupied) | b_moves_mask_magic(origin, occupied)
}

pub fn get_orthogonals(origin: u64) -> u64 {
    let mut res: u64 = 0;
    for i in 0..8 {
        if FILES[i] & origin != 0 {
            res |= FILES[i];
        }
        if RANKS[i] & origin != 0 {
            res |= RANKS[i];
        }
    }
    res
}

pub fn get_cropped_orthogonals(origin: u64) -> u64 {
    let mut res: u64 = 0;
    for i in 0..8 {
        if FILES[i] & origin != 0 {
            res |= CROPPED_FILES[i];
        }
        if RANKS[i] & origin != 0 {
            res |= CROPPED_RANKS[i];
        }
    }
    res
}

pub fn get_diagonals(origin: u64) -> u64 {
    let mut res: u64 = 0;
    for i in 0..15 {
        if DIAGONALS[i] & origin != 0 {
            res |= DIAGONALS[i];
        }
        if ANTIDIAGONALS[i] & origin != 0 {
            res |= ANTIDIAGONALS[i];
        }
    }
    res
}

// pub fn get_cropped_diagonals(origin: u64) -> u64 {
//     let mut res: u64 = 0;
//     for i in 0..15 {
//         if CROPPED_DIAGONALS[i] & origin != 0 {
//             res |= CROPPED_DIAGONALS[i];
//         }
//         if CROPPED_ANTIDIAGONALS[i] & origin != 0 {
//             res |= CROPPED_ANTIDIAGONALS[i];
//         }
//     }
//     res
// }