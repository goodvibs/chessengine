use rand::Rng;
use std::fs::{File, metadata};
use std::io::{BufWriter, BufReader};
use serde::{Serialize, Deserialize};
use crate::moves_masks;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ROOK_MAGIC_DICT: MagicDict = MagicDict::new_rook_dict();
    pub static ref BISHOP_MAGIC_DICT: MagicDict = MagicDict::new_bishop_dict();
}

const ROOK_MAGIC_FILE: &str = "rook_magics.bin";
const BISHOP_MAGIC_FILE: &str = "bishop_magics.bin";

pub struct MagicDict {
    pub magics: Vec<Magic>
}

impl MagicDict {
    pub fn get_moves(&self, pos: u64, blockers: u64) -> u64 {
        let magic_vec_index: usize = pos.trailing_zeros() as usize;
        let magic: &Magic = &self.magics[magic_vec_index];
        let index = magic_index(magic.magic_number, blockers, magic.shift);
        magic.moves[index]
    }

    pub fn new_rook_dict() -> MagicDict {
        create_r_magic_dict()
    }

    pub fn new_bishop_dict() -> MagicDict {
        create_b_magic_dict()
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Magic {
    pub magic_number: u64,
    pub blocking_mask: u64,
    pub moves: Vec<u64>,
    pub shift: u32
}

fn create_r_magic_dict() -> MagicDict {
    let magics: Vec<Magic>;
    if path_exists(ROOK_MAGIC_FILE) {
        magics = read_magics(ROOK_MAGIC_FILE);
    }
    else {
        magics = generate_magic_vec(moves_masks::get_orthogonals, moves_masks::r_moves_mask);
        write_magics(&magics, ROOK_MAGIC_FILE);
    }
    MagicDict {
        magics
    }
}

fn create_b_magic_dict() -> MagicDict {
    let magics: Vec<Magic>;
    if path_exists(BISHOP_MAGIC_FILE) {
        magics = read_magics(BISHOP_MAGIC_FILE);
    }
    else {
        magics = generate_magic_vec(moves_masks::get_diagonals, moves_masks::b_moves_mask);
        write_magics(&magics, BISHOP_MAGIC_FILE);
    }
    MagicDict {
        magics
    }
}

fn path_exists(path: &str) -> bool {
    metadata(path).is_ok()
}

pub fn read_magics(file_name: &str) -> Vec<Magic> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let magics: Vec<Magic> = bincode::deserialize_from(reader).unwrap();
    magics
}

pub fn write_magics(magics: &Vec<Magic>, file_name: &str) {
    let file = File::create(file_name).unwrap();
    let writer = BufWriter::new(file);
    bincode::serialize_into(writer, &magics).unwrap();
}

pub fn generate_magic_vec(get_blocking_mask: fn(u64) -> u64, get_moves: fn(u64, u64) -> u64) -> Vec<Magic> {
    let mut magics: Vec<Magic> = Vec::new();
    let occupancies: Vec<Vec<u64>> = generate_occupancies(get_blocking_mask);
    let mut pos: u64 = 1; // start with bitboard with only the first bit set
    for i in 0..64usize {
        let blockers: &Vec<u64> = &occupancies[i];
        let magic: Magic = generate_magic(pos, blockers, get_blocking_mask(pos), get_moves);
        magics.push(magic);
        pos <<= 1; // shift bitboard left by 1
    }
    magics
}

pub fn generate_magic(pos: u64, blocking_boards: &Vec<u64>, blocking_mask: u64, get_move_board: fn(u64, u64) -> u64) -> Magic {
    println!("Generating magics for square {}", pos.trailing_zeros());
    let mut rng = rand::thread_rng();
    let index_bits: u32 = blocking_mask.count_ones();
    let shift: u32 = 64 - index_bits;
    //let shift: u32 = 0;
    let mut move_boards: Vec<u64> = vec![0; 1 << index_bits];
    let mut magic_number: u64;
    loop {
        let mut move_boards_temp: Vec<u64> = vec![0; 1 << index_bits];
        magic_number = rng.gen::<u64>() & rng.gen::<u64>() & rng.gen::<u64>();
        let mut fail = false;
        for blockers in blocking_boards {
            let index: usize = magic_index(magic_number, *blockers, shift);
            let moves: u64 = get_move_board(pos, *blockers);
            if move_boards_temp[index] != 0 && move_boards_temp[index] != moves {
                let test = move_boards_temp[index];
                fail = true;
                break;
            }
            move_boards_temp[index] = moves;
        }
        if !fail {
            for blockers in blocking_boards {
                let index: usize = magic_index(magic_number, *blockers, shift);
                let move_board: u64 = get_move_board(pos, *blockers);
                move_boards[index] = get_move_board(pos, *blockers);
            }
            break;
        }
    }
    Magic {
        magic_number: magic_number,
        blocking_mask: blocking_mask,
        moves: move_boards,
        shift: shift
    }
}

fn magic_index(magic_number: u64, blockers: u64, shift: u32) -> usize {
    blockers.wrapping_mul(magic_number).wrapping_shr(shift) as usize
}

/// Generates a vector of vectors of all the possible blocking_boards for each square on the board.
/// 64 squares, so 64 subvectors. All blocking_boards in a subvector are unique.
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
fn fixed_zeros_perms(x: u64) -> Vec<u64> {
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