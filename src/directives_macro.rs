#[macro_export(local_inner_macros)]
macro_rules! directives {
    ($($tt:tt)+) => {{
        let mut d = Vec::new();
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
    ($directives:ident,) => {};
    () => {};
}

#[cfg(test)]
mod tests {
    use crate::JmpTarget;

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
}
