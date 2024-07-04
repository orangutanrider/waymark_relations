use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

use crate::{
    entity_step::entity_step_entrance,
    exit_rule_step::{exit_rule_step, ExitRule},
    syntax_in::{EXIT_RULE_NOTATION, LINE_BREAK},
};

pub(crate) fn root_step(
    mut caravan: TokenIter, 
    package: TokenStream,
    mut exit_rule: ExitRule,
) -> Result<(TokenIter, TokenStream, ExitRule), ()> {
    let Some(token) = caravan.next() else {
        return Ok((caravan, package, exit_rule))
    };

    match token {
        TokenTree::Ident(_) => {
            let (caravan, package) = match entity_step_entrance(caravan, package, &exit_rule, false, false, token) {
                Ok(ok) => ok,
                Err(err) => return Err(err),
            };
        
            return root_step(caravan, package, exit_rule)
        },
        TokenTree::Punct(punct) => {
            if punct == LINE_BREAK { // Continue on line breaks
                return root_step(caravan, package, exit_rule);
            }

            match punct == EXIT_RULE_NOTATION {
                true => {
                    let caravan = match exit_rule_step(caravan, &mut exit_rule, punct.spacing()) {
                        Ok(ok) => ok,
                        Err(err) => return Err(err),
                    };
                
                    return root_step(caravan, package, exit_rule)
                },
                false => {
                    let token = TokenTree::Punct(punct);

                    let (caravan, package) = match entity_step_entrance(caravan, package, &exit_rule, false, false, token) {
                        Ok(ok) => ok,
                        Err(err) => return Err(err),
                    };
                
                    return root_step(caravan, package, exit_rule)
                },
            }
        },
        _ => return Err(()),
    }
}