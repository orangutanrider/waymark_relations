use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

use crate::{bindings_step::IntoNext, syntax_in::{LINE_BREAK, NEXT}};

use super::entity_step_entrance;

pub(super) fn nested_entity_step_entrance(
    mut caravan: TokenIter, 
    package: TokenStream,
    exit_rule: &TokenStream,
) -> Result<(TokenIter, TokenStream), ()> { 
    let Some(token) = caravan.next() else {
        return Ok((caravan, package)) // End of iterator
    };

    return entity_step_entrance(caravan, package, exit_rule, true, IntoNext::Escape, token)
}


pub(super) fn nested_entity_step_exit(
    mut caravan: TokenIter, 
    package: TokenStream,
    exit_rule: &TokenStream,
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
            if token != NEXT { return Err(()) } // Is NEXT?

            return nested_entity_step_entrance(caravan, package, exit_rule)
        },
        false => {
            if token != LINE_BREAK { return Err(()) } // Is LINE_BREAK?

            return Ok((caravan, package)) 
        },
    }
}