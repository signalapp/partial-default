//
// Original Copyright 2017 Idan Arye
// Modifications Copyright 2023 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

use proc_macro2::TokenStream;

use quote::quote;
use syn::parse::Error;
use syn::spanned::Spanned;
use syn::DeriveInput;

use crate::default_attr::DefaultAttr;
use crate::util::find_only;

pub fn impl_my_derive(input: &DeriveInput) -> Result<TokenStream, Error> {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let where_clause_generics = where_clause.map(|clause| {
        // Convert predicates individually to guarantee a trailing comma.
        let predicates = clause.predicates.iter();
        quote! { #(#predicates,)* }
    });
    let additional_generics = additional_generics_tt(input)?;

    let default_expr = match input.data {
        syn::Data::Struct(ref body) => {
            let body_assignment = default_body_tt(&body.fields)?;
            quote! {
                #name #body_assignment
            }
        }
        syn::Data::Enum(ref body) => {
            let default_variant = find_only(
                body.variants.iter(),
                "Only one variant can be marked #[partial_default]",
                |variant| {
                    if let Some(meta) = DefaultAttr::find_in_attributes(&variant.attrs)? {
                        if matches!(meta, DefaultAttr::Empty) {
                            Ok(true)
                        } else {
                            Err(Error::new_spanned(
                                &variant.ident,
                                "Attribute #[partial_default] on variants should have no value",
                            ))
                        }
                    } else {
                        Ok(false)
                    }
                },
            )?
            .ok_or_else(|| Error::new(input.span(), "No default variant"))?;
            let default_variant_name = &default_variant.ident;
            let body_assignment = default_body_tt(&default_variant.fields)?;
            quote! {
                #name :: #default_variant_name #body_assignment
            }
        }
        syn::Data::Union(_) => {
            panic!()
        }
    };
    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics ::partial_default::PartialDefault for #name #ty_generics where #where_clause_generics #additional_generics {
            fn partial_default() -> Self {
                #default_expr
            }
        }
    })
}

fn additional_generics_tt(item: &syn::DeriveInput) -> Result<TokenStream, Error> {
    if let Some(default_attr) = DefaultAttr::find_in_attributes(&item.attrs)? {
        if let DefaultAttr::Bound(bound) = default_attr {
            bound.parse()
        } else {
            Err(Error::new(
                item.ident.span(),
                r#"Expected #[partial_default(bound = "...")"#,
            ))
        }
    } else {
        let bounds = item.generics.type_params().map(|param| {
            let ident = &param.ident;
            quote! { #ident: ::partial_default::PartialDefault }
        });
        Ok(quote! {
            #(#bounds),*
        })
    }
}

/// Return a token-tree for the default "body" - the part after the name that contains the values.
///
/// That is, the `{ ... }` part for structs, the `(...)` part for tuples, and nothing for units.
fn default_body_tt(body: &syn::Fields) -> Result<TokenStream, Error> {
    Ok(match body {
        syn::Fields::Named(ref fields) => {
            let field_assignments = fields
                .named
                .iter()
                .map(|field| {
                    let field_name = field.ident.as_ref();
                    let default_value = field_default_expr(field)?;
                    Ok(quote! { #field_name : #default_value })
                })
                .collect::<Result<Vec<_>, Error>>()?;
            quote! {
                {
                    #( #field_assignments ),*
                }
            }
        }
        syn::Fields::Unnamed(ref fields) => {
            let field_assignments = fields
                .unnamed
                .iter()
                .map(field_default_expr)
                .collect::<Result<Vec<TokenStream>, Error>>()?;
            quote! {
                (
                    #( #field_assignments ),*
                )
            }
        }
        &syn::Fields::Unit => quote! {},
    })
}

/// Return a default expression for a field based on it's `#[default = "..."]` attribute.
///
/// Errors if there is more than one, of if there is a `#[default]` attribute without value.
fn field_default_expr(field: &syn::Field) -> Result<TokenStream, Error> {
    if let Some(default_attr) = DefaultAttr::find_in_attributes(&field.attrs)? {
        if let DefaultAttr::Value(field_value) = default_attr {
            field_value.parse()
        } else {
            Err(Error::new(
                field.span(),
                r#"Expected #[partial_default(value = "...")"#,
            ))
        }
    } else {
        Ok(quote! {
            ::partial_default::PartialDefault::partial_default()
        })
    }
}
