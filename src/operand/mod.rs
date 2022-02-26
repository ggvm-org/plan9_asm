use std::fmt;

use crate::register_with_offset::Register;

use super::register_with_offset::RegisterWithOffset;
#[derive(Debug, PartialEq)]
pub enum Operand {
    Ident(String),
    Int(i64),
    RegisterWithOffset(RegisterWithOffset),
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Operand::Ident(s) => s.clone(),
            Operand::Int(n) => format!("${n}"),
            Operand::RegisterWithOffset(inner) => inner.to_string(),
        };
        write!(f, "{s}")
    }
}

macro_rules! impl_from_operand {
    ($from_ty:ty => Ident) => {
        impl From<$from_ty> for Operand {
            fn from(v: $from_ty) -> Self {
                Self::Ident(v.to_string())
            }
        }
    };
    ($from_ty:ty => RegisterWithOffset) => {
        impl From<$from_ty> for Operand {
            fn from(v: $from_ty) -> Self {
                Self::RegisterWithOffset(crate::register_with_offset::RegisterWithOffset::from(v))
            }
        }
    };
    ($from_ty:ty => $variant:ident) => {
        impl From<$from_ty> for Operand {
            fn from(v: $from_ty) -> Self {
                Self::$variant(v)
            }
        }
    };
}

impl_from_operand!(i64 => Int);
impl_from_operand!(&str => Ident);
impl_from_operand!(String => Ident);
impl_from_operand!(RegisterWithOffset => RegisterWithOffset);
impl_from_operand!(Register => RegisterWithOffset);

#[macro_export(local_inner_macros)]
macro_rules! operand {
    ($offset:expr => $register_variant:ident) => {
        $crate::operand::Operand::RegisterWithOffset(crate::register_with_offset!($offset => $register_variant))
    };
    ($register:ident) => {
       $crate::operand::Operand::RegisterWithOffset(crate::register_with_offset!($register))
    };
    ($expr:expr) => {
        $crate::operand::Operand::from($expr)
    };
}

snapshot_test!(operand: operand!(AX), operand!(16=>SP), operand!(1));
