use quote::quote;

pub fn new_signature(struct_name: &syn::Ident, field_type: &syn::Type) -> proc_macro2::TokenStream {
    quote! {
        impl #struct_name {
            pub fn new(value: impl Into<#field_type>) -> Self {
                Self(value.into())
            }
        }
    }
}

pub fn value_signature(
    struct_name: &syn::Ident,
    field_type: &syn::Type,
) -> proc_macro2::TokenStream {
    let value_type = match crate::is_string(field_type.clone()) {
        true => quote! { &str },
        false => quote! { &#field_type },
    };

    quote! {
        impl #struct_name {
            pub fn value(&self) -> #value_type {
                &self.0
            }
        }
    }
}

pub fn display_signature(struct_name: &syn::Ident) -> proc_macro2::TokenStream {
    quote! {
        impl std::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.value())
            }
        }
    }
}

pub fn from_inner_signature(
    struct_name: &syn::Ident,
    field_type: &syn::Type,
) -> proc_macro2::TokenStream {
    quote! {
        impl<'a> std::convert::From<#field_type> for #struct_name {
            fn from(from: #field_type) -> Self {
                Self(from)
            }
        }
    }
}

//TODO: when the type is Copy, is it faster to do *from.value() ?
pub fn into_inner_signature(
    struct_name: &syn::Ident,
    field_type: &syn::Type,
) -> proc_macro2::TokenStream {
    quote! {
        impl<'a> std::convert::From<#struct_name> for #field_type {
            fn from(from: #struct_name) -> Self {
                from.value().clone().into()
            }
        }
    }
}

pub fn from_str_signature(struct_name: &syn::Ident) -> proc_macro2::TokenStream {
    quote! {
        impl<'a> std::convert::From<&str> for #struct_name {
            fn from(from: &str) -> Self {
                Self(from.into())
            }
        }
    }
}
