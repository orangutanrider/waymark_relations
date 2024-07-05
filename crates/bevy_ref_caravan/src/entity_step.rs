mod exit_step; use exit_step::*;
mod nested_step; use nested_step::*;

use crate::*;
use crate::{
    syntax_in::ENTIY_STEP_SCOPABLE_DELIMITER,
    nesting_exit::nesting_exit,
    wildcard_step::*,
};

pub(crate) fn entity_step_entrance(
    mut caravan: TokenIter, 
    package: TokenStream,
    exit_rule: &ExitRule,
    pre_process: &Option<EntityPreProcess>,
    is_nested: bool,

    followed: bool, // If this step was proceeded by a NEXT combo, then nesting is allowed.
    current: TokenTree,
) -> Result<(TokenIter, TokenStream), ()> {
    match current {
        // Into nested entity step
        TokenTree::Group(group) => {
            match followed {
                true => { /* Proceed */ },
                false => return Err(()),
            }

            if group.delimiter() != ENTIY_STEP_SCOPABLE_DELIMITER {
                return Err(())
            }

            let nested_caravan: TokenIter = group.stream().into_iter();
            let (_, package) = match nested_entity_step_entrance(nested_caravan, package, exit_rule, pre_process) {
                Ok(ok) => ok,
                Err(err) => return Err(err),
            };

            return nesting_exit(caravan, package, is_nested);
        },
        // Into single entity step
        TokenTree::Ident(_) => {
            return entity_step_exit(caravan, package, exit_rule, pre_process, is_nested, current, EntityWildcard::DefaultedDirect);
        },
        // Into wildcard step, entity step following.
        TokenTree::Punct(current) => {
            let wildcard = match wildcard_step(current) {
                Ok(ok) => ok,
                Err(err) => return Err(err),
            };

            let Some(current) = caravan.next() else {
                return Err(())
            };

            return entity_step_exit(caravan, package, exit_rule, pre_process, is_nested, current, wildcard);
        },
        // Unexpected, throw error.
        TokenTree::Literal(_) => {
            return Err(())
        },
    }
}