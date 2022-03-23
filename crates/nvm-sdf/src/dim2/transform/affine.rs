/*===================================================================*\
** NotVeryMoe DSF | Copyright 2021 NotVeryMoe (projects@notvery.moe) **
\*===================================================================*/

use glam::{Vec2, mat2, vec2};

use crate::dim2::angle_to_vec;

pub fn translate(p: Vec2, t: Vec2) -> Vec2 {
    p - t
}

pub fn rotate(p: Vec2, a: f32) -> Vec2 {
    rotate_direct(p, angle_to_vec(a))
}

pub fn rotate_direct(p: Vec2, a: Vec2) -> Vec2 {
    mat2(vec2(a.x, a.y), vec2(-a.y, a.x)) * p
}
