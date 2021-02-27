use std::panic::{catch_unwind, AssertUnwindSafe, UnwindSafe};

pub fn test_each<'a, T, A>(test_cases: Vec<A>, test: T)
where
    T: Clone + Fn(A) -> () + UnwindSafe,
    A: UnwindSafe,
{
    for args in test_cases {
        let test_clone = AssertUnwindSafe(test.clone());
        let result = catch_unwind(|| test_clone(args));
        assert!(result.is_ok())
    }
}
