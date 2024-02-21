use crate::{AlgebraicType, AlgebraicValue, ArrayType, BuiltinType, MapType, ProductType, ProductValue};
use spacetimedb_primitives::{ColId, ConstraintId, IndexId, SequenceId, TableId};

impl crate::Value for AlgebraicValue {
    type Type = AlgebraicType;
}

impl From<Vec<AlgebraicValue>> for ProductValue {
    fn from(elements: Vec<AlgebraicValue>) -> Self {
        ProductValue { elements }
    }
}

impl<const N: usize> From<[AlgebraicValue; N]> for ProductValue {
    fn from(fields: [AlgebraicValue; N]) -> Self {
        Vec::from(fields).into()
    }
}

impl From<AlgebraicValue> for ProductValue {
    fn from(x: AlgebraicValue) -> Self {
        [x].into()
    }
}

impl From<AlgebraicType> for ProductType {
    fn from(x: AlgebraicType) -> Self {
        Self::new([x.into()].into())
    }
}

impl From<ArrayType> for AlgebraicType {
    fn from(x: ArrayType) -> Self {
        BuiltinType::Array(x).into()
    }
}

impl From<MapType> for AlgebraicType {
    fn from(x: MapType) -> Self {
        BuiltinType::Map(Box::new(x)).into()
    }
}

macro_rules! built_in_into {
    ($native:ty, $kind:ident) => {
        impl From<$native> for AlgebraicValue {
            fn from(x: $native) -> Self {
                Self::$kind(x.into())
            }
        }
    };
}

built_in_into!(f32, F32);
built_in_into!(f64, F64);
built_in_into!(&str, String);
built_in_into!(&[u8], Bytes);

macro_rules! system_id {
    ($name:ident) => {
        impl From<$name> for AlgebraicValue {
            fn from(value: $name) -> Self {
                value.0.into()
            }
        }
    };
}
system_id!(TableId);
system_id!(ColId);
system_id!(SequenceId);
system_id!(IndexId);
system_id!(ConstraintId);
