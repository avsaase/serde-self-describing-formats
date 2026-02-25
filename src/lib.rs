#![allow(dead_code)]

mod formats;
mod roundtrip;
mod schema_evolution;

#[macro_export]
macro_rules! test_format_impl {
    ([$($impl_type:ty),*], $function:ident) => {
        $(
            paste::paste! {
                #[test]
                fn [<$function _$impl_type:snake>]() {
                    $function::<$impl_type>();
                }
            }
        )*
    };
}
