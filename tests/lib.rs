fn untyped_traits_check<'a, T: rsip::headers::header::UntypedHeader<'a>>() {}
fn typed_traits_check<'a, T: rsip::headers::header::TypedHeader<'a>>() {}

macro_rules! validate_untyped_header_trait {
    ($mod:ident, $name:ident) => {
        #[test]
        fn generated_methods() {
            crate::untyped_traits_check::<$mod::$name>();
        }
    };
}

macro_rules! validate_typed_header_trait {
    ($mod:ident, $name:ident) => {
        #[test]
        fn generated_methods() {
            crate::typed_traits_check::<$mod::typed::$name>();
        }
    };
}

pub mod common;
pub mod headers;
pub mod message;
pub mod services;
pub mod support;
