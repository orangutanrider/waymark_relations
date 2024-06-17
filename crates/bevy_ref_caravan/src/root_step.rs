use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

use crate::{
    bindings_step::IntoNext,
    entity_step::entity_step_entrance,
    exit_rule_step::exit_rule_step,
    syntax_in::EXIT_RULE_NOTATION,
};

pub(crate) fn root_step(
    mut caravan: TokenIter, 
    package: TokenStream,
    mut exit_rule: TokenStream,
) -> Result<(TokenIter, TokenStream, TokenStream), ()> {
    let Some(token) = caravan.next() else {
        return Ok((caravan, package, exit_rule))
    };


    match token {
        TokenTree::Ident(_) => {
            let (caravan, package) = match entity_step_entrance(caravan, package, &exit_rule, false, IntoNext::Escape, token) {
                Ok(ok) => ok,
                Err(err) => return Err(err),
            };
        
            return root_step(caravan, package, exit_rule)
        },
        TokenTree::Punct(punct) => {
            match punct == EXIT_RULE_NOTATION {
                true => {
                    exit_rule = TokenStream::new();

                    let caravan = match exit_rule_step(caravan, &mut exit_rule) {
                        Ok(ok) => ok,
                        Err(err) => return Err(err),
                    };
                
                    return root_step(caravan, package, exit_rule)
                },
                false => {
                    let token = TokenTree::Punct(punct);

                    let (caravan, package) = match entity_step_entrance(caravan, package, &exit_rule, false, IntoNext::Escape, token) {
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