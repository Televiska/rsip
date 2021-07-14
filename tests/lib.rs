fn untyped_traits_check<'a, T: rsip::headers::untyped::UntypedHeader<'a>>() {}
fn to_typed_traits_check<'a, T: rsip::headers::untyped::ToTypedHeader<'a>>() {}
fn typed_traits_check<'a, T: rsip::headers::typed::TypedHeader<'a>>() {}

macro_rules! validate_untyped_header_trait {
    ($name:ident) => {
        #[test]
        fn validate_untyped_header_trait() {
            crate::untyped_traits_check::<$name>();
        }
    };
}

macro_rules! validate_to_typed_header_trait {
    ($name:ident) => {
        #[test]
        fn validate_to_typed_header_trait() {
            crate::to_typed_traits_check::<$name>();
        }
    };
}

macro_rules! validate_typed_header_trait {
    ($name:ident) => {
        #[test]
        fn validate_typed_header_trait() {
            crate::typed_traits_check::<$name>();
        }
    };
}

pub mod common;
pub mod headers;
pub mod message;
pub mod services;
pub mod support;
