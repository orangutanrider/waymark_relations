use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

use crate::syntax_in::{LINE_BREAK, NEXT};

pub(super) fn post_nesting_entity_step_exit(
    mut caravan: TokenIter, 
    package: TokenStream,
    is_nested: bool,
) -> Result<(TokenIter, TokenStream), ()> { 
    // Expect end of iterator 
    // OR
    // Expect NEXT if nested
    // Expect LINE_BREAK if not nested

    let Some(token) = caravan.next() else {
        return Ok((caravan, package)) // End of iterator
    };

    let TokenTree::Punct(token) = token else { // Is Punct?
        return Err(())
    };

    match is_nested {
        true => {
            if token != NEXT { return Err(()) } // Is NEXT?

            return Ok((caravan, package)) 
        },
        false => {
            if token != LINE_BREAK { return Err(()) } // Is LINE_BREAK?

            return Ok((caravan, package)) 
        },
    }
}