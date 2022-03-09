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

    // SUBQ	[16(AX)], [16(SP)]
    ($directives:ident, SUBQ [$($left:tt)+] , [$($right:tt)*]; $($rest:tt)*) => {{
        let (left, right) = binary_op!([$($left)+], [$($right)+]);
        $directives.push($crate::Directive::Subq(left, right));
        directives_inner!($directives, $($rest)*);
    }};

    // CMPQ	[16(AX)], [16(SP)]
    ($directives:ident, CMPQ [$($left:tt)+] , [$($right:tt)*]; $($rest:tt)*) => {{
        let (left, right) = binary_op!([$($left)+], [$($right)+]);
        $directives.push($crate::Directive::Cmpq(left, right));
        directives_inner!($directives, $($rest)*);
    }};

    // PCDATA #0, #-2
    ($directives:ident, PCDATA #$left:expr, #$right:expr; $($rest:tt)*) => {{
        $directives.push($crate::Directive::PCData(operand!($left), operand!($right)));
        directives_inner!($directives, $($rest)*);
    }};

    // CALL runtime.morestack_noctxt;
    ($directives:ident, CALL $package:ident . $name:ident; $($rest:tt)*) => {{
        let call_directive = call_inner!($package.$name);
        $directives.push(call_directive);
        directives_inner!($directives, $($rest)*);
    }};

    // @body:
    ($directives:ident, @ $label_name:ident : $($rest:tt)*) => {{
        let label_name = std::stringify!($label_name).to_string();
        $directives.push(Directive::Label(label_name));
        directives_inner!($directives, $($rest)*);
    }};

    ($directives:ident,) => {};
    () => {};
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! call_inner {
    ($package:ident.$name:ident) => {
        $crate::Directive::Call {
            package: std::stringify!($package).to_string(),
            name: std::stringify!($name).to_string(),
        }
    };
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
    ($register_variant:ident) => {
        $crate::operand::Operand::RegisterWithOffset(new_register_with_offset!($register_variant))
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
    ($register_variant:ident) => {
        $crate::register_with_offset::RegisterWithOffset {
            offset: 0,
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
    fn subq() {
        assert_eq!(
            directives!(SUBQ [16(SP)], [32(AX)];),
            vec![Directive::Subq(
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
            directives!(NOP; SUBQ [16(SP)], [32(AX)]; NOP;),
            vec![
                Directive::Nop,
                Directive::Subq(
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
    fn call() {
        assert_eq!(
            directives!(
                CALL runtime.morestack_noctxt;
                NOP;
            ),
            vec![
                Directive::Call {
                    package: "runtime".to_string(),
                    name: "morestack_noctxt".to_string(),
                },
                Directive::Nop
            ]
        )
    }

    #[test]
    fn cmpq() {
        assert_eq!(
            directives!(
                CMPQ [SP], [16(R14)];
            ),
            vec![Directive::Cmpq(
                Operand::RegisterWithOffset(RegisterWithOffset {
                    register: crate::Register::SP,
                    offset: 0
                }),
                Operand::RegisterWithOffset(RegisterWithOffset {
                    register: crate::Register::R14,
                    offset: 16
                }),
            )]
        )
    }

    #[test]
    fn label() {
        assert_eq!(
            directives!(
                JMP @epi;
                @epi:
            ),
            vec![
                Directive::Jmp(JmpTarget::Label("epi".to_string())),
                Directive::Label("epi".to_string())
            ]
        )
    }

    #[test]
    fn call_inner() {
        assert_eq!(
            call_inner!(runtime.morestack_noctxt),
            Directive::Call {
                package: "runtime".to_string(),
                name: "morestack_noctxt".to_string()
            }
        )
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

        assert_eq!(
            new_register_with_offset!(SP),
            crate::RegisterWithOffset {
                offset: 0,
                register: crate::Register::SP
            }
        );
    }
}
