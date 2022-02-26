use std::fmt;

pub use self::jmp_target::JmpTarget;
#[macro_use]
mod common;

#[macro_use]
pub mod operand;
#[macro_use]
pub mod register_with_offset;

#[macro_use]
pub mod macros;

use operand::Operand;

#[macro_use]
pub mod jmp_target;

pub use operand::*;
pub use register_with_offset::*;
pub use Directive::{Nop, Ret};

#[derive(Debug, PartialEq)]
pub enum Directive {
    Text { package: String, name: String },
    Subq(Operand, Operand),
    Leaq(Operand, Operand),
    Movq(Operand, Operand),
    Call { package: String, name: String },
    Addq(Operand, Operand),
    Ret,

    // CMPQ	SP, 16(R14)
    Cmpq(Operand, Operand),
    // PCDATA	$0, $-2
    PCData(Operand, Operand),

    // epi:
    Label(String),
    // NOP
    Nop,
    // JMP body
    Jmp(JmpTarget),
    // Jls	epi
    Jls(JmpTarget),
}

impl fmt::Display for Directive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Text { package, name } => format!("TEXT	{}.{}(SB), 4, $0-0", package, name),
            Self::Subq(left, right) => format!("SUBQ	{}, {}", left, right),
            Self::Call { package, name } => format!("CALL    {package}·{name}(SB)"),
            Self::Addq(left, right) => format!("ADDQ	{}, {}", left, right),
            Self::Movq(left, right) => format!("MOVQ	{}, {}", left, right),
            Self::Cmpq(left, right) => format!("CMPQ	{}, {}", left, right),
            Self::Jls(target) => format!("JLS	{}", target),
            Self::PCData(left, right) => format!("PCDATA {}, {}", left, right),
            Self::Label(label_name) => format!("{}:", label_name),
            Self::Jmp(target) => format!("JMP	{}", target),
            Self::Nop => "NOP".to_string(),
            _ => unimplemented!(),
        };
        write!(f, "{s}")
    }
}

impl fmt::Display for JmpTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Addr(s) => s.to_string(),
            Self::Label(l) => l.to_string(),
        };
        write!(f, "{s}")
    }
}
