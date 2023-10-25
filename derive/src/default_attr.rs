//
// Original Copyright 2017 Idan Arye
// Modifications Copyright 2023 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

use syn::parse::Error;

use crate::util::find_only;

pub enum DefaultAttr {
    Empty,
    Bound(syn::LitStr),
    Value(syn::LitStr),
}

impl DefaultAttr {
    pub fn find_in_attributes(attrs: &[syn::Attribute]) -> Result<Option<Self>, Error> {
        if let Some(default_attr) = find_only(
            attrs.iter(),
            "cannot have multiple #[partial_default] attributes on the same item",
            |attr| Ok(attr.path().is_ident("partial_default")),
        )? {
            match &default_attr.meta {
                syn::Meta::Path(_) => Ok(Some(Self::Empty)),
                syn::Meta::List(meta) => {
                    let mut result = None;
                    meta.parse_nested_meta(|nested| {
                        if result.is_some() {
                            return Err(nested.error("invalid syntax for partial_default"));
                        }

                        if nested.path.is_ident("bound") {
                            result = Some(Self::Bound(nested.value()?.parse()?));
                            return Ok(());
                        }

                        if nested.path.is_ident("value") {
                            result = Some(Self::Value(nested.value()?.parse()?));
                            return Ok(());
                        }

                        Err(nested.error("invalid syntax for partial_default"))
                    })?;
                    Ok(result)
                }
                syn::Meta::NameValue(_) => Err(Error::new_spanned(
                    &default_attr.meta,
                    "invalid syntax for partial_default",
                )),
            }
        } else {
            Ok(None)
        }
    }
}
