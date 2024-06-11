mod exit_step; use exit_step::*;
//mod wildcard_step; use wildcard_step::*;
mod nested_step; use nested_step::*;

use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

use crate::syntax_in::ENTIY_STEP_SCOPABLE_DELIMITER;

pub(crate) fn entity_step_entrance(
    caravan: TokenIter, 
    package: TokenStream,
    exit_rule: &TokenStream,
    is_nested: bool,

    current: TokenTree,
) -> Result<(TokenIter, TokenStream), ()> {
    match current {
        // Into nested entity step
        TokenTree::Group(group) => {
            if group.delimiter() != ENTIY_STEP_SCOPABLE_DELIMITER {
                return Err(())
            }

            let mut nested_caravan = group.stream().into_iter();
            let Some(current) = nested_caravan.next() else {
                return Err(())
            };

            let (_, package) = match entity_step_entrance(nested_caravan, package, exit_rule, true, current) {
                Ok(ok) => ok,
                Err(err) => return Err(err),
            };
            
            return post_nesting_entity_step_exit(caravan, package, is_nested)
        },
        // Into single entity step
        TokenTree::Ident(_) => {
            return entity_step_exit(caravan, package, exit_rule, is_nested, current);
        },
        // Into wildcard step, entity step following.
        TokenTree::Punct(_) => {
            todo!()
        },
        // Unexpected, throw error.
        TokenTree::Literal(_) => {
            return Err(())
        },
    }
}

/* 
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
*/