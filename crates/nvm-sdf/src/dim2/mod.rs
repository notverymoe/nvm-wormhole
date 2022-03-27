/*===================================================================*\
** NotVeryMoe DSF | Copyright 2021 NotVeryMoe (projects@notvery.moe) **
\*===================================================================*/

#[allow(unused_imports)]
use num_traits::Float;

pub mod shape;
pub mod transform;
pub mod operation;
pub mod tree;

const SQRT_3: f32 = 1.7320508_f32;

fn angle_to_vec(angle: f32) -> glam::Vec2 {
    let (s, c) = angle.sin_cos();
    glam::Vec2::new(c, s)
}
