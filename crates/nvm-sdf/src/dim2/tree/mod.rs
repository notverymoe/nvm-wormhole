use bytemuck::cast_slice;
use glam::{Vec2, vec2, vec4};

use num_traits::FromPrimitive;
use num_derive::FromPrimitive;

use super::{operation, transform, shape};

#[repr(u32)]
#[derive(FromPrimitive)]
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

pub fn evaluate_tree(tree: &[u32], start: u32, mut point: Vec2) -> f32 {
    let mut result = core::f32::INFINITY;
    for i in (start as usize..tree.len()).step_by(8) {
        let f: &[f32; 6] = cast_slice(&tree[i..i+6]).try_into().unwrap();
        let u: &[u32; 6] = &tree[i..i+6].try_into().unwrap();
        match TreeNodeKind::from_u32(tree[i+7]) {
            Some(TreeNodeKind::TransformAffineTranslate   ) => point  = transform::translate(point, vec2(f[0], f[1])),
            Some(TreeNodeKind::TransformAffineRotate      ) => point  = transform::rotate(point, f[0]),
            Some(TreeNodeKind::TransformAffineRotateDirect) => point  = transform::rotate_direct(point, vec2(f[0], f[1])),
            Some(TreeNodeKind::TransformMirrorAxis        ) => point  = transform::mirror_axis(point, vec2(f[0], f[1]), f[2]),
            Some(TreeNodeKind::TransformMirrorBox         ) => point  = transform::mirror_box(point, vec2(f[0], f[1])),
            Some(TreeNodeKind::TransformMirrorCircle      ) => point  = transform::mirror_circle(point, f[0]),
            Some(TreeNodeKind::TransformRepeatAxis        ) => point  = transform::mirror_axis(point, vec2(f[0], f[1]), f[2]),
            Some(TreeNodeKind::TransformRepeatBox         ) => point  = transform::mirror_box(point, vec2(f[0], f[1])),
            Some(TreeNodeKind::TransformRepeatCircle      ) => point  = transform::mirror_circle(point, f[0]),
            Some(TreeNodeKind::ShapeCircle                ) => result = shape::circle(point, f[0]),
            Some(TreeNodeKind::ShapeCircleSlice           ) => result = shape::circle_slice(point, vec2(f[0], f[1]), f[2]),
            Some(TreeNodeKind::ShapeCircleCut             ) => result = shape::circle_cut(point, f[0], f[1]),
            Some(TreeNodeKind::ShapeCircleArc             ) => result = shape::circle_arc(point, vec2(f[0], f[1]), f[2], f[3]),
            Some(TreeNodeKind::ShapeCircleHorseshoe       ) => result = shape::circle_horseshoe(point, vec2(f[0], f[1]), f[2], vec2(f[3], f[4])),
            Some(TreeNodeKind::ShapeEllipse               ) => result = shape::ellipse(point, vec2(f[0], f[1])),
            Some(TreeNodeKind::ShapeCircleSweep           ) => result = shape::circle_sweep(point, f[0], f[1]),
            Some(TreeNodeKind::ShapeCircleSweepUneven     ) => result = shape::circle_sweep_uneven(point, f[0], f[1], f[2]),
            Some(TreeNodeKind::ShapeSquare                ) => result = shape::square(point, f[0]),
            Some(TreeNodeKind::ShapeRectangle             ) => result = shape::rectangle(point, vec2(f[0], f[1])),
            Some(TreeNodeKind::ShapeRectangleOriented     ) => result = shape::rectangle_oriented(point, vec2(f[0], f[1]), f[2]),
            Some(TreeNodeKind::ShapeRectangleRounded      ) => result = shape::rectangle_rounded(point, vec2(f[0], f[1]), f[2]),
            Some(TreeNodeKind::ShapeRectangleRounded4     ) => result = shape::rectangle_rounded_4(point, vec2(f[0], f[1]), vec4(f[2], f[3], f[4], f[5])),
            Some(TreeNodeKind::ShapeRhombus               ) => result = shape::rhombus(point, vec2(f[0], f[1])),
            Some(TreeNodeKind::ShapeTrapezoidIso          ) => result = shape::trapezoid_iso(point, f[0], f[1], f[2]),
            Some(TreeNodeKind::ShapeParallelogram         ) => result = shape::parallelogram(point, f[0], f[1], f[2]),
            Some(TreeNodeKind::ShapeTriangle              ) => result = shape::triangle(point, f[0], f[1], f[2]),
            Some(TreeNodeKind::ShapeTrianglePnt           ) => result = shape::triangle_pnt(point, vec2(f[0], f[1]), vec2(f[2], f[3]), vec2(f[4], f[5])),
            Some(TreeNodeKind::ShapeTriangleIsosceles     ) => result = shape::trapezoid_iso(point, f[0], f[1], f[2]),
            Some(TreeNodeKind::ShapeTriangleEquilateral   ) => result = shape::triangle_equilateral(point, f[0]),
            Some(TreeNodeKind::ShapeLineSegmentThick      ) => result = shape::line_segment_thick(point, vec2(f[0], f[1]), vec2(f[2], f[3]), f[4]),
            Some(TreeNodeKind::ShapeLineSegmentThin       ) => result = shape::line_segment_thin(point, vec2(f[0], f[1]), vec2(f[2], f[3])),
            Some(TreeNodeKind::ShapeParabola              ) => result = shape::parabola(point, f[0]),
            Some(TreeNodeKind::ShapeParabolaSegment       ) => result = shape::parabola_segment(point, f[0], f[1]),
            Some(TreeNodeKind::ShapeBezierSegment         ) => result = shape::bezier_segment(point, vec2(f[0], f[1]), vec2(f[2], f[3]), vec2(f[4], f[5])),
            Some(TreeNodeKind::ShapePentagon              ) => result = shape::pentagon(point, f[0]),
            Some(TreeNodeKind::ShapeHexagon               ) => result = shape::hexagon(point, f[0]),
            Some(TreeNodeKind::ShapeOctogon               ) => result = shape::octogon(point, f[0]),
            Some(TreeNodeKind::ShapePolygonN              ) => result = shape::polygon_n(point, f[0], f[1]),
            Some(TreeNodeKind::ShapeStarN                 ) => result = shape::polystar_n(point, f[0], f[1], f[2]),
            Some(TreeNodeKind::ShapePolygonHull           ) => result = { let (from, len) = (u[0] as usize, u[1] as usize); shape::polygon_hull(point, cast_slice(&tree[from..(from+len)])) },
            Some(TreeNodeKind::OperationUnaryNegate       ) => result = operation::unary_negate(result),
            Some(TreeNodeKind::OperationUnaryOffset       ) => result = operation::unary_offset(result, f[0]),
            Some(TreeNodeKind::OperationBooleanUnion      ) => result = operation::boolean_union(result, evaluate_tree(tree, u[0], point)),
            Some(TreeNodeKind::OperationBooleanIntersect  ) => result = operation::boolean_intersect(result, evaluate_tree(tree, u[0], point)),
            Some(TreeNodeKind::OperationBooleanDifference ) => result = operation::boolean_difference(result, evaluate_tree(tree, u[0], point)),
            Some(TreeNodeKind::OperationBooleanInterpolate) => result = operation::boolean_interpolate(result, evaluate_tree(tree, u[0], point), f[1]),
            None                                            => unreachable!(),
        }
    }
    result
}
