//
// Original Copyright 2017 Idan Arye
// Modifications Copyright 2023 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

use syn::parse::Error;
use syn::spanned::Spanned;

/// Return the value that fulfills the predicate if there is one in the iterator.
///
/// Produces an error if there is more than one matching value.
pub fn find_only<T, F>(
    iter: impl Iterator<Item = T>,
    error_message_for_multiple_matches: &str,
    pred: F,
) -> Result<Option<T>, Error>
where
    T: Spanned,
    F: Fn(&T) -> Result<bool, Error>,
{
    let mut result = None;
    for item in iter {
        if pred(&item)? {
            if result.is_some() {
                return Err(Error::new(item.span(), error_message_for_multiple_matches));
            }
            result = Some(item);
        }
    }
    Ok(result)
}
