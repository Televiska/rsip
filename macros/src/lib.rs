use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod typed;
mod untyped;

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

    let untyped_methods = untyped::trait_methods(struct_name.clone());
    let display = untyped::display(struct_name.clone(), opts.display_name);
    let into_header = untyped::into_header(struct_name.clone());
    let from_into_string = untyped::from_into_string(struct_name.clone());
    let from_str = untyped::from_str(struct_name.clone());

    let expanded = quote! {
        #untyped_methods
        #display
        #into_header
        #from_into_string
        #from_str
    };

    expanded.into()
}

#[proc_macro_derive(TypedHeader)]
pub fn typed_header_signature(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_name = &ast.ident;

    //let field_type = field_type(ast.data);
    //let field_name = field_type_name(field_type.clone());

    let typed_methods = typed::trait_methods(struct_name.clone());
    let into_string = typed::into_string(struct_name.clone());
    let into_untyped = typed::into_untyped(struct_name.clone());
    let into_header = typed::into_header(struct_name.clone());
    let try_from_untyped = typed::try_from_untyped(struct_name.clone());

    let expanded = quote! {
        #typed_methods
        #into_string
        #into_untyped
        #into_header
        #try_from_untyped
    };

    expanded.into()
}

#[proc_macro_derive(StringTyped)]
pub fn string_typed_header_signature(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_name = &ast.ident;

    let expanded = typed::string_typed_mods(struct_name.clone());

    expanded.into()
}

#[proc_macro_derive(IntegerTyped, attributes(header))]
pub fn integer_typed_header_signature(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_name = &ast.ident;

    let integer_type = HeaderOpts::from_derive_input(&ast)
        .expect("Wrong options")
        .integer_type
        .unwrap_or_else(|| "i32".into());

    let expanded = typed::integer_typed_mods(struct_name.clone(), integer_type);

    expanded.into()
}

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

//TODO: improve PamelCase to Kebab-Case impl here
#[proc_macro_derive(Display)]
pub fn display_signature(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_name = &ast.ident;
    let struct_name_str_chars: Vec<char> = struct_name.to_string().chars().collect();
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
    let dashed_struct_name: String = dashed_struct_name.into_iter().collect::<String>();

    let expanded = quote! {
        impl std::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}: {}", #dashed_struct_name, self.value())
            }
        }
    };

    expanded.into()
}

#[proc_macro_derive(ValueDisplay)]
pub fn value_display_signature(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_name = &ast.ident;

    let expanded = quote! {
        impl std::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.value())
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

#[proc_macro_derive(FromValue)]
pub fn from_value_signature(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_name = &ast.ident;

    let expanded = quote! {
        impl<'a> std::convert::From<&'a str> for #struct_name<'a> {
            fn from(value: &'a str) -> Self {
                Self { value }
            }
        }
    };

    expanded.into()
}

#[proc_macro_derive(Typed)]
pub fn typed_signature(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_name = &ast.ident;

    let expanded = quote! {
        impl #struct_name {
            pub fn typed(self) -> Result<typed::#struct_name, crate::Error> {
                self.try_into()
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

    let from_value = quote! {
        impl<'a> std::convert::From<#struct_name> for #field_type {
            fn from(from: #struct_name) -> Self {
                from.value().clone().into()
            }
        }
    };

    let expanded = quote! {
        #from
        #from_value
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
