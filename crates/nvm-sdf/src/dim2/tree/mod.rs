use num_derive::FromPrimitive;
use num_traits::FromPrimitive;


#[repr(u8)]
#[derive(FromPrimitive)]
pub enum NodeType {
    OperationUnary = 0,
    OperationBoolean,
    OperationTransform,
    OperationShape,
}

#[repr(u8)]
#[derive(FromPrimitive)]
pub enum NodeOperationUnary {
    OperationUnaryNegate = 0,
    OperationUnaryOffset,
}

#[repr(u8)] 
#[derive(FromPrimitive)]
pub enum NodeOperationBoolean {
    OperationBooleanUnion = 0,
    OperationBooleanIntersect,
    OperationBooleanDifference,
    OperationBooleanInterpolate,
}

#[repr(u8)]
#[derive(FromPrimitive)]
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
#[derive(FromPrimitive)]
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

pub trait Reader {
    fn read_packet_header(data: &[u8], idx: u32) -> [u8; 2];
    fn read_packet_body_u32(data: &[u8], idx: u32, arg: u32) -> u32;
    fn read_packet_body_f32(data: &[u8], idx: u32, arg: u32) -> f32;
}

pub fn process<T: Reader>(data: &[u8], idx: u32) -> f32 {
    let [kind, subKind] = T::read_packet_header(data, idx);
    match NodeType::from_u8(kind).unwrap() {
        NodeType::OperationUnary     => process_operation_unary::<T>(subKind, data, idx),
        NodeType::OperationBoolean   => process_operation_boolean::<T>(subKind, data, idx),
        NodeType::OperationTransform => process_operation_transform::<T>(subKind, data, idx),
        NodeType::OperationShape     => process_operation_shape::<T>(subKind, data, idx),
    }
}

pub fn process_operation_unary<T: Reader>(kind: u8, data: &[u8], idx: u32) -> f32 {
    // Transforms return
    match NodeOperationUnary::from_u8(kind).unwrap() {

    }
}

pub fn process_operation_boolean<T: Reader>(kind: u8, data: &[u8], idx: u32) -> f32 {
    // Transforms two returns
    match NodeOperationBoolean::from_u8(kind).unwrap() {

    }
}

pub fn process_operation_transform<T: Reader>(kind: u8, data: &[u8], idx: u32) -> f32 {
    // Transforms
    match NodeOperationTransform::from_u8(kind).unwrap() {

    }
}

pub fn process_operation_shape<T: Reader>(kind: u8, data: &[u8], idx: u32) -> f32 {
    match NodeOperationShape::from_u8(kind).unwrap() {

    }
}