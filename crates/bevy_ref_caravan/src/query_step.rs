/* 
mod single_query_step;

use proc_macro::*;
use super::*;

pub(crate) query_step_entrance(mut caravan: Caravan, entity_input: TokenStream, exit_rule: &TokenStream) -> Result<Caravan, ()> {
    let token = caravan.next();
    let Some(token) = token else {
        return Ok(caravan);
    };

    match token {
        TokenTree::Group(_group) => { 
            todo!()
        },
        TokenTree::Ident(_) => {
            return single_query_step(caravan, token, entity_input)
        },
        TokenTree::Punct(_) => {
            return single_query_step(caravan, token, entity_input)
        },
        TokenTree::Literal(_) => {
            return Err(())
        },
    }
}
*/