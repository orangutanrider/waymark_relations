use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

use crate::{
    common::{collect_until_punct::*, *}, construction_step::construction_step, entity_step::*, exit_rule_override_step::exit_rule_override_step, query_step::QueryMutation, syntax_in::*
};


pub(crate) fn collect_individual_bindings(bindings_clause: &Vec<TokenTree>) -> Vec<TokenStream> {

}

fn entrance(
    mut caravan: TokenIter,
    collected: &mut Vec<TokenStream>
) {
    let Some(token) = caravan.next() else {
        // End
    };

    match token {
        TokenTree::Group(group) => {
            // entrance(caravan)
            // Continue across our own scope
        },
        TokenTree::Punct(_) => {
            // If comma error
            // Collect
        },
        _ => {
            // Collect
        }
    }

}

fn collect(
    current: TokenTree,
    caravan: &mut TokenIter,
    output: &mut Vec<TokenTree>,
) {    
    // If comma, Back to entrance

    // Collect

    let Some(token) = caravan.next() else {
        // Back to entrance
    };
} 