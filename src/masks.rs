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
pub const RANK_1_8: u64 = RANK_1 | RANK_8;
pub const FILE_A_B: u64 = FILE_A | FILE_B;
pub const FILE_G_H: u64 = FILE_G | FILE_H;
pub const FILE_A_H: u64 = FILE_A | FILE_H;

pub const CROPPED_FILES: [u64; 8] = [
    FILE_A & !RANK_1_8,
    FILE_B & !RANK_1_8,
    FILE_C & !RANK_1_8,
    FILE_D & !RANK_1_8,
    FILE_E & !RANK_1_8,
    FILE_F & !RANK_1_8,
    FILE_G & !RANK_1_8,
    FILE_H & !RANK_1_8
];

pub const CROPPED_RANKS: [u64; 8] = [
    RANK_1 & !FILE_A_H,
    RANK_2 & !FILE_A_H,
    RANK_3 & !FILE_A_H,
    RANK_4 & !FILE_A_H,
    RANK_5 & !FILE_A_H,
    RANK_6 & !FILE_A_H,
    RANK_7 & !FILE_A_H,
    RANK_8 & !FILE_A_H
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
