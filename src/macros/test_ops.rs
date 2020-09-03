#[macro_export]
macro_rules! test_ops {
    ($lhs:ty, $construction:expr) => {
        concat_idents::concat_idents!(test_negation = neg_, $lhs {
            #[test]
            fn test_negation() {
                let lhs = $construction(10.0);
                assert_eq!(-lhs, $construction(-10.0));
            }
        });
    };
    ($lhs:ty, $lhs_construction:expr, Percent) => {
        test_ops!(add, add_assign, $lhs, $lhs_construction, Percent);
        test_ops!(sub, sub_assign, $lhs, $lhs_construction, Percent);
        test_ops!(mul, mul_assign, $lhs, $lhs_construction, Percent, Percent::new);
        test_ops!(div, div_assign, $lhs, $lhs_construction, Percent, Percent::new);
        test_ops!(rem, rem_assign, $lhs, $lhs_construction, Percent, Percent::new);
    };
    ($lhs:ty, $lhs_construction:expr, $rhs:ty, $rhs_construction:expr) => {
        test_ops!(add, add_assign, $lhs, $lhs_construction, $rhs, $rhs_construction);
        test_ops!(sub, sub_assign, $lhs, $lhs_construction, $rhs, $rhs_construction);
        test_ops!(mul, mul_assign, $lhs, $lhs_construction, $rhs, $rhs_construction);
        test_ops!(div, div_assign, $lhs, $lhs_construction, $rhs, $rhs_construction);
        test_ops!(rem, rem_assign, $lhs, $lhs_construction, $rhs, $rhs_construction);
    };
    ($binary_operation:ident, $assign_operation:ident, $lhs:ty, $lhs_construction:expr, $rhs:ty, $rhs_construction:expr) => {
        concat_idents::concat_idents!(test_ops = $binary_operation, _, $lhs, _, $rhs {
            #[test]
            fn test_ops() {for _ in 0..*TEST_ROUNDS {
                let lhs_value = rand::random();
                let rhs_value = rand::random();

                let mut lhs = $lhs_construction(lhs_value);
                let rhs = $rhs_construction(rhs_value);

                let correct_res = lhs_value.$binary_operation(rhs_value);
                let op_res = lhs.$binary_operation(rhs);
                lhs.$assign_operation(rhs);

                assert_eq!(op_res, lhs);
                assert_eq!(op_res, $lhs_construction(correct_res));
            }}
        });
    };
    ($binary_operation:ident, $assign_operation:ident, $lhs:ty, $lhs_construction:expr, Percent) => {
        concat_idents::concat_idents!(test_ops = $binary_operation, _, $lhs, _, Percent {
            #[test]
            fn test_ops() {for _ in 0..*TEST_ROUNDS {
                let lhs_value = rand::random();
                let rhs_value = rand::random();

                let mut lhs = $lhs_construction(lhs_value);
                let rhs = Percent::new(rhs_value);

                let correct_res = lhs_value.$binary_operation( lhs_value * rhs_value );
                let op_res = lhs.$binary_operation(rhs);
                lhs.$assign_operation(rhs);

                assert_eq!(op_res, lhs);
                assert_eq!(op_res, $lhs_construction(correct_res));
            }}
        });
    };
}
