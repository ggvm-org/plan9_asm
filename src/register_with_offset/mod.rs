use std::fmt;

pub mod register;

pub use register::Register;

#[derive(Debug, PartialEq)]
pub struct RegisterWithOffset {
    pub offset: usize,
    pub register: Register,
}

impl fmt::Display for RegisterWithOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = {
            let offset = self.offset;
            let register = self.register;
            if offset == 0 {
                register.to_string()
            } else {
                format!("{offset}({register})")
            }
        };
        write!(f, "{s}")
    }
}

impl From<Register> for RegisterWithOffset {
    fn from(register: Register) -> Self {
        RegisterWithOffset {
            offset: 0,
            register,
        }
    }
}
