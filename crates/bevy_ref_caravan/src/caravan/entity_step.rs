mod collect_entity_clause;

use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;
use crate::syntax_in::*;
use super::*;

// (caravan, exit_rule)
pub(crate) fn entity_step_entrance(mut caravan: Caravan, exit_rule: Vec<TokenTree>) -> Result<(Caravan, Vec<TokenTree>), ()> {
    let token = caravan.next();
    let Some(token) = token else {
        return Ok((caravan, exit_rule));
    };
    
    match token {
        // Into nested entity step
        TokenTree::Group(group) => {
            let iter = group.stream().into_iter();

            // Into nested caravan.
            let package = caravan.unpack();
            let depth = caravan.next_depth();
            let nested = Caravan::into_nested(&caravan, iter, package, depth);

            // Nested caravan go.
            let nested = nested_entity_step(nested, EntityBindingKind::Direct, &exit_rule);
            let mut nested = match nested {
                Ok(ok) => ok,
                Err(err) => return Err(err),
            };

            // Repack and continue.
            caravan.repack(nested.unpack());
            return entity_step_entrance(caravan, exit_rule);
        },
        // Into single entity step
        TokenTree::Ident(_) => {
            match single_entity_step(caravan, token, EntityBindingKind::Direct) {
                Ok(caravan) => {
                    return Ok((caravan, exit_rule))
                },
                Err(err) => {
                    return Err(err)
                },
            }
        },
        // Into wildcard check, entity step following.
        // Or into exit rule decleration step.
        TokenTree::Punct(_) => {
            todo!()
        },
        // Unexpected, throw error.
        TokenTree::Literal(_) => {
            return Err(())
        },
    }
}

fn nested_entity_step(mut caravan: Caravan, macro_wildcard: EntityBindingKind, exit_rule: &Vec<TokenTree>) -> Result<Caravan, ()> {
    let token = caravan.next();
    let Some(token) = token else {
        return Ok(caravan);
    };

    match token {
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
        // Into wildcard check, single entity step following, then repeat   nested entity step.
        // No exit rule declerations within nested steps.
        TokenTree::Punct(_) => {
            todo!()
        },
        // Unexpected, throw error.
        TokenTree::Group(_) => {
            return Err(())
        },
        // Unexpected, throw error.
        TokenTree::Literal(_) => {
            return Err(())
        },
    }
}

fn single_entity_step(caravan: Caravan, current: TokenTree, wildcard: EntityBindingKind) -> Result<Caravan, ()> {
    
    todo!()
}

#[derive(Clone, Copy)]
/// Matched to entity wildcard symbols.
enum EntityBindingKind {
    /// DEFAULT
    /// A waymark binding.
    /// The component is used directly, feeding the entity data into the query; no entity binding is created.
    Direct,
    /// LIFT ^
    /// A waymark binding.
    /// The component is used to create an entity binding, without shadowing the component binding that the entity came from.
    Lifted,
    /// OVERLAP ~
    /// A waymark binding.
    /// The component is used to create an entity binding, that shadows the component binding that the entity came from.
    Overlap,
    /// LITERAL @
    /// A literal entity binding.
    Literal,
}