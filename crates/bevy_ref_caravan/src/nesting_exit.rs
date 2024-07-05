use crate::*;
use crate::syntax_in::{LINE_BREAK, SCOPED_BREAK};

/// This comes after the end of the scope
pub(crate) fn nesting_exit(
    mut caravan: TokenIter, 
    package: TokenStream,
    is_nested: bool,
) -> Result<(TokenIter, TokenStream), ()> { 
    // Expect end of iterator 
    // OR
    // Expect NEXT if nested
    // Expect LINE_BREAK if not nested
    // If NEXT, repeat entry step

    let Some(token) = caravan.next() else {
        return Ok((caravan, package)) // End of iterator
    };

    let TokenTree::Punct(token) = token else { // Is Punct?
        return Err(())
    };

    match is_nested {
        true => {
            if token != SCOPED_BREAK { return Err(()) } // Is SCOPED_BREAK? ( , )

            return Ok((caravan, package)) 
        },
        false => {
            if token != LINE_BREAK { return Err(()) } // Is LINE_BREAK? ( ; )

            return Ok((caravan, package)) 
        },
    }
}