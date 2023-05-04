#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use std::collections::HashSet;
use rand::{Rng, RngCore};

pub const INITIAL_CHARBOARD: [[char; 8]; 8] = [
    ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
    ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
    ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R']
];

pub const FILE_A: u64 = 0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001;
pub const FILE_B: u64 = 0b00000010_00000010_00000010_00000010_00000010_00000010_00000010_00000010;
pub const FILE_C: u64 = 0b00000100_00000100_00000100_00000100_00000100_00000100_00000100_00000100;
pub const FILE_D: u64 = 0b00001000_00001000_00001000_00001000_00001000_00001000_00001000_00001000;
pub const FILE_E: u64 = 0b00010000_00010000_00010000_00010000_00010000_00010000_00010000_00010000;
pub const FILE_F: u64 = 0b00100000_00100000_00100000_00100000_00100000_00100000_00100000_00100000;
pub const FILE_G: u64 = 0b01000000_01000000_01000000_01000000_01000000_01000000_01000000_01000000;
pub const FILE_H: u64 = 0b10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000;

pub const RANK_1: u64 = 0xFF00_0000_0000_0000;
pub const RANK_2: u64 = 0x00FF_0000_0000_0000;
pub const RANK_3: u64 = 0x0000_FF00_0000_0000;
pub const RANK_4: u64 = 0x0000_00FF_0000_0000;
pub const RANK_5: u64 = 0x0000_0000_FF00_0000;
pub const RANK_6: u64 = 0x0000_0000_00FF_0000;
pub const RANK_7: u64 = 0x0000_0000_0000_FF00;
pub const RANK_8: u64 = 0x0000_0000_0000_00FF;

pub const FILES: [u64; 8] = [
    FILE_A,
    FILE_B,
    FILE_C,
    FILE_D,
    FILE_E,
    FILE_F,
    FILE_G,
    FILE_H
];

pub const RANKS: [u64; 8] = [
    RANK_1,
    RANK_2,
    RANK_3,
    RANK_4,
    RANK_5,
    RANK_6,
    RANK_7,
    RANK_8
];

pub const DIAGONALS: [u64; 15] = [
    0x0000000000000001,
    0x0000000000000102,
    0x0000000000010204,
    0x0000000001020408,
    0x0000000102040810,
    0x0000010204081020,
    0x0001020408102040,
    0x0102040810204080,
    0x0204081020408000,
    0x0408102040800000,
    0x0810204080000000,
    0x1020408000000000,
    0x2040800000000000,
    0x4080000000000000,
    0x8000000000000000
];

pub const ANTIDIAGONALS: [u64; 15] = [
    0x0000000000000080,
    0x0000000000008040,
    0x0000000000804020,
    0x0000000080402010,
    0x0000008040201008,
    0x0000804020100804,
    0x0080402010080402,
    0x8040201008040201,
    0x4020100804020100,
    0x2010080402010000,
    0x1008040201000000,
    0x0804020100000000,
    0x0402010000000000,
    0x0201000000000000,
    0x0100000000000000
];

pub const DARK_SQ: u64 = 0xAA55AA55AA55AA55;
pub const LIGHT_SQ: u64 = !DARK_SQ;
pub const KING_SIDE_B: u64 = 0x0000000000000070;
pub const QUEEN_SIDE_B: u64 = 0x000000000000000E;

pub const CENTER: u64 = 0x0000001818000000;
pub const CENTER_EXTENDED: u64 = 0x00003C3C3C3C0000;
pub const OUTER_EDGES: u64 = 0xFF818181818181FF;
pub const DIAG_A1_H8: u64 = DIAGONALS[7];
pub const DIAG_A8_H1: u64 = ANTIDIAGONALS[7];
pub const RANK_1_2: u64 = RANK_1 | RANK_2;
pub const RANK_7_8: u64 = RANK_7 | RANK_8;
pub const FILE_A_B: u64 = FILE_A | FILE_B;
pub const FILE_G_H: u64 = FILE_G | FILE_H;

pub const CROPPED_FILES: [u64; 8] = [
    FILE_A & !OUTER_EDGES,
    FILE_B & !OUTER_EDGES,
    FILE_C & !OUTER_EDGES,
    FILE_D & !OUTER_EDGES,
    FILE_E & !OUTER_EDGES,
    FILE_F & !OUTER_EDGES,
    FILE_G & !OUTER_EDGES,
    FILE_H & !OUTER_EDGES
];

pub const CROPPED_RANKS: [u64; 8] = [
    RANK_1 & !OUTER_EDGES,
    RANK_2 & !OUTER_EDGES,
    RANK_3 & !OUTER_EDGES,
    RANK_4 & !OUTER_EDGES,
    RANK_5 & !OUTER_EDGES,
    RANK_6 & !OUTER_EDGES,
    RANK_7 & !OUTER_EDGES,
    RANK_8 & !OUTER_EDGES
];

pub const CROPPED_DIAGONALS: [u64; 15] = [
    DIAGONALS[0] & !OUTER_EDGES,
    DIAGONALS[1] & !OUTER_EDGES,
    DIAGONALS[2] & !OUTER_EDGES,
    DIAGONALS[3] & !OUTER_EDGES,
    DIAGONALS[4] & !OUTER_EDGES,
    DIAGONALS[5] & !OUTER_EDGES,
    DIAGONALS[6] & !OUTER_EDGES,
    DIAGONALS[7] & !OUTER_EDGES,
    DIAGONALS[8] & !OUTER_EDGES,
    DIAGONALS[9] & !OUTER_EDGES,
    DIAGONALS[10] & !OUTER_EDGES,
    DIAGONALS[11] & !OUTER_EDGES,
    DIAGONALS[12] & !OUTER_EDGES,
    DIAGONALS[13] & !OUTER_EDGES,
    DIAGONALS[14] & !OUTER_EDGES
];

pub const CROPPED_ANTIDIAGONALS: [u64; 15] = [
    ANTIDIAGONALS[0] & !OUTER_EDGES,
    ANTIDIAGONALS[1] & !OUTER_EDGES,
    ANTIDIAGONALS[2] & !OUTER_EDGES,
    ANTIDIAGONALS[3] & !OUTER_EDGES,
    ANTIDIAGONALS[4] & !OUTER_EDGES,
    ANTIDIAGONALS[5] & !OUTER_EDGES,
    ANTIDIAGONALS[6] & !OUTER_EDGES,
    ANTIDIAGONALS[7] & !OUTER_EDGES,
    ANTIDIAGONALS[8] & !OUTER_EDGES,
    ANTIDIAGONALS[9] & !OUTER_EDGES,
    ANTIDIAGONALS[10] & !OUTER_EDGES,
    ANTIDIAGONALS[11] & !OUTER_EDGES,
    ANTIDIAGONALS[12] & !OUTER_EDGES,
    ANTIDIAGONALS[13] & !OUTER_EDGES,
    ANTIDIAGONALS[14] & !OUTER_EDGES
];

pub struct Board {
    wp: u64,
    wn: u64,
    wb: u64,
    wr: u64,
    wq: u64,
    wk: u64,
    bp: u64,
    bn: u64,
    bb: u64,
    br: u64,
    bq: u64,
    bk: u64,
}

/// converts a charboard (2d array of chars) to a bitboard (u64)
pub fn charboard_to_bitboard(array: [[char; 8]; 8]) -> u64 {
    let mut bitboard: u64 = 0;
    for (i, row) in array.iter().enumerate() {
        for (j, &piece) in row.iter().enumerate() {
            if piece != ' ' {
                bitboard |= 1 << (i * 8 + j);
            }
        }
    }
    bitboard
}

/// converts a bitboard (u64) to a charboard (2d array of chars)
pub fn bitboard_to_charboard(bitboard: u64, piece: char) -> [[char; 8]; 8] {
    let mut array = [[' '; 8]; 8];
    for i in 0..8 {
        for j in 0..8 {
            if bitboard & (1 << (i * 8 + j)) != 0 {
                array[i][j] = piece;
            }
        }
    }
    array
}

/// prints a charboard (2d array of chars) to the console
pub fn print_charboard(array: &[[char; 8]; 8]) {
    for (i, row) in array.iter().enumerate() {
        print!("{} ", 8 - i);
        for &piece in row.iter() {
            if piece == ' ' {
                print!(". ");
                continue;
            }
            else {
                print!("{} ", piece);
            }
        }
        println!();
    }
    println!("  a b c d e f g h");
}

/// converts a charboard (2d array of chars) to a Board object
pub fn boardify(charboard: &[[char; 8]; 8]) -> Board {
    let mut board = Board {
        wp: 0,
        wn: 0,
        wb: 0,
        wr: 0,
        wq: 0,
        wk: 0,
        bp: 0,
        bn: 0,
        bb: 0,
        br: 0,
        bq: 0,
        bk: 0
    };
    for (i, row) in charboard.iter().enumerate() {
        for (j, &piece) in row.iter().enumerate() {
            match piece {
                'P' => board.wp |= 1 << (i * 8 + j),
                'N' => board.wn |= 1 << (i * 8 + j),
                'B' => board.wb |= 1 << (i * 8 + j),
                'R' => board.wr |= 1 << (i * 8 + j),
                'Q' => board.wq |= 1 << (i * 8 + j),
                'K' => board.wk |= 1 << (i * 8 + j),
                'p' => board.bp |= 1 << (i * 8 + j),
                'n' => board.bn |= 1 << (i * 8 + j),
                'b' => board.bb |= 1 << (i * 8 + j),
                'r' => board.br |= 1 << (i * 8 + j),
                'q' => board.bq |= 1 << (i * 8 + j),
                'k' => board.bk |= 1 << (i * 8 + j),
                _ => ()
            }
        }
    }
    board
}

pub fn get_wp_moves(pawns: u64, occupied: u64, captureable: u64) -> u64 {
    (!occupied & (pawns >> 8 & !RANK_1)) | (captureable & (pawns >> 7 & !FILE_A | pawns >> 9 & !FILE_H))
}

pub fn get_bp_moves(pawns: u64, occupied: u64, captureable: u64) -> u64 {
    (!occupied & (pawns << 8 & !RANK_8)) | (captureable & (pawns << 7 & !FILE_H | pawns << 9 & !FILE_A))
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

pub fn get_cropped_orthogonals(origin: u64) -> u64 {
    let mut res: u64 = 0;
    for i in 0..8 {
        if CROPPED_FILES[i] & origin != 0 {
            res |= CROPPED_FILES[i];
        }
        if CROPPED_RANKS[i] & origin != 0 {
            res |= CROPPED_RANKS[i];
        }
    }
    res
}

pub fn get_cropped_diagonals(origin: u64) -> u64 {
    let mut res: u64 = 0;
    for i in 0..15 {
        if CROPPED_DIAGONALS[i] & origin != 0 {
            res |= CROPPED_DIAGONALS[i];
        }
        if CROPPED_ANTIDIAGONALS[i] & origin != 0 {
            res |= CROPPED_ANTIDIAGONALS[i];
        }
    }
    res
}

pub fn get_r_moves(origin: u64, occupancy: u64) -> u64 {
    let mut res: u64 = 0;
    let leading_zeros: u32 = origin.leading_zeros();
    let trailing_zeros: u32 = 64 - leading_zeros - 1;
    let n_distance: u32 = leading_zeros / 8;
    let s_distance: u32 = trailing_zeros / 8;
    let e_distance: u32 = leading_zeros % 8;
    let w_distance: u32 = trailing_zeros % 8;
    let mut is_blocked: bool = false;
    for i in 0..n_distance {
        let pos = origin << (8 * (i+1));
        if occupancy & pos != 0 {
            is_blocked = true;
        }
        if !is_blocked {
            res |= pos;
        }
    }
    is_blocked = false;
    for i in 0..s_distance {
        let pos = origin >> (8 * (i+1));
        if occupancy & pos != 0 {
            is_blocked = true;
        }
        if !is_blocked {
            res |= pos;
        }
    }
    is_blocked = false;
    for i in 0..e_distance {
        let pos = origin << (i+1);
        if occupancy & pos != 0 {
            is_blocked = true;
        }
        if !is_blocked {
            res |= pos;
        }
    }
    is_blocked = false;
    for i in 0..w_distance {
        let pos = origin >> (i+1);
        if occupancy & pos != 0 {
            is_blocked = true;
        }
        if !is_blocked {
            res |= pos;
        }
    }
    res
}

/// converts a Board object to a charboard (2d array of chars)
pub fn charboardify(board: &Board) -> [[char; 8]; 8] {
    let mut charboard = [[' '; 8]; 8];
    for i in 0..8 {
        for j in 0..8 {
            if board.wp & (1 << (i * 8 + j)) != 0 {
                charboard[i][j] = 'P';
            }
            else if board.wn & (1 << (i * 8 + j)) != 0 {
                charboard[i][j] = 'N';
            }
            else if board.wb & (1 << (i * 8 + j)) != 0 {
                charboard[i][j] = 'B';
            }
            else if board.wr & (1 << (i * 8 + j)) != 0 {
                charboard[i][j] = 'R';
            }
            else if board.wq & (1 << (i * 8 + j)) != 0 {
                charboard[i][j] = 'Q';
            }
            else if board.wk & (1 << (i * 8 + j)) != 0 {
                charboard[i][j] = 'K';
            }
            else if board.bp & (1 << (i * 8 + j)) != 0 {
                charboard[i][j] = 'p';
            }
            else if board.bn & (1 << (i * 8 + j)) != 0 {
                charboard[i][j] = 'n';
            }
            else if board.bb & (1 << (i * 8 + j)) != 0 {
                charboard[i][j] = 'b';
            }
            else if board.br & (1 << (i * 8 + j)) != 0 {
                charboard[i][j] = 'r';
            }
            else if board.bq & (1 << (i * 8 + j)) != 0 {
                charboard[i][j] = 'q';
            }
            else if board.bk & (1 << (i * 8 + j)) != 0 {
                charboard[i][j] = 'k';
            }
        }
    }
    charboard
}

pub fn overlay_charboards(charboard1: &[[char; 8]; 8], charboard2: &[[char; 8]; 8], char1: char, char2: char) -> [[char; 8]; 8] {
    let mut res = [[' '; 8]; 8];
    for i in 0..8 {
        for j in 0..8 {
            if charboard1[i][j] != ' ' {
                res[i][j] = char1;
            }
            else if charboard2[i][j] != ' ' {
                res[i][j] = char2;
            }
        }
    }
    res
}

pub fn print_bitboard(bitboard: u64) {
    print_charboard(&bitboard_to_charboard(bitboard, 'x'));
}

pub fn print_board(board: &Board) {
    print_charboard(&charboardify(board));
}

