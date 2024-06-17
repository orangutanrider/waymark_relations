use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

use crate::common::collect_until_punct::collect_until_matching_punct;
use crate::syntax_in::LINE_BREAK;

pub(crate) fn exit_rule_step(
    caravan: TokenIter, 
    exit_rule: &mut TokenStream,
) -> TokenIter {
    // Continue until semi-colon or nothing.
    // Collect tokens as new exit_rule.
    // Do not enter groups.

    let (_, caravan, new_exit_rule) = collect_until_matching_punct(LINE_BREAK, caravan, Vec::new());

    exit_rule.extend(new_exit_rule.into_iter());

    return caravan
}
