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

