use cozy_chess::{BitBoard, Board, BoardBuilder, BoardBuilderError, Color};
use self::nibble::Nibble;
use std::fs::File;
use std::io::{self, BufReader, Read};

mod nibble; 

fn quantize(value: f32) -> u8 {
    // Scale the value to fit into the u8 range [0, 255]
    (value * 255.0) as u8
}

#[derive(Debug, PartialEq)]
pub struct AnnotatedBoard {
    board: Board,
    score: f32,
    outcome: f32
}

#[derive(Debug, PartialEq)]
pub struct CompressedBoard {
    bytes: Vec<u8>
}

impl AnnotatedBoard {
    pub fn new(board: Board, score: f32, outcome: f32) -> AnnotatedBoard {
        AnnotatedBoard{
            board,
            score, 
            outcome
        }
    }

    pub fn board(&self) -> &Board{
        &self.board
    }

    pub fn score(&self) -> f32 {
        self.score
        
    }

    pub fn outcome(&self) -> f32{
        self.outcome
    }

    fn occupied(&self) -> BitBoard {
        self.board().occupied()
    }

    fn num_pieces(&self) -> usize {
        self.occupied().len() as usize
    }

    fn num_piece_bytes(&self) -> usize {
        let num_pieces = self.num_pieces();
        num_pieces / 2 + num_pieces % 2
    }

    fn num_bytes(&self) -> usize {
        // 8 bytes for occupied, 1 for score and 1 for outcome
        self.num_piece_bytes() + 10
    }

    pub fn compress(&self) -> CompressedBoard {

        // Get the occupied bitboard
        let occupied = self.occupied().0;

        let mut bytes: Vec<u8> = Vec::with_capacity(self.num_bytes());

        // Extend the Vec with the bitboard's bytes
        bytes.extend_from_slice(&occupied.to_le_bytes());

        // Get the side to move
        let stm = self.board().side_to_move();

        // Iterate over the pieces two at a time
        // Convert them to nibbles and pack them into a byte
        let mut idx = 0;
        let mut nibble_pair = [Nibble::index(0); 2];
        for square in self.occupied() {
            let piece = self.board().piece_on(square).unwrap();
            let color = self.board().color_on(square).unwrap();
            nibble_pair[idx] = Nibble::new(piece, color, stm);
            if idx == 1 {
                bytes.push(Nibble::to_byte(&nibble_pair));
            }
            idx = (idx + 1) % 2;
        }

        // Quantize the score and outcome
        bytes.push(quantize(self.score()));
        bytes.push(quantize(self.outcome()));

        CompressedBoard {
            bytes
        }

    }

}

impl CompressedBoard {

    pub fn new(bytes: Vec<u8>) -> CompressedBoard {
        CompressedBoard {bytes}
    }

    pub fn decompress(&self) -> Result<AnnotatedBoard, BoardBuilderError> {
        let mut builder = BoardBuilder::empty();

        let occupied_bytes: [u8; 8] = self.bytes[..8].try_into().map_err(|_| BoardBuilderError::InvalidBoard)?;
        let occupied = BitBoard(u64::from_le_bytes(occupied_bytes));

        let mut bytes_read = 8;
        let mut idx = 0;

        for square in occupied {
            let byte = self.bytes[bytes_read];
            let nibble = Nibble::from_byte(byte, idx % 2);

            let (piece, color, stm) = nibble.get_info();
            if stm == Color::Black {
                builder.side_to_move = Color::Black;
            }

            builder.board[square as usize] = Some((piece, color));
            idx += 1;
            if idx % 2 == 0 { bytes_read += 1;};
        }

        bytes_read += idx % 2;

        let score = self.bytes[bytes_read] as f32 / 255.0;
        let outcome = self.bytes[bytes_read+1] as f32 / 255.0;

        let pos = AnnotatedBoard {
            board: builder.build()?,
            score: score,
            outcome: outcome

        };

        Ok(pos)

    }
}

    