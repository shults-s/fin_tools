use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::Data::Struct;
use syn::{DeriveInput, Error, Fields, parse_macro_input};

/// Implements get methods for all of struct's fields.
/// Applicable for structs with named fields only.
#[proc_macro_derive(Getters)]
pub fn derive_getters(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields: Vec<_> = if let Struct(data) = &input.data {
        if let Fields::Named(fields) = &data.fields {
            fields
                .named
                .iter()
                // FOR_REVIEWER: I iterate over named fields, therefore,
                // I consider ident=None as a violation of an invariant.
                .map(|field| (field.ident.as_ref().unwrap(), &field.ty))
                .collect()
        } else {
            return Error::new_spanned(&data.fields, "Getters supports only named fields.")
                .into_compile_error()
                .into();
        }
    } else {
        return Error::new_spanned(&input, "Getters can derive only structs.")
            .into_compile_error()
            .into();
    };

    let methods = fields.iter().map(|(field_name, field_ty)| {
        let getter_name = format_ident!("get_{}", field_name);
        quote! {
            #[inline]
            pub fn #getter_name(&self) -> &#field_ty {
                &self.#field_name
            }
        }
    });

    let expanded = quote! {
        impl #name {
            #(
                #methods
            )*
        }
    };

    TokenStream::from(expanded)
}
