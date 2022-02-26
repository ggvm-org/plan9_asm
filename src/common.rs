#[macro_export]
macro_rules! snapshot_test {
    ($($testname:ident: $($testcases:expr),+)+) => {
        #[cfg(test)]
        mod snapshots {
            $(insta_test!($testname: $($testcases),+);)+
        }
    };
}

#[macro_export]
macro_rules! insta_test {
    ($testname:ident: $($testcases:expr),+) => {
        #[test]
        fn $testname() {
            use insta::assert_display_snapshot;
            $(assert_display_snapshot!($testcases);)+
        }
    };
}
