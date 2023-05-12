use crate::moves::*;
use crate::board::Board;
use crate::magic::{BISHOP_MAGIC_DICT, ROOK_MAGIC_DICT};

pub struct Position {
    pub board: Board,
    pub player: Color,
    pub turn: Color,
    pub context: Context,
    pub halfmove: u8
}

impl Position {
    pub fn w_pawn_moves(&self) -> u64 {
        get_wp_moves(self.board.wp, self.board.get_b_subjects())
    }

    pub fn b_pawn_moves(&self) -> u64 {
        get_bp_moves(self.board.bp, self.board.get_w_subjects())
    }

    pub fn w_pawn_captures(&self) -> u64 {
        get_wp_captures(self.board.wp, self.board.get_b_subjects())
    }

    pub fn b_pawn_captures(&self) -> u64 {
        get_bp_captures(self.board.bp, self.board.get_w_subjects())
    }

    pub fn w_knight_moves(&self) -> u64 {
        get_n_moves_precomp(self.board.wn, self.board.get_w_noncapturable())
    }

    pub fn b_knight_moves(&self) -> u64 {
        get_n_moves_precomp(self.board.bn, self.board.get_b_noncapturable())
    }

    pub fn w_bishop_moves(&self) -> u64 {
        BISHOP_MAGIC_DICT.get_moves(self.board.wb, self.board.get_w_noncapturable())
    }

    pub fn b_bishop_moves(&self) -> u64 {
        BISHOP_MAGIC_DICT.get_moves(self.board.bb, self.board.get_b_noncapturable())
    }

    pub fn w_rook_moves(&self) -> u64 {
        ROOK_MAGIC_DICT.get_moves(self.board.wr, self.board.get_w_noncapturable())
    }

    pub fn b_rook_moves(&self) -> u64 {
        ROOK_MAGIC_DICT.get_moves(self.board.br, self.board.get_b_noncapturable())
    }

    pub fn w_queen_moves(&self) -> u64 {
        self.w_bishop_moves() | self.w_rook_moves()
    }

    pub fn b_queen_moves(&self) -> u64 {
        self.b_bishop_moves() | self.b_rook_moves()
    }

    pub fn w_king_moves(&self) -> u64 {
        get_k_moves(self.board.wk, self.board.get_w_noncapturable())
    }

    pub fn b_king_moves(&self) -> u64 {
        get_k_moves(self.board.bk, self.board.get_b_noncapturable())
    }
}

fn unflatten_bb(mut bb: u64) -> Vec<u64> {
    let mut res: Vec<u64> = Vec::new();
    while bb != 0 {
        let lsb = bb & bb.wrapping_neg();
        res.push(lsb);
        bb ^= lsb;
    }
    res
}

pub struct Context {
    pub wk_castle: bool,
    pub wq_castle: bool,
    pub bk_castle: bool,
    pub bq_castle: bool,
    pub en_passant: u8,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            wk_castle: true,
            wq_castle: true,
            bk_castle: true,
            bq_castle: true,
            en_passant: 0
        }
    }
}

pub enum Color {
    White = 0,
    Black = 1,
}

impl Color {
    pub fn flip(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}