use crate::*;

use super::entity_step_entrance;

pub(super) fn nested_entity_step_entrance(
    mut caravan: TokenIter, 
    package: TokenStream,
    exit_rule: &ExitRule,
    pre_process: &Option<EntityPreProcess>,
) -> Result<(TokenIter, TokenStream), ()> { 
    let Some(token) = caravan.next() else {
        return Ok((caravan, package)) // End of iterator
    };

    let (caravan, package) = match entity_step_entrance(caravan, package, exit_rule, pre_process, true, false, token) {
        Ok(ok) => ok,
        Err(err) => return Err(err),
    };

    return nested_entity_step_entrance(caravan, package, exit_rule, pre_process)
}