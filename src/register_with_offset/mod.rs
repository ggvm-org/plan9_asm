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

#[macro_export(local_inner_macros)]
macro_rules! register_with_offset {
    // TODO: `=> $register_variant` to `($register_variant)`
    ($offset:expr => $register_variant:ident) => {
        $crate::register_with_offset::RegisterWithOffset {
            offset: $offset,
            register: $crate::register_with_offset::Register::$register_variant,
        }
    };
    ($variant:ident) => {
        $crate::register_with_offset::RegisterWithOffset {
            offset: 0,
            register: $crate::register_with_offset::Register::$variant,
        }
    }; // (1()2) => {};
}

#[cfg(test)]
mod snapshots {
    use insta::assert_display_snapshot;

    macro_rules! insta_test {
        ($testname:ident: $($testcases:expr),+) => {
            #[test]
            fn $testname() {
                $(assert_display_snapshot!($testcases);)+
            }
        };
    }

    const TEST_OFFSET: usize = 10;

    insta_test!(
        register_with_offset: register_with_offset!(AX),
        register_with_offset!(16=>SP),
        register_with_offset!(TEST_OFFSET=>SP)
    );
}

// register_with_offset!(1()2);
