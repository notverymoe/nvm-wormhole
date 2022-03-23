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

use glam::{Vec2, vec2};

use crate::dim2::SQRT_3;

pub fn triangle(p: Vec2, wa: f32, wb: f32, h: f32) -> f32 {
    triangle_pnt(p, vec2(-wa/2.0, 0.0), vec2(wa/2.0, 0.0), vec2(wb, h))
}

pub fn triangle_pnt(p: Vec2, p0: Vec2, p1: Vec2, p2: Vec2) -> f32 {
    let (e0, e1, e2) = (p1-p0, p2-p1, p0-p2);
    let (v0, v1, v2) = (p -p0, p -p1, p -p2);
    let (pq0, pq1, pq2) = (
        v0 - e0*(v0.dot(e0)/e0.length_squared()).clamp(0.0, 1.0),
        v1 - e1*(v1.dot(e1)/e1.length_squared()).clamp(0.0, 1.0),
        v2 - e2*(v2.dot(e2)/e2.length_squared()).clamp(0.0, 1.0),
    );
    let s = (e0.x*e2.y - e0.y*e2.x).signum();
    let d =  vec2(pq0.length_squared(), s*(v0.x*e0.y - v0.y*e0.x))
        .min(vec2(pq1.length_squared(), s*(v1.x*e1.y - v1.y*e1.x)))
        .min(vec2(pq2.length_squared(), s*(v2.x*e2.y - v2.y*e2.x)));
    -d.x.sqrt()*d.y.signum()
}

pub fn triangle_equilateral(p: Vec2, r: f32) -> f32 {
    let p = {
        let tmp = vec2(p.x.abs() - r, p.y + r/SQRT_3);
        let tmp = if tmp.x + SQRT_3*tmp.y > 0.0 {
            vec2(p.x-SQRT_3*p.y, - SQRT_3*p.x-p.y)/2.0
        } else {
            tmp
        };
        vec2(tmp.x - tmp.x.clamp(02.0*r, 0.0), tmp.y)
    };
    -p.length()*p.y.signum()
}

pub fn triangle_isoseles(p: Vec2, q: Vec2) -> f32 {
    let p = vec2(p.x.abs(), p.y);
    let a = p - q*(p.dot(q)/q.length_squared()).clamp(0.0, 1.0);
    let b = p - q*vec2((p.x/p.y).clamp(0.0, 1.0), 1.0);
    let s = -q.y.signum();
    let d = vec2(
        a.length_squared().min(b.length_squared()),
        (s*(p.x*q.y - p.y*p.x)).min(s*(p.y-q.y))
    );
    -d.x.sqrt()*d.y.signum()
}

