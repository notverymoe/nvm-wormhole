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

use glam::{Vec2, mat2, vec2};

use crate::dim2::SQRT_3;

// TODO cross square, requires fixed interior

pub fn cross_round(p: Vec2, w: f32, r: f32) -> f32 {
    let p = p.abs();
    (p - (p.x+p.y).min(w)*0.5).length() - r
}

pub fn vesica(p: Vec2, r: f32, d: f32) -> f32 {
    let p = p.abs();
    let b = (r*r - d*d).sqrt();
    if (p.y-b)*d > p.x*b {
        (p-vec2(0.0, b)).length()
    } else {
        (p-vec2(-d, 0.0)).length() - r
    }
}

pub fn moon(p: Vec2, d: f32, ra: f32, rb: f32) -> f32 {
    let p = vec2(p.x, p.y.abs());
    let a = (ra*ra - rb*rb + d*d) / (2.0*d);
    let b = (ra*ra - a*a).max(0.0).sqrt();
    if d*(p.x*b - p.y*a) >  d*d*(b - p.y).max(0.0) {
        (p - vec2(a,b)).length() - ra
    } else {
        (p.length() - ra).max(-(p-vec2(d, 0.0)).length() - rb)
    }
}

pub fn egg(p: Vec2, ra: f32, rb: f32) -> f32 {
    let p = vec2(p.x.abs(), p.y);
    let r = ra - rb;
    if p.y < 0.0 {
        p.length() - r
    } else if SQRT_3*(p.x+r) < p.y {
        vec2(p.x, p.y-SQRT_3*r).length()
    } else {
        (vec2(p.x+r, p.y).length() - 2.0*r) -rb
    }
}

pub fn heart(p: Vec2, r: f32) -> f32 {
    let p = vec2(p.x.abs(), p.y)/r;

    if p.y + p.x > 1.0 {
        ((p-vec2(0.25, 0.75)).length_squared() - core::f32::consts::SQRT_2/4.0).sqrt()
    } else {
        ((p-vec2(0.0, 1.0)).length_squared().min((p-0.5*(p.x+p.y).max(0.0)).length_squared())).sqrt() * (p.x-p.y).signum()
    }

}

pub fn tunnel(p: Vec2, wh: Vec2) -> f32 {
    let p  = vec2(p.x.abs(), p.y);
    let q  = p - wh;
    let d1 = vec2(q.x.max(0.0), q.y).length_squared();
    let q  = vec2(if p.y > 0.0 { q.x } else { p.length() - wh.x }, q.y);
    let d2 = vec2(q.x, q.y.max(0.0)).length_squared();
    let d  = d1.min(d2).sqrt();
    if q.x.max(q.y) < 0.0 {
        -d
    } else {
        d
    }
}

pub fn stairs(p: Vec2, wh: Vec2, n: f32) -> f32 {
    let ba = wh*n;
    let d = (p - vec2(p.x.clamp(0.0, ba.x), 0.0)).length_squared()
       .min((p - vec2(ba.x, p.y.clamp(0.0, ba.y))).length_squared());
    let s = (-p.y).max(p.x-ba.x).signum();
    let dia = wh.length();
    let p = mat2(vec2(wh.x, -wh.y), vec2(wh.y, wh.x))*p/dia;
    let id = (p.x/dia).round().clamp(0.0, n-1.0);
    let p = vec2(p.x - id*dia, p.y);
    let p = mat2(vec2(wh.x, -wh.y), vec2(wh.y, wh.x))*p/dia;
    let hh = wh.y/2.0;
    let p = vec2(p.x, p.y-hh);
    let s = if p.y < hh * p.x.signum() { 1.0 } else { s };
    let p = if id < 0.5 || p.x > 0.0 { p } else { -p };
    let d = d.min((p-vec2(                 0.0, p.y.clamp(-hh, hh))).length_squared());
    let d = d.min((p-vec2(p.x.clamp(0.0, wh.x),                 hh)).length_squared());
    d.sqrt()*s
}
