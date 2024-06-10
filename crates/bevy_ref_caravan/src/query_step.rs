mod exit_step;
use exit_step::*;

use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

pub(crate) fn query_step_entrance(
    mut caravan: TokenIter, 
    package: TokenStream,
    exit_rule: &TokenStream,
    entity_clause: Vec<TokenTree>, 
) -> Result<(TokenIter, TokenStream), ()> {
    let token = caravan.next();
    let Some(token) = token else {
        return Ok((caravan, package));
    };

    return query_step_exit(caravan, package, exit_rule, entity_clause, token);

    /* 
    match token {
        TokenTree::Group(_group) => { 
            todo!()
        },
        TokenTree::Ident(_) => {
            
        },
        TokenTree::Punct(_) => {
            return single_query_step(caravan, token, entity_input)
        },
        TokenTree::Literal(_) => {
            return Err(())
        },
    }
    */
}