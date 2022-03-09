#[macro_export(local_inner_macros)]
macro_rules! directives {
    ($($tt:tt)+) => {{
        let mut d: Vec<Directive> = Vec::new();
        directives_inner!(d, $($tt)+);
        d
    }};
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! directives_inner {
    ($directives:ident, NOP; $($rest:tt)*) => {
        $directives.push($crate::Directive::Nop);
        directives_inner!($directives, $($rest)*)
    };

    // TODO: refactorable?
    ($directives:ident, JMP $tt:expr; $($rest:tt)*) => {
        $directives.push(JMP!($tt));
        directives_inner!($directives, $($rest)*)
    };
    ($directives:ident, JMP @$target:ident; $($rest:tt)*) => {
        $directives.push(JMP!(@$target));
        directives_inner!($directives, $($rest)*)
    };
    // ($directives:ident, JMP $($tt:tt)+; $($rest:tt)*) => {
    //     $directives.push(JMP!($($tt)+));
    //     directives_inner!($directives, $($rest)*)
    // };

    // TODO: refactorable?
    ($directives:ident, JLS $tt:expr; $($rest:tt)*) => {
        $directives.push(JLS!($tt));
        directives_inner!($directives, $($rest)*)
    };
    ($directives:ident, JLS @$target:ident; $($rest:tt)*) => {
        $directives.push(JLS!(@$target));
        directives_inner!($directives, $($rest)*)
    };

    // ADDQ	[16(AX)], [16(SP)]
    ($directives:ident, ADDQ [$($left:tt)+] , [$($right:tt)*]; $($rest:tt)*) => {{
        let (left, right) = binary_op!([$($left)+], [$($right)+]);
        $directives.push($crate::Directive::Addq(left, right));
        directives_inner!($directives, $($rest)*);
    }};

    // PCDATA #0, #-2
    ($directives:ident, PCDATA #$left:expr, #$right:expr; $($rest:tt)*) => {{
        $directives.push($crate::Directive::PCData(operand!($left), operand!($right)));
        directives_inner!($directives, $($rest)*);
    }};
    ($directives:ident,) => {};
    () => {};
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! binary_op {
    ([$($left:tt)+] , [$($right:tt)*]) => {{
        let left_op = new_operand!($($left)+);
        let right_op = new_operand!($($right)*);
       (left_op, right_op)
    }};
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! new_operand {
    ($offset:tt($register_variant:ident)) => {
        $crate::operand::Operand::RegisterWithOffset(new_register_with_offset!($offset(
            $register_variant
        )))
    };
    ($lit:expr) => {
        $crate::operand::Operand::from($lit)
    };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! new_register_with_offset {
    ($offset:tt($register_variant:ident)) => {
        $crate::register_with_offset::RegisterWithOffset {
            offset: $offset,
            register: $crate::register_with_offset::Register::$register_variant,
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::{operand::Operand, Directive, JmpTarget, RegisterWithOffset};

    #[test]
    fn directives_inner() {
        assert_eq!(directives!(NOP;), vec![crate::Directive::Nop]);
        assert_eq!(
            directives!(NOP;NOP;),
            vec![crate::Directive::Nop, crate::Directive::Nop]
        );
        assert_eq!(
            directives!(JMP 1;),
            vec![crate::Directive::Jmp(JmpTarget::Addr(1))]
        );
        assert_eq!(
            directives!(JMP @body;),
            vec![crate::Directive::Jmp(JmpTarget::Label("body".to_string()))]
        );
        assert_eq!(
            directives!(NOP;JMP 1; JMP @body;),
            vec![
                crate::Directive::Nop,
                crate::Directive::Jmp(JmpTarget::Addr(1)),
                crate::Directive::Jmp(JmpTarget::Label("body".to_string()))
            ]
        );
    }

    #[test]
    fn jls() {
        assert_eq!(
            directives!(JLS 1;),
            vec![crate::Directive::Jls(JmpTarget::Addr(1))]
        );

        assert_eq!(
            directives!(JLS @body;),
            vec![crate::Directive::Jls(JmpTarget::Label("body".to_string()))]
        );
    }

    #[test]
    fn addq() {
        assert_eq!(
            directives!(ADDQ [16(SP)], [32(AX)];),
            vec![Directive::Addq(
                Operand::RegisterWithOffset(RegisterWithOffset {
                    offset: 16,
                    register: crate::Register::SP
                }),
                Operand::RegisterWithOffset(RegisterWithOffset {
                    offset: 32,
                    register: crate::Register::AX
                })
            )]
        );

        assert_eq!(
            directives!(NOP; ADDQ [16(SP)], [32(AX)]; NOP;),
            vec![
                Directive::Nop,
                Directive::Addq(
                    Operand::RegisterWithOffset(RegisterWithOffset {
                        offset: 16,
                        register: crate::Register::SP
                    }),
                    Operand::RegisterWithOffset(RegisterWithOffset {
                        offset: 32,
                        register: crate::Register::AX
                    })
                ),
                Directive::Nop,
            ]
        )
    }

    #[test]
    fn pcdata() {
        let right = -2;
        assert_eq!(
            directives!(PCDATA #0, #right; NOP;),
            vec![
                Directive::PCData(Operand::Int(0), Operand::Int(right)),
                Directive::Nop,
            ]
        )
    }
    #[test]
    fn binary_op() {
        assert_eq!(
            binary_op!([16(SP)], [32(AX)]),
            (
                Operand::RegisterWithOffset(RegisterWithOffset {
                    offset: 16,
                    register: crate::Register::SP
                }),
                Operand::RegisterWithOffset(RegisterWithOffset {
                    offset: 32,
                    register: crate::Register::AX
                })
            )
        );
    }

    #[test]
    fn new_operand() {
        assert_eq!(
            new_operand!(16(SP)),
            Operand::RegisterWithOffset(crate::RegisterWithOffset {
                offset: 16,
                register: crate::Register::SP
            })
        );
    }

    #[test]
    fn new_register_with_offset() {
        assert_eq!(
            new_register_with_offset!(16(SP)),
            crate::RegisterWithOffset {
                offset: 16,
                register: crate::Register::SP
            }
        );
    }
}
