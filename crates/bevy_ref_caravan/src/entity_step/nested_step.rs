use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

use crate::exit_rule_step::ExitRule;

use super::entity_step_entrance;

pub(super) fn nested_entity_step_entrance(
    mut caravan: TokenIter, 
    package: TokenStream,
    exit_rule: &ExitRule,
) -> Result<(TokenIter, TokenStream), ()> { 
    let Some(token) = caravan.next() else {
        return Ok((caravan, package)) // End of iterator
    };

    let (caravan, package) = match entity_step_entrance(caravan, package, exit_rule, true, false, token) {
        Ok(ok) => ok,
        Err(err) => return Err(err),
    };

    return nested_entity_step_entrance(caravan, package, exit_rule)
}