use shakmaty_syzygy::{Dtz, Wdl, MaybeRounded};
use cozy_chess::{Color};

fn clamp<T: Ord>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub fn dtz_to_f32(mr_dtz: MaybeRounded<Dtz>, wdl: Wdl, stm: Color) -> f32 {
    let mut dtz_i32: i32 = clamp(mr_dtz.ignore_rounding().into(), -128, 127);
    if stm == Color::Black {dtz_i32 *= -1;}
    if wdl == Wdl::Draw {
        dtz_i32 = 0;
    }

    let score_u8: u8 = match dtz_i32.cmp(&0) {
        std::cmp::Ordering::Less => {
            (-dtz_i32 -1) as u8
            // Handle the case where the number is less than 0
        },
        std::cmp::Ordering::Greater => {
            (-dtz_i32 + 256) as u8
        },
        std::cmp::Ordering::Equal => {
            128
        }
    };

    let score_f32 = score_u8 as f32 / 255.0;

    // if stm == Color::Black {
    //     return 1.0 -score_f32;
    // }

    score_f32
    

}

pub fn wdl_to_f32(wdl: Wdl, stm: Color) -> f32 {
    match wdl {
        Wdl::Win => 1.0,
        Wdl::Loss => 0.0,
        _ => 0.5
    }

} 