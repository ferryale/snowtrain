use rayon::prelude::*;
use cozy_chess::{Piece, Color};
use snowtrain::compression::*;
use snowtrain::dataload::*;
use itertools::Itertools;
use core::slice::Iter;
use snowtrain::generation::*;

// struct MyFloat {
//     value: f32
// }
// struct Rayon {
//     i32_vector: Vec<i32>,
//     float_value: MyFloat,
//     u32_vector: Vec<u32>,
// }




fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

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

fn main() {
    let pieces = [Piece::Pawn, Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen];
    let generator = Generator::new(3, 2, 2, 2);
    let v = generator.gen_boards(100);
    
    //let perm: Vec<Vec<Piece>> = pieces.into_iter().combinations_with_replacement(6).collect();
    //     el.iter().map(|&p| piece_value(*p)).sum::<u32>()
    //     );
    // //let sum = map(|p| piece_value(p)).sum();
    

    // let combos: Vec<_> = pieces.iter().combinations_with_replacement(6).collect();
    // println!("{:?}", type_of(combos));
    // let values = combos.clone().map(|combo|
    //     combo.iter().map(|&p| piece_value(*p)).sum::<u32>().filter(|val| val < 5)
    // );
    // let filtered_combos = combos.filter

    // for (combo, value) in combos.zip(values) {
        
    //     //let sum: u32 = el.iter().map(|p| piece_value(**p)).sum();
    //     println!("{:?} {}", el, el);

    // }
    


}


// fn main() {

//     println!("---");
//     let p = Piece::index(0);
//     println!("{:?}", p);

//     let file_path = "test100.bin";
//     let num_chunks = 5; // Set the desired number of chunks

//     let mut file_reader1 = match FileReader::new(file_path) {
//         Ok(reader) => reader,
//         Err(e) => {
//             eprintln!("Error opening file: {}", e);
//             return;
//         }
//     };

//     let chunks1 = file_reader1.read_all_batches(100).expect("Fail");
//     println!("Chunks 1: {:?}", chunks1.len());

//     let mut file_reader2 = match FileReader::new(file_path) {
//         Ok(reader) => reader,
//         Err(e) => {
//             eprintln!("Error opening file: {}", e);
//             return;
//         }
//     };

//     let chunks2 = file_reader2.read_all_chunks().expect("Fail");
//     println!("Chunks 1: {:?}", chunks2.len());

//     let mut file_reader3 = match FileReader::new(file_path) {
//         Ok(reader) => reader,
//         Err(e) => {
//             eprintln!("Error opening file: {}", e);
//             return;
//         }
//     };

//     let mut idx = 0;
//     let mut chunks3 = Vec::new();
//     for chunk in file_reader3{
//         chunks3.push(chunk.clone());
//         // Process each chunk
        
//         idx += 1;

//         if idx % 100 == 0 {
//             println!("Chunk {} {:?}", idx, chunk);
//         }

//     }

//     println!("Chunks = {}", idx);
//     let var = chunks2 == chunks3;

//     println!("Chunks = {}", var);

//     let mut compr_boards = Vec::new();
//     let mut idx = 0;
//     for chunk in chunks3 {
//         //
//         idx += 1;
//         if idx > 10 {break;}

//         let compr_board = CompressedBoard::new(chunk);
//         let annotated_board = compr_board.decompress();
//         compr_boards.push(compr_board);
//         println!("{:?}", annotated_board);

//     }

//     let res: Vec::<AnnotatedBoard> = compr_boards.par_iter().map(|compr| {

//         compr.decompress().unwrap()
//     }).collect();

//     println!("{:?}", res)

//     // for idx in 0..1000 {
//     //     let var = chunks2[idx] == chunks3[idx];
//     //     println!("{}", var);
//     // }
//     // // Read and print the first set of chunks
//     // let chunks1 = match file_reader.read_n_chunks(3) {
//     //     Ok(chunks) => chunks,
//     //     Err(e) => {
//     //         eprintln!("Error reading chunks: {}", e);
//     //         return;
//     //     }
//     // };

//     // println!("Chunks 1: {:?}", chunks1);

//     // // Perform some processing or stopping logic here

//     // // Read and print the second set of chunks
//     // let chunks2 = match file_reader.read_n_chunks(2) {
//     //     Ok(chunks) => chunks,
//     //     Err(e) => {
//     //         eprintln!("Error reading chunks: {}", e);
//     //         return;
//     //     }
//     // };

//     //println!("Chunks 2: {:?}", chunks2);
//     // let rayon_array = vec![
//     //     Rayon {
//     //         i32_vector: vec![1, 2, 3],
//     //         float_value: MyFloat{value: 42.0},
//     //         u32_vector: vec![4, 5, 6],
//     //     },
//     //     // Add more instances of Rayon struct as needed
//     // ];

//     // rayon_array.par_iter().for_each(|rayon_instance| {
//     //     // Access fields of the struct and perform parallel operations
//     //     let sum_i32: i32 = rayon_instance.i32_vector.iter().sum();
//     //     let product_u32: u32 = rayon_instance.u32_vector.iter().product();

//     //     let result = sum_i32 as f32 * rayon_instance.float_value.value * product_u32 as f32;

//     //     // let results: Vec<f32> = rayon_array
//     //     // .par_iter()
//     //     // .map(|rayon_instance| {
//     //     //     // Access fields of the struct and perform parallel operations
//     //     //     let sum_i32: i32 = rayon_instance.i32_vector.iter().sum();
//     //     //     let product_u32: u32 = rayon_instance.u32_vector.iter().product();

//     //     //     sum_i32 as f32 * rayon_instance.float_value * product_u32 as f32
//     //     // })
//     //     // .collect();

//     //     println!("Result: {:?}", result);
//     // });

//     // let my_vector = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

//     // for chunk in my_vector.chunks(2) {
//     //     // 'chunk' is a slice of two elements
//     //     for &element in chunk {
//     //         println!("{}", element);
//     //     }
//     //     println!("---");
//     // }
// }