use std::vec::IntoIter;
use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

use crate::entity_step::EntityWildcard;
use crate::query_step::query_step;
use crate::syntax_in::*;
use crate::nesting_exit::nesting_exit;

pub(crate) fn into_next_step(
    mut caravan: TokenIter, 
    package: TokenStream,
    exit_rule: &TokenStream,
    is_nested: bool,

    mut bindings: IntoIter<Vec<TokenTree>>,
) -> Result<(TokenIter, TokenStream), ()> {
    let Some(current) = caravan.next() else {
        return Ok((caravan, package));
    };

    match current {
        TokenTree::Group(group) => {
            if group.delimiter() != ENTIY_STEP_SCOPABLE_DELIMITER {
                return Err(())
            }

            let nested_caravan: TokenIter = group.stream().into_iter();
            let (_, package) = match nested_into_next_step_entrance(nested_caravan, package, exit_rule, is_nested, bindings) {
                Ok(ok) => ok,
                Err(err) => return Err(err),
            };

            return nesting_exit(caravan, package, is_nested);
        },
        _ => {
            let Some(entity_clause) = bindings.next() else {
                return Err(())
            };

            return query_step(current, caravan, package, exit_rule, is_nested, (EntityWildcard::Direct, entity_clause));
        }, 
    }
}

fn nested_into_next_step_entrance(
    mut caravan: TokenIter, 
    package: TokenStream,
    exit_rule: &TokenStream,
    is_nested: bool,

    mut bindings: IntoIter<Vec<TokenTree>>,
) -> Result<(TokenIter, TokenStream), ()> {
    let Some(current) = caravan.next() else {
        return Ok((caravan, package))
    };

    let Some(entity_clause) = bindings.next() else {
        return Err(())
    };

    let (caravan, package) = match query_step(current, caravan, package, exit_rule, is_nested, (EntityWildcard::Direct, entity_clause)) {
        Ok(ok) => ok,
        Err(err) => return Err(err),
    };

    return nested_into_next_step_entrance(caravan, package, exit_rule, is_nested, bindings);
}