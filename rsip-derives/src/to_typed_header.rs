use quote::quote;

pub fn trait_methods(struct_name: &syn::Ident) -> proc_macro2::TokenStream {
    quote! {
        impl<'a> crate::headers::untyped::ToTypedHeader<'a> for #struct_name {
            type Typed = crate::headers::typed::#struct_name;

            fn typed(&self) -> Result<Self::Typed, crate::Error> {
                std::convert::TryInto::try_into(self.clone())
            }

            fn into_typed(self) -> Result<Self::Typed, crate::Error> {
                std::convert::TryInto::try_into(self)
            }
        }
    }
}
