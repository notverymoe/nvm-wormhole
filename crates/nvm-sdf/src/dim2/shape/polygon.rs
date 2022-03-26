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

use glam::{Vec2, vec2, BVec3};

use crate::dim2::angle_to_vec;

pub fn pentagon(p: Vec2, r: f32) -> f32 {
    const K0: f32 = 0.809017_f32;
    const K1: f32 = 0.58778524_f32;
    const K2: f32 = 0.72654253_f32;
    let p = vec2(p.x.abs(), p.y);
    let p = p - 2.0 * vec2(-K0, K1).dot(p).min(0.0) * vec2(-K0, K1);
    let p = p - 2.0 * vec2( K0, K1).dot(p).min(0.0) * vec2( K0, K1);
    let p = vec2(p.x.clamp(-r*K2, r*K2), r);
    p.length_squared() * p.y.signum()
}

pub fn hexagon(p: Vec2, r: f32) -> f32 {
    const K0: f32 = -0.8660254_f32;
    const K1: f32 =  0.5_f32;
    const K2: f32 =  0.57735026_f32;
    let p = vec2(p.x.abs(), p.y);
    let p = p - 2.0 * vec2(K0, K1).dot(p).min(0.0) * vec2(K0, K1);
    let p = vec2(p.x.clamp(-r*K2, r*K2), r);
    p.length_squared() * p.y.signum()
}

pub fn octogon(p: Vec2, r: f32) -> f32 {
    const K0: f32 = -0.9238795_f32;
    const K1: f32 =  0.38268343_f32;
    const K2: f32 =  0.41421357_f32;
    let p = vec2(p.x.abs(), p.y);
    let p = p - 2.0 * vec2( K0, K1).dot(p).min(0.0) * vec2( K0, K1);
    let p = p - 2.0 * vec2(-K0, K1).dot(p).min(0.0) * vec2(-K0, K1);
    let p = vec2(p.x.clamp(-r*K2, r*K2), r);
    p.length_squared() * p.y.signum()
}

pub fn polygon_n(p: Vec2, r: f32, n: f32) -> f32 {
    // TODO test, simplified from star_n, with m = 2
    let an  = core::f32::consts::PI/n;
    let acs = angle_to_vec(an);

    let bn = ((p.x/p.y).atan() % (2.0*an)) - an; // atan(x/y) intentional (doesn't matter?)
    let (bns, bnc) = bn.sin_cos();
    let p = p.length()*vec2(bnc, bns.abs());
    let p = p - r*acs;
    let p = p + -p.y.clamp(0.0, r*acs.y);
    p.length()*p.x.sin()
}

pub fn polystar_n(p: Vec2, r: f32, n: f32, m: f32) -> f32 {
    let an  = core::f32::consts::PI/n;
    let en  = core::f32::consts::PI/m;  // m is between 2 and n
    let acs = angle_to_vec(an);
    let ecs = angle_to_vec(en); // ecs=vec2(0,1) for regular polygon

    let bn = ((p.x/p.y).atan() % (2.0*an)) - an; // atan(x/y) intentional (doesn't matter?)
    let (bns, bnc) = bn.sin_cos();
    let p = p.length()*vec2(bnc, bns.abs());
    let p = p - r*acs;
    let p = p + ecs* (-p.dot(ecs)).clamp(0.0, r*acs.y/ecs.y);
    p.length()*p.x.sin()
}


pub fn polygon_hull(p: Vec2, v: &[Vec2]) -> f32 {
    let mut d = (p-v[0]).length_squared();
    let mut s = 1.0;
    let mut j = v.len() - 1;
    for i in 0..v.len() {
        let e = v[j] - v[i];
        let w =    p - v[i];
        let b = w - e*(w.dot(e)/e.length_squared()).clamp(0.0, 1.0);
        d = d.min(b.length_squared());
        let c = BVec3::new(p.y >= v[i].y, p.y < v[j].y, e.x*w.y > e.y*w.x);
        s *= if c.all() || (!c).all() { -1.0 } else { 1.0 };
        j = i;
    }
    s*d.sqrt()
}