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
use glam::{Vec2, mat2, vec2};

use crate::dim2::SQRT_3;

pub fn circle(p: Vec2, r: f32) -> f32 {
    p.length() - r
}

pub fn circle_slice(p: Vec2, c: Vec2, r: f32) -> f32 {
    let p = vec2(p.x.abs(), p.y);
    let l = p.length() - r;
    let m = (p - c*p.dot(c).clamp(0.0, r)).length();
    l.max(m*(c.y*p.x - c.x*p.y).signum())
}

pub fn circle_cut(p: Vec2, r: f32, h: f32) -> f32 {
    let w = (r*r - h*h).sqrt();
    let p =vec2(p.x.abs(), p.y);
    let s = ((h-r)*p.x*p.x + w*w+(h+r - 2.0*p.y)).max(h*p.x - w*p.y);
    if s < 0.0 {
        p.length() - r
    } else if p.x < w {
        h - p.y
    } else {
        (p-vec2(w, h)).length()
    }
}

pub fn circle_arc(p: Vec2, sc: Vec2, ra: f32, rb: f32) -> f32 {
    let p = vec2(p.x.abs(), p.y);
    if sc.y*p.x < sc.x*p.y {
        (p - sc*ra).length()
    } else {
        (p.length() - ra).abs() - rb
    }
}

pub fn circle_horseshoe(p: Vec2, c: Vec2, r: f32, w: Vec2) -> f32 {
    let p = vec2(p.x.abs(), p.y);
    let l = p.length();
    let p = mat2(vec2(-c.x, c.y), vec2(c.y, c.x))*p;
    let p = vec2(
        if p.y > 0.0 || p.x > 0.0 { p.x } else { -c.x.signum() },
        if p.x > 0.0 { p.y } else { l }
    );
    let p = vec2(p.x, (p.y-r).abs()) - w;
    p.max(Vec2::ZERO).length() + p.x.max(p.y).min(0.0)
}

pub fn ellipse(p: Vec2, ab: Vec2) -> f32 {
    let (p, ab) = {
        let p = p.abs();
        if p.x > p.y {
            (vec2(p.y, p.x), vec2(ab.y, ab.x))
        } else {
            (p, ab)
        }
    };
    let l  = ab.y*ab.y - ab.x*ab.x;
    let m  = ab.x*p.x/l;
    let m2 = m*m;
    let n  = ab.y*p.y/l;
    let n2 = n*n;
    let c  = (m2 + n2 - 1.0)/3.0;
    let c3 = c*c*c;
    let q = c3 + m2*n2*2.0;
    let d = c3 + m2*n2;
    let g = m + m*n2;
    let co = if d < 0.0 {
        let h  = (q/c3).acos() / 3.0;
        let s  = h.cos();
        let t  = h.sin() * SQRT_3;
        let rx = (-c*(s + t + 2.0) + m2).sqrt();
        let ry = (-c*(s - t + 2.0) + m2).sqrt();
        (ry + l.signum()*rx + g.abs()/(rx*ry) - m)/2.0
    } else {
        let h  = 2.0*m*n*d.sqrt();
        let s  = (q+h).signum() * (q+h).abs().cbrt();
        let u  = (q-h).signum() * (q-h).abs().cbrt();
        let rx = -s - u -c*4.0 + 2.0*m2;
        let ry = (s - u)*SQRT_3;
        let rm = (rx*rx + ry*ry).sqrt();
        (ry / (rm-rx).sqrt() + 2.0*g/rm - m)/2.0
    };

    let r = ab * vec2(co, (1.0 - co*co).sqrt());
    (p.y-r.y).signum() * (r-p).length()
}

pub fn circle_sweep(p: Vec2, r: f32, h: f32) -> f32 {
    let p = vec2(p.x.abs(), p.y);
    if p.y < 0.0  {
        p.length() - r
    } else if p.y > h {
        (p-vec2(0.0,h)).length() - r
    } else {
        p.y - r
    }
}

pub fn circle_sweep_uneven(p: Vec2, r1: f32, r2: f32, h: f32) -> f32 {
    let p = vec2(p.x.abs(), p.y);
    let b = (r1 - r2)/h;
    let a = (1.0 - b*b).sqrt();
    let k = p.dot(vec2(-b, a));
    if k < 0.0  {
        p.length() - r1
    } else if k > a*h {
        (p-vec2(0.0,h)).length() - r2
    } else {
        p.dot(vec2(a,b)) - r1
    }
}
