use crate::moves_masks::*;
use crate::board::Board;
use crate::magic::{BISHOP_MAGIC_DICT, ROOK_MAGIC_DICT};
use crate::board::unpack_bb;
use crate::mcts::Move;
use crate::const_masks::*;

pub struct Position {
    pub board: Board,
    pub player: Color,
    pub context: Context,
}

impl Position {

    pub fn w_pawn_moves(&mut self) -> Vec<Move> {
        let mut res: Vec<Move> = Vec::new();
        let blockers: u64 = self.board.get_all();
        let capturable: u64 = self.board.get_b_subjects();
        self.context.checked |= wp_captures_mask(self.board.wp, !0);
        for origin in unpack_bb(self.board.wp) {
            for dest in unpack_bb(wp_moves_mask(origin, blockers) | wp_captures_mask(origin, capturable)) {
                if dest & RANK_8 == 0 {
                    res.push(Move {
                        piece: Piece::Pawn,
                        origin: origin,
                        dest: dest,
                        promotion: None
                    });
                }
                else {
                    res.push(Move {
                        piece: Piece::Pawn,
                        origin: origin,
                        dest: dest,
                        promotion: Some(Piece::Queen)
                    });
                    res.push(Move {
                        piece: Piece::Pawn,
                        origin: origin,
                        dest: dest,
                        promotion: Some(Piece::Rook)
                    });
                    res.push(Move {
                        piece: Piece::Pawn,
                        origin: origin,
                        dest: dest,
                        promotion: Some(Piece::Bishop)
                    });
                    res.push(Move {
                        piece: Piece::Pawn,
                        origin: origin,
                        dest: dest,
                        promotion: Some(Piece::Knight)
                    });
                }
            }
        }
        res
    }

    pub fn b_pawn_moves(&mut self) -> Vec<Move> {
        let mut res: Vec<Move> = Vec::new();
        let blockers: u64 = self.board.get_all();
        let capturable: u64 = self.board.get_w_subjects();
        self.context.checked |= bp_captures_mask(self.board.bp, !0);
        for origin in unpack_bb(self.board.bp) {
            for dest in unpack_bb(bp_moves_mask(origin, blockers) | bp_captures_mask(origin, capturable)) {
                if dest & RANK_1 == 0 {
                    res.push(Move {
                        piece: Piece::Pawn,
                        origin: origin,
                        dest: dest,
                        promotion: None
                    });
                }
                else {
                    res.push(Move {
                        piece: Piece::Pawn,
                        origin: origin,
                        dest: dest,
                        promotion: Some(Piece::Queen)
                    });
                    res.push(Move {
                        piece: Piece::Pawn,
                        origin: origin,
                        dest: dest,
                        promotion: Some(Piece::Rook)
                    });
                    res.push(Move {
                        piece: Piece::Pawn,
                        origin: origin,
                        dest: dest,
                        promotion: Some(Piece::Bishop)
                    });
                    res.push(Move {
                        piece: Piece::Pawn,
                        origin: origin,
                        dest: dest,
                        promotion: Some(Piece::Knight)
                    });
                }
            }
        }
        res
    }

    pub fn w_knight_moves(&mut self) -> Vec<Move> {
        let mut res: Vec<Move> = Vec::new();
        let noncapturable: u64 = self.board.get_w_noncapturable();
        for origin in unpack_bb(self.board.wn) {
            for dest in unpack_bb(n_moves_mask_precomp(origin, noncapturable) | (self.board.wn ^ origin)) {
                self.context.checked |= dest;
                res.push(Move {
                    piece: Piece::Knight,
                    origin: origin,
                    dest: dest,
                    promotion: None
                });
            }
        }
        res
    }

    pub fn b_knight_moves(&mut self) -> Vec<Move> {
        let mut res: Vec<Move> = Vec::new();
        let noncapturable: u64 = self.board.get_b_noncapturable();
        for origin in unpack_bb(self.board.bn) {
            for dest in unpack_bb(n_moves_mask_precomp(origin, noncapturable)) {
                self.context.checked |= dest;
                res.push(Move {
                    piece: Piece::Knight,
                    origin: origin,
                    dest: dest,
                    promotion: None
                });
            }
        }
        res
    }

    pub fn w_bishop_moves(&mut self) -> Vec<Move> {
        let mut res: Vec<Move> = Vec::new();
        let noncapturable: u64 = self.board.get_w_noncapturable();
        for origin in unpack_bb(self.board.wb) {
            for dest in unpack_bb(b_moves_mask_magic(origin, noncapturable)) {
                self.context.checked |= dest;
                res.push(Move {
                    piece: Piece::Bishop,
                    origin: origin,
                    dest: dest,
                    promotion: None
                });
            }
        }
        res
    }

    pub fn b_bishop_moves(&mut self) -> Vec<Move> {
        let mut res: Vec<Move> = Vec::new();
        let noncapturable: u64 = self.board.get_b_noncapturable();
        for origin in unpack_bb(self.board.bb) {
            for dest in unpack_bb(b_moves_mask_magic(origin, noncapturable)) {
                self.context.checked |= dest;
                res.push(Move {
                    piece: Piece::Bishop,
                    origin: origin,
                    dest: dest,
                    promotion: None
                });
            }
        }
        res
    }

    pub fn w_rook_moves(&mut self) -> Vec<Move> {
        let mut res: Vec<Move> = Vec::new();
        let noncapturable: u64 = self.board.get_w_noncapturable();
        for origin in unpack_bb(self.board.wr) {
            for dest in unpack_bb(r_moves_mask_magic(origin, noncapturable)) {
                self.context.checked |= dest;
                res.push(Move {
                    piece: Piece::Rook,
                    origin: origin,
                    dest: dest,
                    promotion: None
                });
            }
        }
        res
    }

    pub fn b_rook_moves(&mut self) -> Vec<Move> {
        let mut res: Vec<Move> = Vec::new();
        let noncapturable: u64 = self.board.get_b_noncapturable();
        for origin in unpack_bb(self.board.br) {
            for dest in unpack_bb(r_moves_mask_magic(origin, noncapturable)) {
                self.context.checked |= dest;
                res.push(Move {
                    piece: Piece::Rook,
                    origin: origin,
                    dest: dest,
                    promotion: None
                });
            }
        }
        res
    }

    pub fn w_queen_moves(&mut self) -> Vec<Move> {
        let mut res: Vec<Move> = Vec::new();
        let noncapturable: u64 = self.board.get_w_noncapturable();
        for origin in unpack_bb(self.board.wq) {
            for dest in unpack_bb(q_moves_mask_magic(origin, noncapturable)) {
                self.context.checked |= dest;
                res.push(Move {
                    piece: Piece::Queen,
                    origin: origin,
                    dest: dest,
                    promotion: None
                });
            }
        }
        res
    }

    pub fn b_queen_moves(&mut self) -> Vec<Move> {
        let mut res: Vec<Move> = Vec::new();
        let noncapturable: u64 = self.board.get_b_noncapturable();
        for origin in unpack_bb(self.board.bq) {
            for dest in unpack_bb(q_moves_mask_magic(origin, noncapturable)) {
                self.context.checked |= dest;
                res.push(Move {
                    piece: Piece::Queen,
                    origin: origin,
                    dest: dest,
                    promotion: None
                });
            }
        }
        res
    }

    pub fn w_king_moves(&self) -> Vec<Move> {
        let mut res: Vec<Move> = Vec::new();
        let noncapturable: u64 = self.board.get_w_noncapturable();
        for origin in unpack_bb(self.board.wk) {
            for dest in unpack_bb(k_moves_mask_precomp(origin, noncapturable) & !self.context.checked) {
                res.push(Move {
                    piece: Piece::King,
                    origin: origin,
                    dest: dest,
                    promotion: None
                });
            }
        }
        res
    }

    pub fn b_king_moves(&self) -> Vec<Move> {
        let mut res: Vec<Move> = Vec::new();
        let noncapturable: u64 = self.board.get_b_noncapturable();
        for origin in unpack_bb(self.board.bk) {
            for dest in unpack_bb(k_moves_mask_precomp(origin, noncapturable) | !self.context.checked) {
                res.push(Move {
                    piece: Piece::King,
                    origin: origin,
                    dest: dest,
                    promotion: None
                });
            }
        }
        res
    }

    pub fn w_moves(&mut self) -> Vec<Move> {
        let mut res: Vec<Move> = Vec::new();
        res.append(&mut self.w_pawn_moves());
        res.append(&mut self.w_knight_moves());
        res.append(&mut self.w_bishop_moves());
        res.append(&mut self.w_rook_moves());
        res.append(&mut self.w_queen_moves());
        res.append(&mut self.w_king_moves());
        res
    }

    pub fn b_moves(&mut self) -> Vec<Move> {
        let mut res: Vec<Move> = Vec::new();
        res.append(&mut self.b_pawn_moves());
        res.append(&mut self.b_knight_moves());
        res.append(&mut self.b_bishop_moves());
        res.append(&mut self.b_rook_moves());
        res.append(&mut self.b_queen_moves());
        res.append(&mut self.b_king_moves());
        res
    }

    pub fn moves(&mut self) -> Vec<Move> {
        match self.context.turn {
            Color::White => self.w_moves(),
            Color::Black => self.b_moves()
        }
    }

    pub fn move_(&mut self, move_: Move) {
        match self.context.turn {
            Color::White => {
                match move_.piece {
                    Piece::Pawn => {
                        self.board.wp &= !(move_.origin);
                        if move_.promotion.is_some() {
                            match move_.promotion.unwrap() {
                                Piece::Queen => self.board.wq |= move_.dest,
                                Piece::Rook => self.board.wr |= move_.dest,
                                Piece::Bishop => self.board.wb |= move_.dest,
                                Piece::Knight => self.board.wn |= move_.dest,
                                _ => panic!("Invalid promotion piece")
                            }
                        }
                        else {
                            self.board.wp |= move_.dest;
                            self.board.bp &= !(move_.dest);
                        }
                        self.board.bn &= !(move_.dest);
                        self.board.bb &= !(move_.dest);
                        self.board.br &= !(move_.dest);
                        self.board.bq &= !(move_.dest);
                    },
                    Piece::Knight => {
                        self.board.wn &= !(move_.origin);
                        self.board.wn |= move_.dest;
                        self.board.bp &= !(move_.dest);
                        self.board.bn &= !(move_.dest);
                        self.board.bb &= !(move_.dest);
                        self.board.br &= !(move_.dest);
                        self.board.bq &= !(move_.dest);
                    },
                    Piece::Bishop => {
                        self.board.wb &= !(move_.origin);
                        self.board.wb |= move_.dest;
                        self.board.bp &= !(move_.dest);
                        self.board.bn &= !(move_.dest);
                        self.board.bb &= !(move_.dest);
                        self.board.br &= !(move_.dest);
                        self.board.bq &= !(move_.dest);
                    },
                    Piece::Rook => {
                        self.board.wr &= !(move_.origin);
                        self.board.wr |= move_.dest;
                        self.board.bp &= !(move_.dest);
                        self.board.bn &= !(move_.dest);
                        self.board.bb &= !(move_.dest);
                        self.board.br &= !(move_.dest);
                        self.board.bq &= !(move_.dest);
                    },
                    Piece::Queen => {
                        self.board.wq &= !(move_.origin);
                        self.board.wq |= move_.dest;
                        self.board.bp &= !(move_.dest);
                        self.board.bn &= !(move_.dest);
                        self.board.bb &= !(move_.dest);
                        self.board.br &= !(move_.dest);
                        self.board.bq &= !(move_.dest);
                    },
                    Piece::King => {
                        self.board.wk &= !(move_.origin);
                        self.board.wk |= move_.dest;
                        self.board.bp &= !(move_.dest);
                        self.board.bn &= !(move_.dest);
                        self.board.bb &= !(move_.dest);
                        self.board.br &= !(move_.dest);
                        self.board.bq &= !(move_.dest);
                    }
                }
            },
            Color::Black => {
                match move_.piece {
                    Piece::Pawn => {
                        self.board.bp &= !(move_.origin);
                        if move_.promotion.is_some() {
                            match move_.promotion.unwrap() {
                                Piece::Queen => self.board.bq |= move_.dest,
                                Piece::Rook => self.board.br |= move_.dest,
                                Piece::Bishop => self.board.bb |= move_.dest,
                                Piece::Knight => self.board.bn |= move_.dest,
                                _ => panic!("Invalid promotion piece")
                            }
                        }
                        else {
                            self.board.bp |= move_.dest;
                            self.board.wp &= !(move_.dest);
                        }
                        self.board.wn &= !(move_.dest);
                        self.board.wb &= !(move_.dest);
                        self.board.wr &= !(move_.dest);
                        self.board.wq &= !(move_.dest);
                    },
                    Piece::Knight => {
                        self.board.bn &= !(move_.origin);
                        self.board.bn |= move_.dest;
                        self.board.wp &= !(move_.dest);
                        self.board.wn &= !(move_.dest);
                        self.board.wb &= !(move_.dest);
                        self.board.wr &= !(move_.dest);
                        self.board.wq &= !(move_.dest);
                    },
                    Piece::Bishop => {
                        self.board.bb &= !(move_.origin);
                        self.board.bb |= move_.dest;
                        self.board.wp &= !(move_.dest);
                        self.board.wn &= !(move_.dest);
                        self.board.wb &= !(move_.dest);
                        self.board.wr &= !(move_.dest);
                        self.board.wq &= !(move_.dest);
                    },
                    Piece::Rook => {
                        self.board.br &= !(move_.origin);
                        self.board.br |= move_.dest;
                        self.board.wp &= !(move_.dest);
                        self.board.wn &= !(move_.dest);
                        self.board.wb &= !(move_.dest);
                        self.board.wr &= !(move_.dest);
                        self.board.wq &= !(move_.dest);
                    },
                    Piece::Queen => {
                        self.board.bq &= !(move_.origin);
                        self.board.bq |= move_.dest;
                        self.board.wp &= !(move_.dest);
                        self.board.wn &= !(move_.dest);
                        self.board.wb &= !(move_.dest);
                        self.board.wr &= !(move_.dest);
                        self.board.wq &= !(move_.dest);
                    },
                    Piece::King => {
                        self.board.bk &= !(move_.origin);
                        self.board.bk |= move_.dest;
                        self.board.wp &= !(move_.dest);
                        self.board.wn &= !(move_.dest);
                        self.board.wb &= !(move_.dest);
                        self.board.wr &= !(move_.dest);
                        self.board.wq &= !(move_.dest);
                    }
                }
            }
        }
        self.context.turn = self.context.turn.flip();
        self.context.halfmoves += 1;
    }
}

#[derive(Clone)]
pub struct Context {
    pub wk_castle: bool,
    pub wq_castle: bool,
    pub bk_castle: bool,
    pub bq_castle: bool,
    pub en_passant: u8,
    pub in_check: bool,
    pub checked: u64,
    pub turn: Color,
    pub halfmoves: u8,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            wk_castle: true,
            wq_castle: true,
            bk_castle: true,
            bq_castle: true,
            en_passant: 0,
            in_check: false,
            checked: 0,
            turn: Color::White,
            halfmoves: 0,
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