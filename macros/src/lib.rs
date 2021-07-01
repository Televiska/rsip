use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod newtype;
//mod typed_header;
//mod untyped_header;
mod headers;

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(header))]
struct HeaderOpts {
    display_name: Option<String>,
    //TODO: this should be an enum with parse trait for better safety
    integer_type: Option<String>,
}

#[proc_macro_derive(HeaderExtImpl)]
pub fn header_ext_impl_signature(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_name = &ast.ident;

    let trait_tokenizer_type = headers::trait_tokenizer_type(&struct_name);
    let into_header = headers::into_header(&struct_name);

    let expanded = quote! {
        #trait_tokenizer_type
        #into_header
    };

    expanded.into()
}

#[proc_macro_derive(DefaultTokenizer)]
pub fn default_tokenizer_header_signature(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_name = &ast.ident;

    let default_tokenizer = headers::default_tokenizer();

    let expanded = quote! {
        #default_tokenizer

        impl<'a> std::convert::TryFrom<Tokenizer<'a>> for #struct_name {
            type Error = crate::Error;

            fn try_from(tokenizer: Tokenizer) -> Result<Self, Self::Error> {
                Ok(Self(tokenizer.part.into()))
            }
        }
    };

    expanded.into()
}

//different from NewType in terms of display
#[proc_macro_derive(HeaderNewType, attributes(header))]
pub fn header_new_type_signature(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_name = &ast.ident;
    let field_type = field_type(ast.data.clone());
    let opts = HeaderOpts::from_derive_input(&ast).expect("Wrong options");

    let new_signature = newtype::new_signature(&struct_name, &field_type);
    let value_signature = newtype::value_signature(&struct_name, &field_type);
    let display_signature = headers::display_signature(&struct_name, opts.display_name);
    let from_inner_signature = newtype::from_inner_signature(&struct_name, &field_type);
    let into_inner_signature = newtype::into_inner_signature(&struct_name, &field_type);

    let from_str_signature = match is_string(field_type) {
        true => newtype::from_str_signature(&struct_name),
        false => quote! {},
    };

    let expanded = quote! {
        #new_signature
        #value_signature
        #display_signature
        #from_inner_signature
        #into_inner_signature
        #from_str_signature
    };

    expanded.into()
}

#[proc_macro_derive(NewType)]
pub fn new_type_signature(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_name = &ast.ident;
    let field_type = field_type(ast.data.clone());

    let new_signature = newtype::new_signature(&struct_name, &field_type);
    let value_signature = newtype::value_signature(&struct_name, &field_type);
    let display_signature = newtype::display_signature(&struct_name);
    let from_inner_signature = newtype::from_inner_signature(&struct_name, &field_type);
    let into_inner_signature = newtype::into_inner_signature(&struct_name, &field_type);

    let from_str_signature = match is_string(field_type) {
        true => newtype::from_str_signature(&struct_name),
        false => quote! {},
    };

    let expanded = quote! {
        #new_signature
        #value_signature
        #display_signature
        #from_inner_signature
        #into_inner_signature
        #from_str_signature
    };

    expanded.into()
}

#[proc_macro_derive(IntoParam)]
pub fn into_param_signature(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_name = &ast.ident;

    //let field_type = field_type(ast.data);
    //let field_name = field_type_name(field_type.clone());

    let expanded = quote! {
        impl std::convert::From<#struct_name> for crate::common::uri::Param {
            fn from(param: #struct_name) -> Self {
                crate::common::uri::Param::#struct_name(param)
            }
        }
    };

    expanded.into()
}

struct FieldType {
    ident: syn::Ident,
    is_option: bool,
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
    let fields = fields.iter().map(|field| match field.ty {
        syn::Type::Path(_) => FieldType {
            ident: field
                .ident
                .clone()
                .expect("expected struct with named fields"),
            is_option: true,
        },
        _ => FieldType {
            ident: field
                .ident
                .clone()
                .expect("expected struct with named fields"),
            is_option: false,
        },
    });
    let raw_fields: Vec<syn::Ident> = fields
        .clone()
        .into_iter()
        .filter_map(|f| (!f.is_option).then(|| f.ident))
        .collect();
    let option_fields: Vec<syn::Ident> = fields
        .clone()
        .into_iter()
        .filter_map(|f| f.is_option.then(|| f.ident))
        .collect();

    let expanded = quote! {
        #[derive(Debug, PartialEq, Eq)]
        pub struct Utf8Tokenizer<'a> {
            #(
                pub #raw_fields: &'a str,
            )*
            #(
                pub #option_fields: Option<&'a str>,
            )*
        }

        impl<'a> std::convert::TryFrom<Tokenizer<'a>> for Utf8Tokenizer<'a> {
            type Error = crate::Error;

            fn try_from(tokenizer: Tokenizer<'a>) -> Result<Self, Self::Error> {
                use std::str::from_utf8;

                Ok(Self {
                    #(
                        #raw_fields: from_utf8(tokenizer.#raw_fields)?,
                    )*
                    #(
                        #option_fields: tokenizer.#option_fields.map(from_utf8).transpose()?,
                    )*
                })
            }
        }
    };

    expanded.into()
}

pub(crate) fn field_type_name(field_type: syn::Type) -> syn::Ident {
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

pub(crate) fn field_type(ast_data: syn::Data) -> syn::Type {
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

pub(crate) fn is_string(field_type: syn::Type) -> bool {
    let field_type_name = field_type_name(field_type);

    matches!(field_type_name, typ if typ == "String")
}

pub(crate) fn kebab_case(struct_name: String) -> String {
    let struct_name_str_chars: Vec<char> = struct_name.chars().collect();
    let mut dashed_struct_name: Vec<char> = Vec::new();
    struct_name_str_chars
        .iter()
        .enumerate()
        .for_each(|(index, c)| {
            if c.is_ascii_uppercase()
                && (index > 0)
                && !(struct_name_str_chars[index - 1].is_ascii_uppercase())
            {
                dashed_struct_name.extend(vec!['-', *c].iter());
            } else {
                dashed_struct_name.push(*c);
            }
        });

    dashed_struct_name.into_iter().collect::<String>()
}
