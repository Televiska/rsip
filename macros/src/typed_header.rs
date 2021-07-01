use quote::quote;

pub fn trait_methods(struct_name: &syn::Ident) -> proc_macro2::TokenStream {
    quote! {
        impl<'a> crate::headers::header::TypedHeader<'a> for #struct_name {
            type Tokenizer = super::Tokenizer<'a>;
        }
    }
}

//TODO: this shouldn't be needed once specialization lands
pub fn into_untyped(struct_name: &syn::Ident) -> proc_macro2::TokenStream {
    quote! {
        impl std::convert::From<#struct_name> for super::#struct_name {
            fn from(typed: #struct_name) -> Self {
                super::#struct_name(typed.into())
            }
        }
    }
}

pub fn untyped(struct_name: &syn::Ident) -> proc_macro2::TokenStream {
    quote! {
        impl #struct_name {
            pub fn untyped(self) -> super::#struct_name {
                super::#struct_name(self.into())
            }
        }
    }
}

//TODO: this shouldn't be needed once specialization lands
pub fn into_string(struct_name: &syn::Ident) -> proc_macro2::TokenStream {
    quote! {
        impl std::convert::From<#struct_name> for String {
            fn from(typed: #struct_name) -> Self {
                typed.to_string()
            }
        }
    }
}

pub fn into_header(struct_name: &syn::Ident) -> proc_macro2::TokenStream {
    quote! {
        impl std::convert::From<#struct_name> for crate::Header {
            fn from(typed: #struct_name) -> Self {
                crate::Header::#struct_name(typed.into())
            }
        }
    }
}

//TODO: this should be needed once specialization lands
pub fn try_from_untyped(struct_name: &syn::Ident) -> proc_macro2::TokenStream {
    quote! {
        impl std::convert::TryFrom<super::#struct_name> for #struct_name {
            type Error = crate::Error;

            fn try_from(untyped: super::#struct_name) -> Result<Self, Self::Error> {
                use crate::headers::header::UntypedHeader;
                use crate::headers::header::Tokenize;

                std::convert::TryInto::try_into(Tokenizer::tokenize(untyped.value())?)
            }
        }
    }
}

pub fn integer_typed_mods(
    struct_name: &syn::Ident,
    integer_type: &str,
) -> proc_macro2::TokenStream {
    let default_tokenizer = default_tokenizer();
    let integer_type = quote::format_ident!("{}", integer_type);

    quote! {
        #default_tokenizer

        pub mod typed {
            use super::Tokenizer;
            use macros::TypedHeader;

            //TODO: reorganize HasValue, reuse custom Display macro
            #[derive(TypedHeader, Eq, PartialEq, Clone, Debug)]
            pub struct #struct_name(#integer_type);

            impl #struct_name {
                pub fn new(value: impl Into<#integer_type>) -> Self {
                    Self(value.into())
                }

                pub fn value(&self) -> &#integer_type {
                    &self.0
                }
            }

            impl<'a> std::convert::TryFrom<Tokenizer<'a>> for #struct_name {
                type Error = crate::Error;

                fn try_from(tokenizer: Tokenizer) -> Result<Self, Self::Error> {
                    Ok(Self(tokenizer.part.parse::<#integer_type>()?))
                }
            }

            impl std::fmt::Display for #struct_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.0)
                }
            }
        }
    }
}

pub fn string_typed_mods(struct_name: &syn::Ident) -> proc_macro2::TokenStream {
    let default_tokenizer = default_tokenizer();

    quote! {
        #default_tokenizer

        pub mod typed {
            use super::Tokenizer;
            use macros::TypedHeader;

            //TODO: reorganize HasValue, reuse custom Display macro
            #[derive(TypedHeader, Eq, PartialEq, Clone, Debug)]
            pub struct #struct_name(String);

            impl #struct_name {
                pub fn new(value: impl Into<String>) -> Self {
                    Self(value.into())
                }

                pub fn value(&self) -> &str {
                    &self.0
                }
            }

            impl<'a> std::convert::TryFrom<Tokenizer<'a>> for #struct_name {
                type Error = crate::Error;

                fn try_from(tokenizer: Tokenizer) -> Result<Self, Self::Error> {
                    Ok(Self(tokenizer.part.into()))
                }
            }

            impl std::fmt::Display for #struct_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.0)
                }
            }
        }
    }
}

