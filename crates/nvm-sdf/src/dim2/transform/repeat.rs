/*===================================================================*\
** NotVeryMoe DSF | Copyright 2021 NotVeryMoe (projects@notvery.moe) **
\*===================================================================*/

#[allow(unused_imports)]
use num_traits::Float;
use glam::{Vec2, vec2};

pub fn repeat_axis(p: Vec2, b: Vec2, l: f32, n: f32) -> Vec2 {
    let dist = repeat_axis_inner(b.dot(p), l, n);
    dist*b
}

pub fn repeat_box(p: Vec2, b: Vec2, n: Vec2) -> Vec2 {
    vec2(
        p.x.signum() * repeat_axis_inner_n(b.x + p.x.abs(), 2.0*b.x, n.x),
        p.y.signum() * repeat_axis_inner_n(b.y + p.y.abs(), 2.0*b.y, n.y),
    )
}

pub fn repeat_circle(p: Vec2, b: f32, n: f32) -> Vec2 {
    let len  = p.length();
    let axis = if len <= 0.0 { p } else { p/len };
    let dist = repeat_axis_inner(len, b, n);
    dist*axis
}

fn repeat_axis_inner(p: f32, l: f32, n: f32) -> f32 {
    p.signum()*repeat_axis_inner_n(p.abs(), l, n)
}

fn repeat_axis_inner_n(p: f32, l: f32, n: f32) -> f32 {
    let r = (n - p/l).max(0.0);
    p - r*l
}