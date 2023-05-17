pub const EMPTY_CHARBOARD: [[char; 8]; 8] = [
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ']
];

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

#[derive(Clone)]
pub struct Board {
    pub wp: u64,
    pub wn: u64,
    pub wb: u64,
    pub wr: u64,
    pub wq: u64,
    pub wk: u64,
    pub bp: u64,
    pub bn: u64,
    pub bb: u64,
    pub br: u64,
    pub bq: u64,
    pub bk: u64
}

impl Board {
    pub fn get_w_subjects(&self) -> u64 {
        self.wp |
            self.wn |
            self.wb |
            self.wr |
            self.wq
    }

    pub fn get_b_subjects(&self) -> u64 {
        self.bp |
            self.bn |
            self.bb |
            self.br |
            self.bq
    }

    pub fn get_kings(&self) -> u64 {
        self.wk | self.bk
    }

    pub fn get_w_noncapturable(&self) -> u64 {
        self.get_w_subjects() | self.get_kings()
    }

    pub fn get_b_noncapturable(&self) -> u64 {
        self.get_b_subjects() | self.get_kings()
    }

    pub fn get_all(&self) -> u64 {
        self.get_w_subjects() | self.get_b_subjects() | self.get_kings()
    }

    pub fn to_charboard(&self) -> [[char; 8]; 8] {
        charboardify(self)
    }

    pub fn print_pretty(&self) {
        print_charboard(&self.to_charboard());
    }
}

impl Default for Board {
    fn default() -> Self {
        Board {
            wp: 0x00FF000000000000,
            wn: 0x4200000000000000,
            wb: 0x2400000000000000,
            wr: 0x8100000000000000,
            wq: 0x1000000000000000,
            wk: 0x0800000000000000,
            bp: 0x000000000000FF00,
            bn: 0x0000000000000042,
            bb: 0x0000000000000024,
            br: 0x0000000000000081,
            bq: 0x0000000000000010,
            bk: 0x0000000000000008
        }
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

