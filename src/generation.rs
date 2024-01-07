use cozy_chess::{Piece};
use itertools::Itertools;
use cozy_chess::{BitBoard, Board, BoardBuilder, BoardBuilderError, Color, Square};
use rand::Rng;
use std::collections::HashSet;
type GenericError = Box<dyn std::error::Error>;
use crate::compression::AnnotatedBoard;
use crate::score::*;

            
use shakmaty::{CastlingMode, Chess, fen::Fen, Position};
use shakmaty_syzygy::{Tablebase, MaybeRounded, Wdl, Dtz, Syzygy};


fn piece_value(piece: Piece) -> u32 {
    match piece {
        Piece::Pawn => 0,
        Piece:: Knight => 3,
        Piece::Bishop => 3,
        Piece:: Rook => 5,
        Piece::Queen => 9,
        Piece:: King => 0,
    }
}

fn generate_random_unique_values(n: usize, max_value: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut unique_values = HashSet::with_capacity(n);
    let mut result = Vec::with_capacity(n);

    while unique_values.len() < n {
        let random_value = rng.gen_range(0..max_value);

        if unique_values.insert(random_value) {
            result.push(random_value);
        }
    }

    result
}

pub struct Generator {
    pub num_white: usize,
    pub num_black: usize,
    pub mat_score: u32,
    pub pawn_score: u32,
    combos: Vec<(Vec<Piece>, Vec<Piece>)>
    //combos: [Vec<Vec<Piece>>; Color::NUM]
}

fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

impl Generator {

    pub fn new(num_white: usize, num_black: usize, mat_score: u32, pawn_score: u32) -> Generator {
        let mut generator = 
            Generator {
                num_white,
                num_black,
                mat_score, 
                pawn_score, 
                combos: Vec::new()
            };
        generator.gen_combos();
        generator
    }

    pub fn set_num_pieces(&mut self, num_white: usize, num_black: usize) {
        self.num_white = num_white;
        self.num_black = num_white;
        self.gen_combos();
    }

    pub fn set_score(&mut self, mat_score: u32, pawn_score: u32) {
        self.mat_score = mat_score;
        self.pawn_score = pawn_score;
        self.gen_combos();
    }

    pub fn set(&mut self, num_white: usize, num_black: usize, mat_score: u32, pawn_score: u32) {
        self.set_num_pieces(num_white, num_black);
        self.set_score(mat_score, pawn_score);
        self.gen_combos();
    }

    pub fn gen_combos(&mut self) {

        self.combos.clear();
        
        let pieces = [Piece::Pawn, Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen].into_iter();

        let white_combos = pieces.clone().combinations_with_replacement(self.num_white-1);
        let black_combos = pieces.combinations_with_replacement(self.num_black-1);
        
        //let mut combos: Vec<(Vec<Piece>, Vec<Piece>)> = Vec::new();

        for (mut white_combo, mut black_combo) in white_combos.zip(black_combos){
            let white_value: u32 = white_combo.iter().map(|&p| piece_value(p)).sum();
            let black_value: u32 = black_combo.iter().map(|&p| piece_value(p)).sum();

            let score = (white_value as i32- black_value as i32).abs() as u32; 

            white_combo.push(Piece::King);
            black_combo.push(Piece::King);

            if score <= self.mat_score {
                self.combos.push((white_combo, black_combo));
            }

        }
    }

    pub fn gen_board(&self, rng: &mut impl Rng) -> Option<Board> {
        
        let combo_idx = rng.gen_range(0..self.combos.len());
        let random_combo = &self.combos[combo_idx];


        let mut builder = BoardBuilder::empty();
        let num_squares = self.num_white + self.num_black;
        for (idx, &sq_idx) in generate_random_unique_values(num_squares, Square::NUM).iter().enumerate() {
            let (color, piece) = if idx < self.num_white {
                (Color::White, random_combo.0[idx])
            } else {
                (Color::Black, random_combo.1[idx-self.num_white])
            };
            
            builder.board[sq_idx] = Some((piece, color));


        }
        builder.side_to_move = Color::index(combo_idx % 2);
        let board = builder.build().ok()?;
        let fen = format!("{}", board);
        Some(board)
        //println!("{}", type_of(var));

    }

    pub fn gen_boards(&self, num_boards: usize) -> Result<Vec<AnnotatedBoard>, GenericError>{
        let mut ann_boards = Vec::new();
        let mut rng = rand::thread_rng();
        let mut tables: Tablebase<Chess> = Tablebase::new();
        tables.add_directory("3_4_5")?;

        for idx in 0..num_boards {
            if let Some(board) = self.gen_board(&mut rng) {
                let result_pos: Result<Chess, shakmaty::PositionError<_>> = format!("{}", board)
                .parse::<Fen>()?
                .into_position(CastlingMode::Standard);

                match result_pos{
                    Ok(pos) => {
                        let wdl = tables.probe_wdl_after_zeroing(&pos)?;

                        let dtz = tables.probe_dtz(&pos)?;
        
                        println!("{:?} {:?} {:?}", format!("{}", board), wdl, dtz);
                        let wdl_f32 = wdl_to_f32(wdl, board.side_to_move());
                        let dtz_f32 = dtz_to_f32(dtz, wdl, board.side_to_move());
                        ann_boards.push(AnnotatedBoard::new(board, dtz_f32, wdl_f32));
                        println!("{:?} {:?} {}", wdl, dtz, dtz_f32);

                        //println!("{:?}", format!("{}", board));
                    },
                    Err(e) => {
                        let fen = format!("{}", board);
                        println!("{e} {fen}")
                    }
                }
            }
        }

        Ok(ann_boards)
    }

    

}

