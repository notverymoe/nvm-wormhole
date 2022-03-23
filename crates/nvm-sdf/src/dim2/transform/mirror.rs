/*===================================================================*\
** NotVeryMoe DSF | Copyright 2021 NotVeryMoe (projects@notvery.moe) **
\*===================================================================*/

use glam::{Vec2, vec2};

pub fn mirror_axis(p: Vec2, b: Vec2, l: f32) -> Vec2 {
    let dist = mirror_axis_inner(b.dot(p), l);
    dist*b
}

pub fn mirror_box(p: Vec2, b: Vec2) -> Vec2 {
    vec2(
        p.x.signum() * mirror_axis_inner(b.x + p.x.abs(), 2.0*b.x),
        p.y.signum() * mirror_axis_inner(b.y + p.y.abs(), 2.0*b.y),
    )
}

pub fn mirror_circle(p: Vec2, b: f32) -> Vec2 {
    let len  = p.length();
    let axis = if len <= 0.0 { p } else { p/len };
    let dist = mirror_axis_inner(len, b);
    dist*axis
}

fn mirror_axis_inner(p: f32, l: f32) -> f32 {
    (p - l).signum() * p
}
