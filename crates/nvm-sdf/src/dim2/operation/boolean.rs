/*===================================================================*\
** NotVeryMoe DSF | Copyright 2021 NotVeryMoe (projects@notvery.moe) **
\*===================================================================*/

use super::unary_negate;

pub fn boolean_union(a: f32, b: f32) -> f32 {
    a.min(b)
}

pub fn boolean_intersect(a: f32, b: f32) -> f32 {
    a.max(b)
}

pub fn boolean_difference(a: f32, b: f32) -> f32 {
    boolean_intersect(a, unary_negate(b))
}

pub fn boolean_interpolate(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a)*t
}
