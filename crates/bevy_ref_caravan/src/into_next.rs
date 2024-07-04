use crate::*;

use crate::{
    wildcard_step::*,
    query_step::query_step,
    syntax_in::*,
    nesting_exit::nesting_exit,
};

use std::vec::IntoIter;

pub(crate) fn into_next_step_entrance(
    mut caravan: TokenIter, 
    package: TokenStream,
    exit_rule: &ExitRule,
    pre_process: &Option<EntityPreProcess>,
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
            let (_, package) = match nested_into_next_step_entrance(nested_caravan, package, exit_rule, pre_process, bindings) {
                Ok(ok) => ok,
                Err(err) => return Err(err),
            };

            return nesting_exit(caravan, package, is_nested);
        },
        TokenTree::Punct(current) => {
            let wildcard = match wildcard_step(current) {
                Ok(ok) => ok,
                Err(err) => return Err(err),
            };

            let Some(current) = caravan.next() else {
                return Err(())
            };

            let Some(indv_binding) = bindings.next() else {
                return Err(())
            };

            return query_step(current, caravan, package, exit_rule, pre_process, is_nested, (wildcard, indv_binding));
        },
        _ => {
            let Some(indv_binding) = bindings.next() else {
                return Err(())
            };

            return query_step(current, caravan, package, exit_rule, pre_process, is_nested, (EntityWildcard::Direct, indv_binding));
        }, 
    }
}

fn nested_into_next_step_entrance(
    mut caravan: TokenIter, 
    package: TokenStream,
    exit_rule: &ExitRule,
    pre_process: &Option<EntityPreProcess>,

    mut bindings: IntoIter<Vec<TokenTree>>,
) -> Result<(TokenIter, TokenStream), ()> {
    let Some(current) = caravan.next() else {
        return Ok((caravan, package));
    };

    let Some(indv_binding) = bindings.next() else {
        return Err(())
    };

    let (caravan, package) = match nested_into_next_step(caravan, package, exit_rule, pre_process, current, indv_binding) {
        Ok(ok) => ok,
        Err(err) => return Err(err),
    };

    return nested_into_next_step_entrance(caravan, package, exit_rule, pre_process, bindings);
}

fn nested_into_next_step(
    mut caravan: TokenIter, 
    package: TokenStream,
    exit_rule: &ExitRule,
    pre_process: &Option<EntityPreProcess>,

    current: TokenTree,
    indv_binding: Vec<TokenTree>,
) -> Result<(TokenIter, TokenStream), ()> {
    let mut wildcard = EntityWildcard::Direct;
    let current = match current {
        TokenTree::Punct(punct) => {
            wildcard = match wildcard_step(punct) {
                Ok(ok) => ok,
                Err(err) => return Err(err),
            };

            let Some(current) = caravan.next() else {
                return Err(())
            };

            current
        },
        _ => { current }
    };

    return query_step(current, caravan, package, exit_rule, pre_process, true, (wildcard, indv_binding));
}

pub(crate) fn collect_individual_bindings(bindings_clause: Vec<TokenTree>) -> Result<Vec<Vec<TokenTree>>, ()> {
    let caravan = bindings_clause.into_iter();
    let caravan = TokenStream::from_iter(caravan).into_iter();

    let mut collected = Vec::new();
    match entrance(caravan, &mut collected) {
        Ok(_) => {/* Proceed */},
        Err(_) => return Err(()),
    }

    return Ok(collected)
}

fn entrance(
    mut caravan: TokenIter,
    collected: &mut Vec<Vec<TokenTree>>
) -> Result<(), ()> {
    let Some(token) = caravan.next() else {
        return Ok(())
    };

    match token {
        TokenTree::Group(group) => {
            // Into nested
            let nested_caravan = group.stream().into_iter();
            match entrance(nested_caravan, collected) {
                Ok(_) => {/* Proceed */},
                Err(_) => {return Err(())},
            }

            // Continue across our own scope
            return entrance(caravan, collected)
        },
        TokenTree::Punct(token) => {
            if token == ',' { // If comma error
                return Err(())
            }

            let mut output= Vec::new();
            collect_unchecked(TokenTree::Punct(token), &mut caravan, &mut output);
            collected.push(output);

            return entrance(caravan, collected)
        },
        _ => {
            let mut output= Vec::new();
            collect_unchecked(token, &mut caravan, &mut output);
            collected.push(output);

            return entrance(caravan, collected)
        }
    }

}

/// First token is not checked to see whether it is a ',' or not.
fn collect_unchecked(
    current: TokenTree,
    caravan: &mut TokenIter,
    output: &mut Vec<TokenTree>,
) {    
    // Collect
    output.push(current);

    let Some(current) = caravan.next() else {
        return
    };

    return collect(current, caravan, output);
} 

fn collect(
    current: TokenTree,
    caravan: &mut TokenIter,
    output: &mut Vec<TokenTree>,
) {    
    match current {
        TokenTree::Punct(current) => {
            if current == ',' {
                return
            }

            output.push(TokenTree::Punct(current));

            let Some(current) = caravan.next() else {
                return
            };
            return collect(current, caravan, output);
        },
        _ => {/* Proceed */},
    }

    // Collect
    output.push(current);

    let Some(current) = caravan.next() else {
        return
    };

    return collect(current, caravan, output);
} 