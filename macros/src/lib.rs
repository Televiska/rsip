use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(HasValue)]
pub fn has_value_signature(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_name = &ast.ident;

    let field_type = field_type(ast.data);
    let field_name = field_type_name(field_type.clone());

    let expanded = if field_name == "String" {
        quote! {
            impl #struct_name {
                pub fn new(value: impl Into<#field_type>) -> Self {
                    Self(value.into())
                }

                pub fn value(&self) -> &str {
                    &self.0
                }
            }
        }
    } else {
        quote! {
            impl #struct_name {
                pub fn new(value: impl Into<#field_type>) -> Self {
                    Self(value.into())
                }

                pub fn value(&self) -> &#field_type {
                    &self.0
                }
            }
        }
    };

    expanded.into()
}

#[proc_macro_derive(Display)]
pub fn display_signature(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_name = &ast.ident;

    let expanded = quote! {
        impl std::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}: {}", stringify!(#struct_name), self.value())
            }
        }
    };

    expanded.into()
}

#[proc_macro_derive(IntoHeader)]
pub fn into_header_signature(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_name = &ast.ident;

    let expanded = quote! {
        impl std::convert::From<#struct_name> for Header {
            fn from(from: #struct_name) -> Self {
                Header::#struct_name(from)
            }
        }
    };

    expanded.into()
}

#[proc_macro_derive(FromStr)]
pub fn from_strs_signature(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_name = &ast.ident;

    let expanded = quote! {
        impl<'a> std::convert::From<&str> for #struct_name {
            fn from(from: &str) -> Self {
                Self(from.into())
            }
        }
    };

    expanded.into()
}

#[proc_macro_derive(FromIntoInner)]
pub fn from_into_inner_signature(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_name = &ast.ident;

    let field_type = field_type(ast.data);

    let from = quote! {
        impl<'a> std::convert::From<#field_type> for #struct_name {
            fn from(from: #field_type) -> Self {
                Self(from)
            }
        }
    };

    let into = quote! {
        impl<'a> std::convert::Into<#field_type> for #struct_name {
            fn into(self) -> #field_type {
                self.0
            }
        }
    };

    let expanded = quote! {
        #from,
        #into
    };

    expanded.into()
}

#[proc_macro_derive(Utf8Tokenizer)]
pub fn utf8_tokenizer(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);

    let fields = match &ast.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };
    let field_name = fields.iter().map(|field| &field.ident);
    let field_name_cloned = field_name.clone();

    let expanded = quote! {
        #[derive(Debug, PartialEq, Eq)]
        pub struct Utf8Tokenizer<'a> {
            #(
                pub #field_name: &'a str,
            )*
        }

        impl<'a> TryFrom<Tokenizer<'a>> for Utf8Tokenizer<'a> {
            type Error = crate::Error;

            fn try_from(tokenizer: Tokenizer<'a>) -> Result<Self, Self::Error> {
                use std::str::from_utf8;

                Ok(Self {
                    #(
                        #field_name_cloned: from_utf8(tokenizer.#field_name_cloned)?,
                    )*
                })
            }
        }
    };

    expanded.into()
}

fn field_type_name(field_type: syn::Type) -> syn::Ident {
    match field_type {
        syn::Type::Reference(syn::TypeReference { elem, .. }) => match *elem {
            syn::Type::Path(syn::TypePath { path, .. }) => {
                path.segments.last().unwrap().ident.clone()
            }
            _ => panic!("supports only reference type"),
        },
        syn::Type::Path(syn::TypePath { path, .. }) => path.segments.last().unwrap().ident.clone(),
        _ => panic!("supports only reference type or type"),
    }
}

fn field_type(ast_data: syn::Data) -> syn::Type {
    match ast_data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }),
            ..
        }) => {
            let field = unnamed
                .first()
                .expect("tuple struct should have at least one element");
            field.ty.clone()
        }
        _ => panic!("Expected a tuple struct"),
    }
}
