use glam::{Vec2, vec2, vec4};

#[allow(unused_imports)]
use num_traits::{Float, FromPrimitive};
use num_derive::FromPrimitive;

use super::{operation, transform, shape};

#[repr(u32)]
#[derive(FromPrimitive, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TreeNodeKind {

    TransformAffineTranslate,
    TransformAffineRotate,
    TransformAffineRotateDirect,

    TransformMirrorAxis,
    TransformMirrorBox,
    TransformMirrorCircle,

    TransformRepeatAxis,
    TransformRepeatBox,
    TransformRepeatCircle,

    ShapeCircle,
    ShapeCircleSlice,
    ShapeCircleCut,
    ShapeCircleArc,
    ShapeCircleHorseshoe,
    ShapeEllipse,
    ShapeCircleSweep,
    ShapeCircleSweepUneven,

    ShapeSquare,
    ShapeRectangle,
    ShapeRectangleOriented,
    ShapeRectangleRounded,
    ShapeRectangleRounded4,
    ShapeRhombus,
    ShapeTrapezoidIso,
    ShapeParallelogram,

    ShapeTriangle,
    ShapeTrianglePnt,
    ShapeTriangleIsosceles,
    ShapeTriangleEquilateral,

    ShapeLineSegmentThick,
    ShapeLineSegmentThin,
    ShapeParabola,
    ShapeParabolaSegment,
    ShapeBezierSegment,

    ShapePentagon,
    ShapeHexagon,
    ShapeOctogon,
    ShapePolygonN,
    ShapeStarN,
    ShapePolygonHull,
    
    OperationUnaryNegate,
    OperationUnaryOffset,

    OperationBooleanUnion,
    OperationBooleanIntersect,
    OperationBooleanDifference,
    OperationBooleanInterpolate,
}

macro_rules! get_f {
    ($a:ident, $i:expr, $j:expr) => {
        f32::from_bits($a[$i+$j])
    };
}

macro_rules! get_v2 {
    ($a:ident, $i:expr, $j:expr) => {
        vec2(f32::from_bits($a[$i+$j]), f32::from_bits($a[$i+$j+1]))
    };
}

pub fn evaluate_tree<const MAX_DEPTH: usize>(
    tree:  &[u32], 
    point_buf: &[Vec2], 
    start:  usize, 
    len:    usize,
    point:  Vec2
) -> f32 {

    let mut depth = 0;

    let mut index:  [usize; MAX_DEPTH]        = [0; MAX_DEPTH];
    let mut ends:   [usize; MAX_DEPTH]        = [0; MAX_DEPTH];
    let mut sample: [ f32;  MAX_DEPTH]        = [core::f32::INFINITY; MAX_DEPTH];
    let mut points: [Vec2;  MAX_DEPTH]        = [vec2(0.0,0.0); MAX_DEPTH];
    let mut ops:    [TreeNodeKind; MAX_DEPTH] = [TreeNodeKind::OperationBooleanUnion; MAX_DEPTH];

    index[0]  = start;
    ends[0]   = start + len*8;
    points[0] = point;


    loop {
        let i = index[depth];
        match TreeNodeKind::from_u32(tree[index[depth]]) {
            Some(TreeNodeKind::TransformAffineTranslate       ) => points[depth] = transform::translate(points[depth], get_v2!(tree, i, 0)),
            Some(TreeNodeKind::TransformAffineRotate          ) => points[depth] = transform::rotate(points[depth], get_f!(tree, i, 0)),
            Some(TreeNodeKind::TransformAffineRotateDirect    ) => points[depth] = transform::rotate_direct(points[depth], get_v2!(tree, i, 0)),
            Some(TreeNodeKind::TransformMirrorAxis            ) => points[depth] = transform::mirror_axis(points[depth], get_v2!(tree, i, 0), get_f!(tree, i, 2)),
            Some(TreeNodeKind::TransformMirrorBox             ) => points[depth] = transform::mirror_box(points[depth], get_v2!(tree, i, 0)),
            Some(TreeNodeKind::TransformMirrorCircle          ) => points[depth] = transform::mirror_circle(points[depth], get_f!(tree, i, 0)),
            Some(TreeNodeKind::TransformRepeatAxis            ) => points[depth] = transform::mirror_axis(points[depth], get_v2!(tree, i, 0), get_f!(tree, i, 2)),
            Some(TreeNodeKind::TransformRepeatBox             ) => points[depth] = transform::mirror_box(points[depth], get_v2!(tree, i, 0)),
            Some(TreeNodeKind::TransformRepeatCircle          ) => points[depth] = transform::mirror_circle(points[depth], get_f!(tree, i, 0)),
            Some(TreeNodeKind::ShapeCircle                    ) => sample[depth] = shape::circle(points[depth], get_f!(tree, i, 0)),
            Some(TreeNodeKind::ShapeCircleSlice               ) => sample[depth] = shape::circle_slice(points[depth], get_v2!(tree, i,  0), get_f!(tree, i, 2)),
            Some(TreeNodeKind::ShapeCircleCut                 ) => sample[depth] = shape::circle_cut(points[depth], get_f!(tree, i, 0), get_f!(tree, i, 1)),
            Some(TreeNodeKind::ShapeCircleArc                 ) => sample[depth] = shape::circle_arc(points[depth], get_v2!(tree, i,  0), get_f!(tree, i, 2), get_f!(tree, i, 3)),
            Some(TreeNodeKind::ShapeCircleHorseshoe           ) => sample[depth] = shape::circle_horseshoe(points[depth], get_v2!(tree, i,  0), get_f!(tree, i, 2), get_v2!(tree, i,  3)),
            Some(TreeNodeKind::ShapeEllipse                   ) => sample[depth] = shape::ellipse(points[depth], get_v2!(tree, i,  0)),
            Some(TreeNodeKind::ShapeCircleSweep               ) => sample[depth] = shape::circle_sweep(points[depth], get_f!(tree, i, 0), get_f!(tree, i, 1)),
            Some(TreeNodeKind::ShapeCircleSweepUneven         ) => sample[depth] = shape::circle_sweep_uneven(points[depth], get_f!(tree, i, 0), get_f!(tree, i, 1), get_f!(tree, i, 2)),
            Some(TreeNodeKind::ShapeSquare                    ) => sample[depth] = shape::square(points[depth], get_f!(tree, i, 0)),
            Some(TreeNodeKind::ShapeRectangle                 ) => sample[depth] = shape::rectangle(points[depth], get_v2!(tree, i,  0)),
            Some(TreeNodeKind::ShapeRectangleOriented         ) => sample[depth] = shape::rectangle_oriented(points[depth], get_v2!(tree, i,  0), get_f!(tree, i, 2)),
            Some(TreeNodeKind::ShapeRectangleRounded          ) => sample[depth] = shape::rectangle_rounded(points[depth], get_v2!(tree, i,  0), get_f!(tree, i, 2)),
            Some(TreeNodeKind::ShapeRectangleRounded4         ) => sample[depth] = shape::rectangle_rounded_4(points[depth], get_v2!(tree, i,  0), vec4(get_f!(tree, i, 2), get_f!(tree, i, 3), get_f!(tree, i, 4), get_f!(tree, i, 5))),
            Some(TreeNodeKind::ShapeRhombus                   ) => sample[depth] = shape::rhombus(points[depth], get_v2!(tree, i,  0)),
            Some(TreeNodeKind::ShapeTrapezoidIso              ) => sample[depth] = shape::trapezoid_iso(points[depth], get_f!(tree, i, 0), get_f!(tree, i, 1), get_f!(tree, i, 2)),
            Some(TreeNodeKind::ShapeParallelogram             ) => sample[depth] = shape::parallelogram(points[depth], get_f!(tree, i, 0), get_f!(tree, i, 1), get_f!(tree, i, 2)),
            Some(TreeNodeKind::ShapeTriangle                  ) => sample[depth] = shape::triangle(points[depth], get_f!(tree, i, 0), get_f!(tree, i, 1), get_f!(tree, i, 2)),
            Some(TreeNodeKind::ShapeTrianglePnt               ) => sample[depth] = shape::triangle_pnt(points[depth], get_v2!(tree, i,  0), get_v2!(tree, i,  2), get_v2!(tree, i,  4)),
            Some(TreeNodeKind::ShapeTriangleIsosceles         ) => sample[depth] = shape::trapezoid_iso(points[depth], get_f!(tree, i, 0), get_f!(tree, i, 1), get_f!(tree, i, 2)),
            Some(TreeNodeKind::ShapeTriangleEquilateral       ) => sample[depth] = shape::triangle_equilateral(points[depth], get_f!(tree, i, 0)),
            Some(TreeNodeKind::ShapeLineSegmentThick          ) => sample[depth] = shape::line_segment_thick(points[depth], get_v2!(tree, i,  0), get_v2!(tree, i,  2), get_f!(tree, i, 4)),
            Some(TreeNodeKind::ShapeLineSegmentThin           ) => sample[depth] = shape::line_segment_thin(points[depth], get_v2!(tree, i,  0), get_v2!(tree, i,  2)),
            Some(TreeNodeKind::ShapeParabola                  ) => sample[depth] = shape::parabola(points[depth], get_f!(tree, i, 0)),
            Some(TreeNodeKind::ShapeParabolaSegment           ) => sample[depth] = shape::parabola_segment(points[depth], get_f!(tree, i, 0), get_f!(tree, i, 1)),
            Some(TreeNodeKind::ShapeBezierSegment             ) => sample[depth] = shape::bezier_segment(points[depth], get_v2!(tree, i,  0), get_v2!(tree, i,  2), get_v2!(tree, i,  4)),
            Some(TreeNodeKind::ShapePentagon                  ) => sample[depth] = shape::pentagon(points[depth], get_f!(tree, i, 0)),
            Some(TreeNodeKind::ShapeHexagon                   ) => sample[depth] = shape::hexagon(points[depth], get_f!(tree, i, 0)),
            Some(TreeNodeKind::ShapeOctogon                   ) => sample[depth] = shape::octogon(points[depth], get_f!(tree, i, 0)),
            Some(TreeNodeKind::ShapePolygonN                  ) => sample[depth] = shape::polygon_n(points[depth], get_f!(tree, i, 0), get_f!(tree, i, 1)),
            Some(TreeNodeKind::ShapeStarN                     ) => sample[depth] = shape::polystar_n(points[depth], get_f!(tree, i, 0), get_f!(tree, i, 1), get_f!(tree, i, 2)),
            Some(TreeNodeKind::ShapePolygonHull               ) => sample[depth] = shape::polygon_hull(points[depth], point_buf, tree[i] as usize, tree[i+1] as usize),
            Some(TreeNodeKind::OperationUnaryNegate           ) => sample[depth] = operation::unary_negate(sample[depth]),
            Some(TreeNodeKind::OperationUnaryOffset           ) => sample[depth] = operation::unary_offset(sample[depth], get_f!(tree, i, 0)),
            Some(v @ TreeNodeKind::OperationBooleanUnion      ) |
            Some(v @ TreeNodeKind::OperationBooleanIntersect  ) |
            Some(v @ TreeNodeKind::OperationBooleanDifference ) |
            Some(v @ TreeNodeKind::OperationBooleanInterpolate) => {
                ops[depth]      = v;
                index[depth+1]  = tree[i] as usize;
                ends[depth+1]   = index[depth+1] + (tree[i+1]*8) as usize;
                sample[depth+1] = core::f32::INFINITY;
                depth += 1;
                continue; // No increment!
            },
            None => unreachable!(),
        }

        index[depth] += 8;
        if index[depth] > ends[depth] {
            if depth > 0 {
                sample[depth-1] = match ops[depth-1] {
                    TreeNodeKind::OperationBooleanUnion       => operation::boolean_union(sample[depth-1], sample[depth]),
                    TreeNodeKind::OperationBooleanIntersect   => operation::boolean_intersect(sample[depth-1], sample[depth]),
                    TreeNodeKind::OperationBooleanDifference  => operation::boolean_difference(sample[depth-1], sample[depth]),
                    TreeNodeKind::OperationBooleanInterpolate => operation::boolean_interpolate(sample[depth-1], sample[depth], get_f!(tree, index[depth-1], 2)),
                    _ => unreachable!(),
                };
                depth -= 1;
                index[depth] += 8;
            } else {
                break;
            }
        }
    }

    sample[0]
}
