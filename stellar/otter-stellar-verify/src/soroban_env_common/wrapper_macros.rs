#[doc(hidden)]
#[macro_export]
macro_rules! declare_tag_based_wrapper {
    ($T:ident) => {
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct $T($crate::Val);
    };
}
