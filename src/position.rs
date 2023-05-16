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
    pub fn create_variant(&self, modified: Piece, new_bb: u64) -> Position {
        match self.turn {
            Color::White => {
                match modified {
                    Piece::Pawn => Position {
                        board: Board {
                            wp: new_bb,
                            wn: self.board.wn,
                            wb: self.board.wb,
                            wr: self.board.wr,
                            wq: self.board.wq,
                            wk: self.board.wk,
                            bp: self.board.bp & !new_bb,
                            bn: self.board.bn & !new_bb,
                            bb: self.board.bb & !new_bb,
                            br: self.board.br & !new_bb,
                            bq: self.board.bq & !new_bb,
                            bk: self.board.bk
                        },
                        player: self.player.clone(),
                        turn: Color::Black,
                        context: self.context.clone(),
                        halfmove: self.halfmove + 1
                    },
                    Piece::Knight => Position {
                        board: Board {
                            wp: self.board.wp,
                            wn: new_bb,
                            wb: self.board.wb,
                            wr: self.board.wr,
                            wq: self.board.wq,
                            wk: self.board.wk,
                            bp: self.board.bp & !new_bb,
                            bn: self.board.bn & !new_bb,
                            bb: self.board.bb & !new_bb,
                            br: self.board.br & !new_bb,
                            bq: self.board.bq & !new_bb,
                            bk: self.board.bk
                        },
                        player: self.player.clone(),
                        turn: Color::Black,
                        context: self.context.clone(),
                        halfmove: self.halfmove + 1
                    },
                    Piece::Bishop => Position {
                        board: Board {
                            wp: self.board.wp,
                            wn: self.board.wn,
                            wb: new_bb,
                            wr: self.board.wr,
                            wq: self.board.wq,
                            wk: self.board.wk,
                            bp: self.board.bp & !new_bb,
                            bn: self.board.bn & !new_bb,
                            bb: self.board.bb & !new_bb,
                            br: self.board.br & !new_bb,
                            bq: self.board.bq & !new_bb,
                            bk: self.board.bk
                        },
                        player: self.player.clone(),
                        turn: Color::Black,
                        context: self.context.clone(),
                        halfmove: self.halfmove + 1
                    },
                    Piece::Rook => Position {
                        board: Board {
                            wp: self.board.wp,
                            wn: self.board.wn,
                            wb: self.board.wb,
                            wr: new_bb,
                            wq: self.board.wq,
                            wk: self.board.wk,
                            bp: self.board.bp & !new_bb,
                            bn: self.board.bn & !new_bb,
                            bb: self.board.bb & !new_bb,
                            br: self.board.br & !new_bb,
                            bq: self.board.bq & !new_bb,
                            bk: self.board.bk
                        },
                        player: self.player.clone(),
                        turn: Color::Black,
                        context: self.context.clone(),
                        halfmove: self.halfmove + 1
                    },
                    Piece::Queen => Position {
                        board: Board {
                            wp: self.board.wp,
                            wn: self.board.wn,
                            wb: self.board.wb,
                            wr: self.board.wr,
                            wq: new_bb,
                            wk: self.board.wk,
                            bp: self.board.bp & !new_bb,
                            bn: self.board.bn & !new_bb,
                            bb: self.board.bb & !new_bb,
                            br: self.board.br & !new_bb,
                            bq: self.board.bq & !new_bb,
                            bk: self.board.bk
                        },
                        player: self.player.clone(),
                        turn: Color::Black,
                        context: self.context.clone(),
                        halfmove: self.halfmove + 1
                    },
                    Piece::King => Position {
                        board: Board {
                            wp: self.board.wp,
                            wn: self.board.wn,
                            wb: self.board.wb,
                            wr: self.board.wr,
                            wq: self.board.wq,
                            wk: new_bb,
                            bp: self.board.bp & !new_bb,
                            bn: self.board.bn & !new_bb,
                            bb: self.board.bb & !new_bb,
                            br: self.board.br & !new_bb,
                            bq: self.board.bq & !new_bb,
                            bk: self.board.bk
                        },
                        player: self.player.clone(),
                        turn: Color::Black,
                        context: self.context.clone(),
                        halfmove: self.halfmove + 1
                    }
                }
            }
            Color::Black => {
                match modified {
                    Piece::Pawn => Position {
                        board: Board {
                            wp: self.board.wp & !new_bb,
                            wn: self.board.wn & !new_bb,
                            wb: self.board.wb & !new_bb,
                            wr: self.board.wr & !new_bb,
                            wq: self.board.wq & !new_bb,
                            wk: self.board.wk,
                            bp: new_bb,
                            bn: self.board.bn,
                            bb: self.board.bb,
                            br: self.board.br,
                            bq: self.board.bq,
                            bk: self.board.bk
                        },
                        player: self.player.clone(),
                        turn: Color::White,
                        context: self.context.clone(),
                        halfmove: self.halfmove + 1
                    },
                    Piece::Knight => Position {
                        board: Board {
                            wp: self.board.wp & !new_bb,
                            wn: self.board.wn & !new_bb,
                            wb: self.board.wb & !new_bb,
                            wr: self.board.wr & !new_bb,
                            wq: self.board.wq & !new_bb,
                            wk: self.board.wk,
                            bp: self.board.bp,
                            bn: new_bb,
                            bb: self.board.bb,
                            br: self.board.br,
                            bq: self.board.bq,
                            bk: self.board.bk
                        },
                        player: self.player.clone(),
                        turn: Color::White,
                        context: self.context.clone(),
                        halfmove: self.halfmove + 1
                    },
                    Piece::Bishop => Position {
                        board: Board {
                            wp: self.board.wp & !new_bb,
                            wn: self.board.wn & !new_bb,
                            wb: self.board.wb & !new_bb,
                            wr: self.board.wr & !new_bb,
                            wq: self.board.wq & !new_bb,
                            wk: self.board.wk,
                            bp: self.board.bp,
                            bn: self.board.bn,
                            bb: new_bb,
                            br: self.board.br,
                            bq: self.board.bq,
                            bk: self.board.bk
                        },
                        player: self.player.clone(),
                        turn: Color::White,
                        context: self.context.clone(),
                        halfmove: self.halfmove + 1
                    },
                    Piece::Rook => Position {
                        board: Board {
                            wp: self.board.wp & !new_bb,
                            wn: self.board.wn & !new_bb,
                            wb: self.board.wb & !new_bb,
                            wr: self.board.wr & !new_bb,
                            wq: self.board.wq & !new_bb,
                            wk: self.board.wk,
                            bp: self.board.bp,
                            bn: self.board.bn,
                            bb: self.board.bb,
                            br: new_bb,
                            bq: self.board.bq,
                            bk: self.board.bk
                        },
                        player: self.player.clone(),
                        turn: Color::White,
                        context: self.context.clone(),
                        halfmove: self.halfmove + 1
                    },
                    Piece::Queen => Position {
                        board: Board {
                            wp: self.board.wp & !new_bb,
                            wn: self.board.wn & !new_bb,
                            wb: self.board.wb & !new_bb,
                            wr: self.board.wr & !new_bb,
                            wq: self.board.wq & !new_bb,
                            wk: self.board.wk,
                            bp: self.board.bp,
                            bn: self.board.bn,
                            bb: self.board.bb,
                            br: self.board.br,
                            bq: new_bb,
                            bk: self.board.bk
                        },
                        player: self.player.clone(),
                        turn: Color::White,
                        context: self.context.clone(),
                        halfmove: self.halfmove + 1
                    },
                    Piece::King => Position {
                        board: Board {
                            wp: self.board.wp & !new_bb,
                            wn: self.board.wn & !new_bb,
                            wb: self.board.wb & !new_bb,
                            wr: self.board.wr & !new_bb,
                            wq: self.board.wq & !new_bb,
                            wk: self.board.wk,
                            bp: self.board.bp,
                            bn: self.board.bn,
                            bb: self.board.bb,
                            br: self.board.br,
                            bq: self.board.bq,
                            bk: new_bb
                        },
                        player: self.player.clone(),
                        turn: Color::White,
                        context: self.context.clone(),
                        halfmove: self.halfmove + 1
                    }
                }
            }
        }
    }

    pub fn w_pawn_moves(&self) -> Vec<Position> {
        let mut res: Vec<Position> = Vec::new();
        let blockers: u64 = self.board.get_all();
        let capturable: u64 = self.board.get_b_subjects();
        for pos in unpack_bb(self.board.wp) {
            res.push(
                self.create_variant(
                    Piece::Pawn,
                    get_wp_moves(pos, blockers) |
                        get_wp_captures(pos, capturable) |
                        (self.board.wp ^ pos)
                ));
        }
        res
    }

    pub fn b_pawn_moves(&self) -> Vec<Position> {
        let mut res: Vec<Position> = Vec::new();
        let blockers: u64 = self.board.get_all();
        let capturable: u64 = self.board.get_w_subjects();
        for pos in unpack_bb(self.board.bp) {
            res.push(
                self.create_variant(
                    Piece::Pawn,
                    get_bp_moves(pos, blockers) |
                        get_bp_captures(pos, capturable) |
                        (self.board.bp ^ pos)
                ));
        }
        res
    }

    pub fn w_knight_moves(&self) -> Vec<Position> {
        let mut res: Vec<Position> = Vec::new();
        let noncapturable: u64 = self.board.get_w_noncapturable();
        for pos in unpack_bb(self.board.wn) {
            res.push(
                self.create_variant(
                    Piece::Knight,
                    get_n_moves_precomp(pos, noncapturable) |
                        (self.board.wn ^ pos)
                ));
        }
        res
    }

    pub fn b_knight_moves(&self) -> Vec<Position> {
        let mut res: Vec<Position> = Vec::new();
        let noncapturable: u64 = self.board.get_b_noncapturable();
        for pos in unpack_bb(self.board.bn) {
            res.push(
                self.create_variant(
                    Piece::Knight,
                    get_n_moves_precomp(pos, noncapturable) |
                        (self.board.bn ^ pos)
                ));
        }
        res
    }

    pub fn w_bishop_moves(&self) -> Vec<Position> {
        let mut res: Vec<Position> = Vec::new();
        let noncapturable: u64 = self.board.get_w_noncapturable();
        for pos in unpack_bb(self.board.wb) {
            res.push(
                self.create_variant(
                    Piece::Bishop,
                    BISHOP_MAGIC_DICT.get_moves(pos, noncapturable) |
                        (self.board.wb ^ pos)
                ));
        }
        res
    }

    pub fn b_bishop_moves(&self) -> Vec<Position> {
        let mut res: Vec<Position> = Vec::new();
        let noncapturable: u64 = self.board.get_b_noncapturable();
        for pos in unpack_bb(self.board.bb) {
            res.push(
                self.create_variant(
                    Piece::Bishop,
                    BISHOP_MAGIC_DICT.get_moves(pos, noncapturable) |
                        (self.board.bb ^ pos)
                ));
        }
        res
    }

    pub fn w_rook_moves(&self) -> Vec<Position> {
        let mut res: Vec<Position> = Vec::new();
        let noncapturable: u64 = self.board.get_w_noncapturable();
        for pos in unpack_bb(self.board.wr) {
            res.push(
                self.create_variant(
                    Piece::Rook,
                    ROOK_MAGIC_DICT.get_moves(pos, noncapturable) |
                        (self.board.wr ^ pos)
                ));
        }
        res
    }

    pub fn b_rook_moves(&self) -> Vec<Position> {
        let mut res: Vec<Position> = Vec::new();
        let noncapturable: u64 = self.board.get_b_noncapturable();
        for pos in unpack_bb(self.board.br) {
            res.push(
                self.create_variant(
                    Piece::Rook,
                    ROOK_MAGIC_DICT.get_moves(pos, noncapturable) |
                        (self.board.br ^ pos)
                ));
        }
        res
    }

    pub fn w_queen_moves(&self) -> Vec<Position> {
        let mut res: Vec<Position> = Vec::new();
        let noncapturable: u64 = self.board.get_w_noncapturable();
        for pos in unpack_bb(self.board.wq) {
            res.push(
                self.create_variant(
                    Piece::Queen,
                    BISHOP_MAGIC_DICT.get_moves(pos, noncapturable) |
                        ROOK_MAGIC_DICT.get_moves(pos, noncapturable) |
                        (self.board.wq ^ pos)
                ));
        }
        res
    }

    pub fn b_queen_moves(&self) -> Vec<Position> {
        let mut res: Vec<Position> = Vec::new();
        let noncapturable: u64 = self.board.get_b_noncapturable();
        for pos in unpack_bb(self.board.bq) {
            res.push(
                self.create_variant(
                    Piece::Queen,
                    BISHOP_MAGIC_DICT.get_moves(pos, noncapturable) |
                        ROOK_MAGIC_DICT.get_moves(pos, noncapturable) |
                        (self.board.bq ^ pos)
                ));
        }
        res
    }

    pub fn w_king_moves(&self) -> Vec<Position> {
        let mut res: Vec<Position> = Vec::new();
        let noncapturable: u64 = self.board.get_w_noncapturable();
        for pos in unpack_bb(self.board.wk) {
            res.push(
                self.create_variant(
                    Piece::King,
                    get_k_moves_precomp(pos, noncapturable) |
                        (self.board.wk ^ pos)
                ));
        }
        res
    }

    pub fn b_king_moves(&self) -> Vec<Position> {
        let mut res: Vec<Position> = Vec::new();
        let noncapturable: u64 = self.board.get_b_noncapturable();
        for pos in unpack_bb(self.board.bk) {
            res.push(
                self.create_variant(
                    Piece::King,
                    get_k_moves_precomp(pos, noncapturable)
                ));
        }
        res
    }
}

pub fn unpack_bb(mut bb: u64) -> Vec<u64> {
    let mut res: Vec<u64> = Vec::new();
    while bb != 0 {
        let lsb = bb & bb.wrapping_neg();
        res.push(lsb);
        bb ^= lsb;
    }
    res
}

#[derive(Clone)]
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

#[derive(Clone)]
pub enum Color {
    White,
    Black
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