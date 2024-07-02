use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

use crate::common::collect_until_punct::*;
use crate::syntax_in::{ABBREVIATED_RETURN, ERR_MATCH_NOTATION, EXIT_RULE_DELIMITER, LINE_BREAK};
use crate::syntax_out::exit_rule_default;

pub(crate) struct ExitRule {
    pub(crate) statement: TokenStream,
    pub(crate) structure: ExitStructure
}
impl Default for ExitRule {
    fn default() -> Self {
        return Self { statement: exit_rule_default(), structure: ExitStructure::IsLetElse }
    }
}
impl ExitRule {
    pub(crate) fn wipe(&mut self) {
        self.statement = TokenStream::new();
        self.structure = ExitStructure::IsLetElse;
    }
}

pub(crate) fn exit_rule_step(
    mut caravan: TokenIter, 
    exit_rule: &mut ExitRule,
    bang_spacing: Spacing,
) -> Result<TokenIter, ()> {
    let (_, caravan, mut collected_exit_rule) = collect_until_matching_punct(LINE_BREAK, caravan, output); 
    todo!(); // Post-processing goes here
}

fn expect_empty_or_line_break(caravan: &mut TokenIter) -> Result<(), ()> {
    let Some(token) = caravan.next() else {
        return Ok(())
    };

    let TokenTree::Punct(token) = token else {
        return Err(())
    };

    if token != LINE_BREAK {
        return Err(())
    }

    Ok(())
}

pub(crate) enum ExitStructure{
    IsErrMatch,
    IsLetElse
}

// Post-processing
// If connected token, check if connected is a ?, if so then save the fact that it is an err_match structure.
// If next token is a group, validate the delimiter, and save group as exit_rule.
// Check the first token of the group, if it is an "r" then expand that to a "return".
// If no group is found, then check the token to see if it is an "r" then expand that to a "return" also save the exit_rule.
// Return data.

pub(crate) fn exit_rule_post_processing(
    exit_rule: &mut Vec<TokenTree>,
) -> ExitStructure {
    match exit_rule.get(0) { // If token 0 is just an "r" then it will be re-created as a "return"
        Some(exit_rule_0) => {
            if exit_rule_0.to_string() == ABBREVIATED_RETURN {
                exit_rule[0] = TokenTree::Ident(Ident::new("return", exit_rule_0.span()));
            }
        },
        None => {/* Do nothing */},
    };

    todo!();
}