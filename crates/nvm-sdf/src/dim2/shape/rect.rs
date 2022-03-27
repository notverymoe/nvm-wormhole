/*==============================================================================*\
** NotVeryMoe SDF | Copyright 2021 NotVeryMoe (projects@notvery.moe)            **
**==============================================================================**
** Adapted from https://iquilezles.org/ and associated coding example, provided **
** under the following license terms:                                           **
**                                                                              **
** Copyright © 2021 Inigo Quilez                                                **
** Permission is hereby granted, free of charge, to any person obtaining a      **
** copy of this software and associated documentation files (the "Software"),   **
** to deal in the Software without restriction, including without limitation    **
** the rights to use, copy, modify, merge, publish, distribute, sublicense,     **
** and/or sell copies of the Software, and to permit persons to whom the        **
** Software is furnished to do so, subject to the following conditions: The     **
** above copyright notice and this permission notice shall be included in all   **
** copies or substantial portions of the Software. THE SOFTWARE IS PROVIDED     **
** "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT     **
** NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR   **
** PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT      **
** HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN   **
** ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION **
** WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.              **
**==============================================================================*/

#[allow(unused_imports)]
use num_traits::Float;
use glam::{Vec2, Vec4, vec2};

use crate::dim2::angle_to_vec;

use super::{ndot, line_segment_thick};

pub fn square(p: Vec2, r: f32) -> f32 {
    rectangle(p, vec2(r, r)) // TODO opt
}

pub fn rectangle(p: Vec2, b: Vec2) -> f32 {
    let d = p.abs() - b;
    d.max(Vec2::ZERO).length() + d.x.max(d.y).min(0.0)
}

pub fn rectangle_oriented(p: Vec2, b: Vec2, a: f32) -> f32 {
    let r = angle_to_vec(a);
    let off = b.x*r;
    line_segment_thick(p, -off, off, b.y)
}

pub fn rectangle_rounded(p: Vec2, b: Vec2, r: f32) -> f32 {
    let q = (p.abs() - b + vec2(r, r)).max(Vec2::ZERO);
    q.x.max(q.y).min(0.0) + q.length() - r
}

pub fn rectangle_rounded_4(p: Vec2, b: Vec2, r: Vec4) -> f32 {
    rectangle_rounded(p, b, match (p.x > 0.0, p.y > 0.0) {
        (false, false) => r.x, 
        (false,  true) => r.y,
        ( true, false) => r.z,
        ( true,  true) => r.w,
    })
}

pub fn rhombus(p: Vec2, b: Vec2) -> f32 {
    let p = p.abs();
    let h = (ndot(b-2.0*p, b)/b.length_squared()).clamp(-1.0, 1.0);
    let d = (p - 0.5*b*vec2(1.0-h, 1.0+h)).length();
    d * (p.x*b.y + p.y*b.x - b.x*b.y).signum()
}

pub fn trapezoid_iso(p: Vec2, r1: f32, r2: f32, he: f32) -> f32 {
    let k1 = vec2(r2, he);
    let k2 = vec2(r2-r1, 2.0*he);
    let p  = vec2(p.x.abs(), p.y);
    let ca = vec2(p.x-p.x.min(if p.y < 0.0 { r1 } else { r2 }), p.y.abs()-he);
    let cb = p - k1 + k2*((k1-p).dot(k2)/k2.length_squared()).clamp(0.0, 1.0);
    let s  = if cb.x < 0.0 && ca.y < 0.0 { -1.0 } else { 1.0 };
    s * ca.length_squared().min(cb.length_squared()).sqrt()
}

pub fn parallelogram(p: Vec2, wi: f32, he: f32, sk: f32) -> f32 {
    let e = vec2(sk, he);
    let p = if p.y < 0.0 { -p } else { p };
    let w = {
        let tmp = p - e;
        vec2(tmp.x.clamp(-wi, wi), tmp.y)
    };
    let d = vec2(w.length_squared(), -w.y);
    let s = p.x*e.y - p.y*e.x;
    let p = if s < 0.0 { -p } else { p };
    let v = {
        let tmp = p - vec2(wi, 0.0);
        tmp - e*(tmp.dot(e)/e.length_squared()).clamp(-1.0, 1.0)
    };
    let d = d.min(vec2(v.length_squared(), wi*he-s.abs()));
    -d.x.sqrt()*d.y.signum()
}
