use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod newtype;
mod to_typed_header;
mod typed_header;
mod untyped_header;

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(header))]
struct HeaderOpts {
    display_name: Option<String>,
    //TODO: this should be an enum with parse trait for better safety
    integer_type: Option<String>,
}

#[proc_macro_derive(UntypedHeader, attributes(header))]
pub fn untyped_header_signature(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let opts = HeaderOpts::from_derive_input(&ast).expect("Wrong options");

    let struct_name = &ast.ident;

    //let field_type = field_type(ast.data);
    //let field_name = field_type_name(field_type.clone());

    let untyped_methods = untyped_header::trait_methods(struct_name);
    let display = untyped_header::display(struct_name, opts.display_name);
    let into_header = untyped_header::into_header(struct_name);
    let from_into_string = untyped_header::from_into_string(struct_name);
    let from_str = untyped_header::from_str(struct_name);

    let expanded = quote! {
        #untyped_methods
        #display
        #into_header
        #from_into_string
        #from_str
    };

    expanded.into()
}

#[proc_macro_derive(ToTypedHeader, attributes(header))]
pub fn to_typed_header_signature(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);

    let struct_name = &ast.ident;

    //let field_type = field_type(ast.data);
    //let field_name = field_type_name(field_type.clone());

    let to_typed_header = to_typed_header::trait_methods(struct_name);

    let expanded = quote! {
        #to_typed_header
    };

    expanded.into()
}

#[proc_macro_derive(TypedHeader)]
pub fn typed_header_signature(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_name = &ast.ident;

    //let field_type = field_type(ast.data);
    //let field_name = field_type_name(field_type.clone());

    let typed_methods = typed_header::trait_methods(struct_name);
    let into_string = typed_header::into_string(struct_name);
    let into_untyped = typed_header::into_untyped(struct_name);
    let untyped = typed_header::untyped(struct_name);
    let into_header = typed_header::into_header(struct_name);
    let try_from_untyped = typed_header::try_from_untyped(struct_name);

    let expanded = quote! {
        #typed_methods
        #into_string
        #into_untyped
        #untyped
        #into_header
        #try_from_untyped
    };

    expanded.into()
}

#[proc_macro_derive(UriAndParamsHelpers)]
pub fn uri_and_params_helpers_signature(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_name = &ast.ident;

    let expanded = quote! {
        impl #struct_name {
            pub fn with_uri(mut self, uri: Uri) -> Self {
                self.uri = uri;
                self
            }

            pub fn with_uri_param(mut self, param: Param) -> Self {
                self.uri.params.push(param);
                self
            }

            pub fn with_param(mut self, param: Param) -> Self {
                self.params.push(param);
                self
            }

            pub fn with_params(mut self, params: Vec<Param>) -> Self {
                self.params = params;
                self
            }
        }
    };

    expanded.into()
}

#[proc_macro_derive(StringTyped)]
pub fn string_typed_header_signature(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_name = &ast.ident;

    let expanded = typed_header::string_typed_mods(struct_name);

    expanded.into()
}

#[proc_macro_derive(IntegerTyped, attributes(header))]
pub fn integer_typed_header_signature(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_name = &ast.ident;

    let integer_type = HeaderOpts::from_derive_input(&ast)
        .expect("Wrong options")
        .integer_type
        .expect("not specified integer type");

    let expanded = typed_header::integer_typed_mods(struct_name, &integer_type);

    expanded.into()
}

#[proc_macro_derive(NewType)]
pub fn new_type_signature(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_name = &ast.ident;
    let field_type = field_type(ast.data.clone());

    let new_signature = newtype::new_signature(struct_name, &field_type);
    let value_signature = newtype::value_signature(struct_name, &field_type);
    let display_signature = newtype::display_signature(struct_name);
    let from_inner_signature = newtype::from_inner_signature(struct_name, &field_type);
    let into_inner_signature = newtype::into_inner_signature(struct_name, &field_type);

    let from_str_signature = match is_string(field_type) {
        true => newtype::from_str_signature(struct_name),
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
