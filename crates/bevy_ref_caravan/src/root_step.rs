use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

use crate::entity_step::entity_step_entrance;

pub(crate) fn root_step(
    mut caravan: TokenIter, 
    mut package: TokenStream,
    exit_rule: TokenStream,
) -> Result<(TokenIter, TokenStream, TokenStream), ()> {
    let Some(token) = caravan.next() else {
        return Ok((caravan, package, exit_rule))
    };

    // add check for line break

    // add check for exit rule

    let (caravan, package) = match entity_step_entrance(caravan, package, &exit_rule, token) {
        Ok(ok) => ok,
        Err(err) => return Err(err),
    };

    return root_step(caravan, package, exit_rule)
}