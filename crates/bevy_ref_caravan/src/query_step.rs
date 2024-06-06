mod single_query_step;
use single_query_step::*;

use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

pub(crate) fn query_step_entrance(
    mut caravan: TokenIter, 
    package: TokenStream,
    exit_rule: &TokenStream,
    entity_input: TokenStream, 
) -> Result<(TokenIter, TokenStream), ()> {
    let token = caravan.next();
    let Some(token) = token else {
        return Ok((caravan, package));
    };

    todo!()
    //return single_query_step(caravan, token, entity_input);

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