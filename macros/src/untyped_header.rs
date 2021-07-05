use quote::quote;

pub fn trait_methods(struct_name: &syn::Ident) -> proc_macro2::TokenStream {
    quote! {
        impl<'a> crate::headers::header::UntypedHeader<'a> for #struct_name {
            type Typed = typed::#struct_name;

            fn new(value: impl std::convert::Into<String>) -> Self {
                Self(value.into())
            }

            fn value(&self) -> &str {
                &self.0
            }

            fn typed(&self) -> Result<typed::#struct_name, crate::Error> {
                std::convert::TryInto::try_into(self.clone())
            }

            fn into_typed(self) -> Result<typed::#struct_name, crate::Error> {
                std::convert::TryInto::try_into(self)
            }

            fn replace(&mut self, new_value: impl Into<String>) {
                self.0 = new_value.into();
            }
        }
    }
}

//TODO: are we sure that we want here the {}: {} ? Maybe Header should do that
pub fn display(struct_name: &syn::Ident, display_name: Option<String>) -> proc_macro2::TokenStream {
    let name = match display_name {
        Some(display_name) => display_name,
        None => crate::kebab_case(struct_name.to_string()),
    };

    quote! {
        impl std::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use crate::headers::header::UntypedHeader;

                write!(f, "{}: {}", #name, self.value())
            }
        }
    }
}

pub fn into_header(struct_name: &syn::Ident) -> proc_macro2::TokenStream {
    quote! {
        impl std::convert::From<#struct_name> for crate::Header {
            fn from(from: #struct_name) -> Self {
                crate::Header::#struct_name(from)
            }
        }
    }
}

//TODO: this shouldn't be needed once specialization lands
pub fn from_into_string(struct_name: &syn::Ident) -> proc_macro2::TokenStream {
    let from = quote! {
        impl<'a> std::convert::From<String> for #struct_name {
            fn from(from: String) -> Self {
                Self(from)
            }
        }
    };

    let from_value = quote! {
        impl<'a> std::convert::From<#struct_name> for String {
            fn from(from: #struct_name) -> Self {
                use crate::headers::header::UntypedHeader;

                from.value().clone().into()
            }
        }
    };

    quote! {
        #from
        #from_value
    }
}

//TODO: this shouldn't be needed once specialization lands
pub fn from_str(struct_name: &syn::Ident) -> proc_macro2::TokenStream {
    quote! {
        impl<'a> std::convert::From<&str> for #struct_name {
            fn from(from: &str) -> Self {
                Self(from.into())
            }
        }
    }
}
