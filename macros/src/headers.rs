use quote::quote;

pub fn trait_tokenizer_type(struct_name: &syn::Ident) -> proc_macro2::TokenStream {
    quote! {
        impl<'a> crate::headers::header::HeaderExt<'a> for #struct_name {
            type Tokenizer = Tokenizer<'a>;
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

pub fn display_signature(
    struct_name: &syn::Ident,
    display_name: Option<String>,
) -> proc_macro2::TokenStream {
    let name = match display_name {
        Some(display_name) => display_name,
        None => crate::kebab_case(struct_name.to_string()),
    };

    quote! {
        impl std::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}: {}", #name, self.value())
            }
        }
    }
}

pub fn default_tokenizer() -> proc_macro2::TokenStream {
    quote! {
        pub use tokenizer::Tokenizer;

        pub mod tokenizer {
            use crate::headers::header::Tokenize;

            #[derive(Eq, PartialEq, Clone, Debug)]
            pub struct Tokenizer<'a> {
                pub part: &'a str,
            }

            impl<'a> Tokenize<'a> for Tokenizer<'a> {
                fn tokenize(part: &'a str) -> Result<Self, crate::Error> {
                    Ok(Self { part })
                }
            }
        }
    }
}
