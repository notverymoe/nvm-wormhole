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

fn ndot(a: glam::Vec2, b: glam::Vec2) -> f32 {
    a.x*b.x - a.y*b.y
}

mod circle;
pub use circle::*;

mod line;
pub use line::*;

mod misc;
pub use misc::*;

mod polygon;
pub use polygon::*;

mod rect;
pub use rect::*;

mod triangle;
pub use triangle::*;

