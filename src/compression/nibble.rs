use::cozy_chess::{Color, Piece};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Nibble {
    WhitePawn,
    BlackPawn,
    WhiteKnight,
    BlackKnight,
    WhiteBishop,
    BlackBishop,
    WhiteRook,
    BlackRook,
    WhiteQueen,
    BlackQueen,
    WhiteKing,
    BlackKing,
    EnPassantPawn,
    WhiteRookCastle,
    BlackRookCastle,
    BlackMoveKing,
}

// impl From<usize> for Nibble {
//     fn from(value: usize) -> Self {
//         match value {
//             0 => Nibble::WhitePawn,
//             1 => Nibble::BlackPawn,
//             2 => Nibble::WhiteKnight,
//             3 => Nibble::BlackKnight,
//             4 => Nibble::WhiteBishop,
//             5 => Nibble::BlackBishop,
//             6 => Nibble::WhiteRook,
//             7 => Nibble::BlackRook,
//             8 => Nibble::WhiteQueen,
//             9 => Nibble::BlackQueen,
//             10 => Nibble::WhiteKing,
//             11 => Nibble::BlackKing,
//             12 => Nibble::EnPassantPawn,
//             13 => Nibble::WhiteRookCastle,
//             14 => Nibble::BlackRookCastle,
//             15 => Nibble::BlackMoveKing,
//             _ => panic!("Invalid Nibble"),
//         }
//     }
// }





// impl From<Nibble> for usize {
//     fn from(nibble: Nibble) -> Self {
//         nibble as usize
//     }
// }

impl Nibble {
    pub fn new(piece: Piece, color: Color, stm: Color) -> Nibble {
        let mut idx = 2*(piece as usize) + color as usize;
        if piece == Piece::King && stm == Color::Black {
            idx = 15;
        }
        Nibble::index(idx)
    }

    pub fn index(value: usize) -> Nibble {
        match value {
            0 => Nibble::WhitePawn,
            1 => Nibble::BlackPawn,
            2 => Nibble::WhiteKnight,
            3 => Nibble::BlackKnight,
            4 => Nibble::WhiteBishop,
            5 => Nibble::BlackBishop,
            6 => Nibble::WhiteRook,
            7 => Nibble::BlackRook,
            8 => Nibble::WhiteQueen,
            9 => Nibble::BlackQueen,
            10 => Nibble::WhiteKing,
            11 => Nibble::BlackKing,
            12 => Nibble::EnPassantPawn,
            13 => Nibble::WhiteRookCastle,
            14 => Nibble::BlackRookCastle,
            15 => Nibble::BlackMoveKing,
            _ => panic!("Invalid Nibble"),
        }
    }

    pub fn to_byte(nibbles: &[Nibble; 2] ) -> u8 {
        (nibbles[0] as u8) << 4 | (nibbles[1] as u8)
    }

    pub fn from_byte(byte: u8, idx: usize) -> Nibble {
        if idx == 0 {
            return Nibble::index(((byte >> 4) & 0xF) as usize);
        } 
        Nibble::index((byte & 0xF) as usize)
    }

    fn piece(self) -> Piece {
        Piece::index(self as usize / 2)

    }

    fn color(self) -> Color {
        if self as usize % 2 == 1 {
            return Color::Black;
        }
        Color::White

    }

    fn side_to_move(self) -> Color {
        if self == Nibble::BlackMoveKing {
            return Color::Black;
        }
        Color::White
    }

    pub fn get_info(&self) -> (Piece, Color, Color) {
        (self.piece(), self.color(), self.side_to_move())
    }
}