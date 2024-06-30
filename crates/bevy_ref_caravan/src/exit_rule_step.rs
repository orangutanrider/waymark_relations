use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

use crate::common::collect_until_punct::*;
use crate::syntax_in::{ABBREVIATED_RETURN, EXIT_RULE_DELIMITER, EXIT_RULE_NOTATION, LINE_BREAK};

pub(crate) fn exit_rule_step(
    mut caravan: TokenIter, 
    exit_rule: &mut TokenStream,
    bang_spacing: Spacing,
) -> Result<TokenIter, ()> {
    let Some(token) = caravan.next() else {
        return Err(())
    };

    match token {
        TokenTree::Group(group) => {
            // Validate delimiter
            if group.delimiter() != EXIT_RULE_DELIMITER {
                return Err(())
            }

            // Collect group's tokens.
            exit_rule.extend(group.stream());

            // Expect empty or line break.
            let Some(token) = caravan.next() else {
                return Ok(caravan)
            };

            let TokenTree::Punct(token) = token else {
                return Err(())
            };

            if token != LINE_BREAK {
                return Err(())
            }

            return Ok(caravan)
        },
        TokenTree::Punct(_) => {
            if bang_spacing == Spacing::Alone {
                return one_line_escape(token, caravan, exit_rule)
            }

            if token.to_string() != EXIT_RULE_NOTATION.to_string() {
                return one_line_escape(token, caravan, exit_rule);
            }

            // Match mode confirmed.

            todo!()
        },
        _ => {
            return one_line_escape(token, caravan, exit_rule)
        },
    }


}

fn one_line_escape(token: TokenTree, caravan: TokenIter, exit_rule: &mut TokenStream) -> Result<TokenIter, ()> {
    let mut output = Vec::new();
    output.push(token);

    let (end, caravan, mut new_exit_rule) = collect_until_matching_punct(LINE_BREAK, caravan, output);

    match new_exit_rule.get(0) { // If token 0 is just an "r" then it will be re-created as a "return"
        Some(exit_rule_0) => {
            if exit_rule_0.to_string() == ABBREVIATED_RETURN {
                new_exit_rule[0] = TokenTree::Ident(Ident::new("return", exit_rule_0.span()));
            }
        },
        None => {/* Do nothing */},
    };
            
    exit_rule.extend(new_exit_rule.into_iter());

    match end {
        PunctMatch::Matching => {
            return Ok(caravan)
        },
        PunctMatch::NotMatching => {
            return Err(())
        },
    }
}