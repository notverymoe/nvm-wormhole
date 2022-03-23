
#[repr(u8)]
pub enum NodeType {
    OperationUnary = 0,
    OperationBoolean,
    OperationTransform,
    OperationShape,
}

#[repr(u8)]
pub enum NodeOperationUnary {
    OperationUnaryNegate = 0,
    OperationUnaryOffset,
}

#[repr(u8)] 
pub enum NodeOperationBoolean {
    OperationBooleanUnion = 0,
    OperationBooleanIntersect,
    OperationBooleanDifference,
    OperationBooleanInterpolate,
}

#[repr(u8)]
pub enum NodeTransform {
    TransformAffineTranslate = 0,
    TransformAffineRotate,
    TransformAffineRotateDirect,

    TransformMirrorAxis,
    TransformMirrorBox,
    TransformMirrorCircle,

    TransformRepeatAxis,
    TransformRepeatBox,
    TransformRepeatCircle,
}

#[repr(u8)]
pub enum NodeShape {
    ShapeCircle = 0,
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
}
