use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

use crate::common::collect_until_punct::*;
use crate::syntax_in::LINE_BREAK;

pub(crate) fn exit_rule_step(
    mut caravan: TokenIter, 
    exit_rule: &mut TokenStream,
) -> Result<TokenIter, ()> {
    let Some(token) = caravan.next() else {
        return Err(())
    };

    match token {
        TokenTree::Group(group) => {
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
        _ => {
            let mut output = Vec::new();
            output.push(token);

            let (end, caravan, new_exit_rule) = collect_until_matching_punct(LINE_BREAK, caravan, output);

            exit_rule.extend(new_exit_rule.into_iter());

            match end {
                PunctMatch::Matching => {
                    return Ok(caravan)
                },
                PunctMatch::NotMatching => {
                    return Err(())
                },
            }
        },
    }


}