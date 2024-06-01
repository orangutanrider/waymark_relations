use single_entity_step::collect_entity_clause;
use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;
use crate::syntax_in::*;
use super::*;

pub(super) fn into_nested_entity_step(new_scope: Group, caravan: &mut Caravan, exit_rule: &Vec<TokenTree>) -> Result<Caravan, ()> {
    let iter = new_scope.stream().into_iter();

    // Into nested caravan.
    let package = caravan.unpack();
    let depth = caravan.next_depth();
    let nested = Caravan::new(iter, package, depth);
    
    // Nested caravan go.
    return nested_entity_step(nested, EntityBindingKind::Direct, exit_rule);
}

fn nested_entity_step(mut caravan: Caravan, macro_wildcard: EntityBindingKind, exit_rule: &Vec<TokenTree>) -> Result<Caravan, ()> {
    let token = caravan.next();
    let Some(token) = token else {
        return Ok(caravan); // Exit.
    };

    match token {
        // Into nested entity step, then repeat nested entity step.
        TokenTree::Group(_) => {
            let mut nested = match into_nested_entity_step(group, &mut caravan, &exit_rule) {
                Ok(ok) => ok,
                Err(err) => return Err(err),
            };

            // Repack and repeat.
            caravan.repack(nested.unpack());
            return nested_entity_step(caravan, macro_wildcard, exit_rule);
        },
        // Into single entity step, then repeat nested entity step.
        TokenTree::Ident(_) => {
            // Single entity step.
            let caravan = single_entity_step(caravan, token, macro_wildcard);
            let caravan = match caravan {
                Ok(ok) => ok,
                Err(err) => return Err(err),
            };
            
            // Repeat.
            return nested_entity_step(caravan, macro_wildcard, exit_rule);
        },
        // Into wildcard step, entity step following, then repeat nested entity step.
        TokenTree::Punct(_) => {
            todo!()
        },
        // Unexpected, throw error.
        TokenTree::Literal(_) => {
            return Err(())
        },
    }
}
