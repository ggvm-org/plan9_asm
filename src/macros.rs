#[macro_export(local_inner_macros)]
macro_rules! directive {
    (NOP) => {
        $crate::Directive::Nop
    };
    (RET) => {
        $crate::Directive::Ret
    };
    (PCDATA $left:expr, $right:expr) => {
        PCDATA!($left, $right)
    };
    (CALL $package:ident.$name:ident) => {
        CALL!($package.$name)
    };
    (JMP @$target:tt) => {
        JMP!(@$target)
    };
    (JMP $target:tt) => {
        JMP!($target)
    };
    (@$label_name:ident) => {
       $crate::Directive::Label(std::stringify!($label_name).to_string())
    };
}

#[macro_export(local_inner_macros)]
macro_rules! PCDATA {
    ($left:expr, $right:expr) => {
        $crate::Directive::PCData(operand!($left), operand!($right))
    };
}

#[macro_export(local_inner_macros)]
macro_rules! ADDQ {
    ($left_op:tt, $right_op:tt) => {
        $crate::Directive::Addq($crate::operand!($left_op), $crate::operand!($right_op))
    };
    ($left_offset:tt => $left_op:tt, $right_op:tt) => {
        $crate::Directive::Addq($crate::operand!($left_offset => $left_op), $crate::operand!($right_op))
    };
    ($left_offset:tt => $left_op:tt, $right_offset:tt => $right_op:tt) => {
        $crate::Directive::Addq($crate::operand!($left_offset => $left_op), $crate::operand!($right_offset => $right_op))
    };
    ($left_op:tt, $right_offset:tt => $right_op:tt) => {
        $crate::Directive::Addq($crate::operand!($left_op), $crate::operand!($right_offset => $right_op))
    };
}

#[macro_export(local_inner_macros)]
macro_rules! SUBQ {
    ($left_op:tt, $right_op:tt) => {
        $crate::Directive::Subq($crate::operand!($left_op), $crate::operand!($right_op))
    };
    ($left_offset:tt => $left_op:tt, $right_op:tt) => {
        $crate::Directive::Subq($crate::operand!($left_offset => $left_op), $crate::operand!($right_op))
    };
    ($left_offset:tt => $left_op:tt, $right_offset:tt => $right_op:tt) => {
        $crate::Directive::Subq($crate::operand!($left_offset => $left_op), $crate::operand!($right_offset => $right_op))
    };
    ($left_op:tt, $right_offset:tt => $right_op:tt) => {
        $crate::Directive::Subq($crate::operand!($left_op), $crate::operand!($right_offset => $right_op))
    };
}

#[macro_export(local_inner_macros)]
macro_rules! CMPQ {
    ($left_op:tt, $right_op:tt) => {
        $crate::Directive::Cmpq($crate::operand!($left_op), $crate::operand!($right_op))
    };
    ($left_offset:tt => $left_op:tt, $right_op:tt) => {
        $crate::Directive::Cmpq($crate::operand!($left_offset => $left_op), $crate::operand!($right_op))
    };
    ($left_offset:tt => $left_op:tt, $right_offset:tt => $right_op:tt) => {
        $crate::Directive::Cmpq($crate::operand!($left_offset => $left_op), $crate::operand!($right_offset => $right_op))
    };
    ($left_op:tt, $right_offset:tt => $right_op:tt) => {
        $crate::Directive::Cmpq($crate::operand!($left_op), $crate::operand!($right_offset => $right_op))
    };
}

#[macro_export(local_inner_macros)]
macro_rules! LEAQ {
    ($left_op:tt, $right_op:tt) => {
        $crate::Directive::Leaq($crate::operand!($left_op), $crate::operand!($right_op))
    };
    ($left_offset:tt => $left_op:tt, $right_op:tt) => {
        $crate::Directive::Leaq($crate::operand!($left_offset => $left_op), $crate::operand!($right_op))
    };
    ($left_offset:tt => $left_op:tt, $right_offset:tt => $right_op:tt) => {
        $crate::Directive::Leaq($crate::operand!($left_offset => $left_op), $crate::operand!($right_offset => $right_op))
    };
    ($left_op:tt, $right_offset:tt => $right_op:tt) => {
        $crate::Directive::Leaq($crate::operand!($left_op), $crate::operand!($right_offset => $right_op))
    };
}

#[macro_export(local_inner_macros)]
macro_rules! MOVQ {
    ($left_op:tt, $right_op:tt) => {
        $crate::Directive::Movq($crate::operand!($left_op), $crate::operand!($right_op))
    };
    ($left_offset:tt => $left_op:tt, $right_op:tt) => {
        $crate::Directive::Movq($crate::operand!($left_offset => $left_op), $crate::operand!($right_op))
    };
    ($left_offset:tt => $left_op:tt, $right_offset:tt => $right_op:tt) => {
        $crate::Directive::Movq($crate::operand!($left_offset => $left_op), $crate::operand!($right_offset => $right_op))
    };
    ($left_op:tt, $right_offset:tt => $right_op:tt) => {
        $crate::Directive::Movq($crate::operand!($left_op), $crate::operand!($right_offset => $right_op))
    };
}
// macro_rules! define_binary_directive {
//     ($macro_name:ident, $variant:ident) => {
//         #[macro_export(local_inner_macros)]
//         macro_rules! $macro_name {
//             ($left_op:tt, $right_op:tt) => {
//                 $crate::Directive::$variant($crate::operand!($left_op), $crate::operand!($right_op))
//             };
//             ($left_offset:tt => $left_op:tt, $right_op:tt) => {
//                 $crate::Directive::$variant($crate::operand!($left_offset => $left_op), $crate::operand!($right_op))
//             };
//             ($left_offset:tt => $left_op:tt, $right_offset:tt => $right_op:tt) => {
//                 $crate::Directive::$variant($crate::operand!($left_offset => $left_op), $crate::operand!($right_offset => $right_op))
//             };
//             ($left_op:tt, $right_offset:tt => $right_op:tt) => {
//                 $crate::Directive::$variant($crate::operand!($left_op), $crate::operand!($right_offset => $right_op))
//             };
//         }
//     };
// }

// define_binary_directive!(SUBQ, Subq);
// define_binary_directive!(CMPQ, Cmpq);
// define_binary_directive!(LEAQ, Leaq);
// define_binary_directive!(MOVQ, Movq);

#[macro_export(local_inner_macros)]
macro_rules! CALL {
    ($package:ident.$name:ident) => {
        $crate::Directive::Call {
            package: std::stringify!($package).to_string(),
            name: std::stringify!($name).to_string(),
        }
    };
}

// macro_rules! define_jmp_macro {
//     ($macro_name:ident, $variant:ident) => {
//         #[macro_export(local_inner_macros)]
//         macro_rules! $macro_name {
//             ($target:expr) => {
//                 $crate::Directive::$variant($crate::jmp_target::JmpTarget::from($target))
//             };
//             (@$label:ident) => {
//                 $crate::Directive::$variant($crate::jmp_target::JmpTarget::from(std::stringify!(
//                     $label
//                 )))
//             };
//         }
//     };
// }

// define_jmp_macro!(JMP, Jmp);
// define_jmp_macro!(JLS, Jls);

#[macro_export(local_inner_macros)]
macro_rules! JMP {
    ($($tt:tt)+) => {
        $crate::Directive::Jmp(jmp_target!($($tt)+))
    };
}

#[macro_export(local_inner_macros)]
macro_rules! JLS {
    ($($tt:tt)+) => {
        $crate::Directive::Jls(jmp_target!($($tt)+))
    };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! jmp_target {
    (@$label:ident) => {
        $crate::jmp_target::JmpTarget::from(std::stringify!($label))
    };
    ($target:expr) => {
        $crate::jmp_target::JmpTarget::from($target)
    };
}

#[cfg(test)]
mod snapshots {
    use insta::assert_display_snapshot;

    use crate::JmpTarget;

    macro_rules! insta_test {
        ($testname:ident: $($testcases:expr),+) => {
            #[test]
            fn $testname() {
                $(assert_display_snapshot!($testcases);)+
            }
        };
    }

    insta_test!(directive_jmp: directive!(JMP 2), directive!(JMP @body));

    insta_test!(directive_call: directive!(CALL main.run));

    insta_test!(nop: directive!(NOP));
    insta_test!(call: CALL!(main.run));

    const TEST_JMP_TARGET_VAR: &str = "AAAAA";
    insta_test!(
        jmp: JMP!(33),
        JMP!("epi"),
        JMP!(TEST_JMP_TARGET_VAR),
        JMP!(@body)
    );

    insta_test!(
        jls: JLS!(33),
        JLS!("epi"),
        JLS!(TEST_JMP_TARGET_VAR),
        JLS!(@body)
    );

    insta_test!(
        addq: ADDQ!(AX, 1),
        ADDQ!(16=>AX, 1),
        ADDQ!(1, 16=>AX),
        ADDQ!(16=>AX, 16=>SP)
    );

    insta_test!(pcdata: PCDATA!(1, 2));
    insta_test!(directive_pcdata: directive!(PCDATA 1, 2));

    insta_test!(
        subq: SUBQ!(AX, 1),
        SUBQ!(16=>AX, 1),
        SUBQ!(1, 16=>AX),
        SUBQ!(16=>AX, 16=>SP)
    );

    #[test]
    fn assert() {
        assert_eq!(jmp_target!(@body), JmpTarget::Label("body".to_string()))
    }
}
