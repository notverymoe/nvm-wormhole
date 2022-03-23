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

use glam::{Vec2, mat2, vec2, vec3};

use crate::dim2::SQRT_3;

pub fn line_segment_thick(p: Vec2, a: Vec2, b: Vec2, th: f32) -> f32 {
    let ab = b-a;
    let l = ab.length();
    let d = ab/l;
    let q = p-(a+b)*0.5;
    let q = (mat2(vec2(d.x, d.y), vec2(-d.y, d.x))*q).abs();
    let q = q - vec2(l, th)*0.5;
    q.max(Vec2::ZERO).length() + q.x.max(q.y).min(0.0)
}

pub fn line_segment_thin(p: Vec2, a: Vec2, b: Vec2) -> f32 {
    let pa = p-a;
    let ba = b-a;
    let h = (pa.dot(ba)/ba.length_squared()).clamp(0.0, 1.0);
    (pa - ba*h).length()
}

pub fn parabola(pos: Vec2, k: f32) -> f32 {
    let pos = vec2(pos.x.abs(), pos.y);
    let ik  = 1.0/k;
    let p   = ik*(pos.y - 0.5*ik)/3.0;
    let q   = 0.25*ik*ik*pos.x;
    let h   = q*q - p*p*p;
    let r   = h.abs().sqrt();
    let x   = if h > 0.0 {
        (q+r).cbrt() - (q-r).abs().cbrt()*(r-q).signum()
    } else {
        2.0 * ((r/q).atan()/3.0).cos() * p.sqrt()
    };

    (pos-vec2(x, k*x*x)).length() * (pos.x-x).signum()
}

pub fn parabola_segment(pos: Vec2, wi: f32, he: f32) -> f32 {
    let pos = vec2(pos.x.abs(), pos.y);
    let ik  = wi*wi/he;
    let p   = ik*(he-pos.y-0.5*ik)/3.0;
    let q   = pos.x*ik*ik*0.25;
    let h   = q*q - p*p*p;
    let r   = h.abs().sqrt();
    let x   = if h > 0.0 {
        (q+r).cbrt() - (q-r).abs().cbrt()*(r-q).signum()
    } else {
        2.0 * ((r/q).atan()/3.0).cos() * p.sqrt()
    };
    (pos - vec2(x, he-x*x/ik)).length() * (ik*(pos.y-he) + pos.x*pos.x).signum()
}

pub fn bezier_segment(pos: Vec2, a: Vec2, b: Vec2, c: Vec2) -> f32 {
    let (a, b, c, d) = {
        let tmp_a = b - a;
        let tmp_b = a - 2.0*b + c;
        let tmp_c = tmp_a * 2.0;
        let tmp_d = a - pos;
        (tmp_a, tmp_b, tmp_c, tmp_d)
    };
    let kk = 1.0/b.length_squared();
    let kx = kk * a.dot(b);
    let ky = kk * (2.0*a.length_squared() + b.length_squared())/3.0;
    let kz = kk * d.dot(a);
    let p  = ky - kx*kx;
    let p3 = p*p*p;
    let q = kx*(2.0*kx*kx - 3.0*ky) + kz;
    let h = q*q + 3.0*p3;

    (if h >= 0.0 {
        let h  = h.sqrt();
        let x  = (vec2(h, -h)-q)/2.0;
        let uv = x.signum()*vec2(x.x.abs().cbrt(), x.y.abs().cbrt());
        let t  = (uv.x+uv.y-kx).clamp(0.0, 1.0);
        (d + (c + b*t)*t).length_squared()
    } else {
        let z = (-p).sqrt();
        let v = (q/(p*z*2.0)).acos()/3.0;
        let (m, n) = { let (s, c) = v.sin_cos(); (c, s*SQRT_3) };
        let t = {
            let tmp = vec3(m+m, -n-m, n-m)*z - kx;
            vec3(tmp.x.clamp(0.0, 1.0), tmp.y.clamp(0.0, 1.0), tmp.z.clamp(0.0, 1.0))
        };
        (d+(c+b*t.x)*t.x).length_squared().min((d+(c+b*t.y)*t.y).length_squared())
    }).sqrt()
}