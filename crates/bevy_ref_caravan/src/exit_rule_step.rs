use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

use crate::common::collect_until_punct::*;
use crate::syntax_in::{ABBREVIATED_RETURN, ERR_MATCH_NOTATION, EXIT_RULE_DELIMITER, LINE_BREAK};
use crate::syntax_out::*;

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

    fn new(statement: TokenStream, structure: ExitStructure) -> Self {
        return Self { statement, structure }
    }
}

pub(crate) fn exit_rule_step(
    caravan: TokenIter, 
    exit_rule: &mut ExitRule,
    bang_spacing: Spacing,
) -> Result<TokenIter, ()> {
    let (_, caravan, proto_exit_rule) = collect_until_matching_punct(LINE_BREAK, caravan, Vec::new()); 
    let new_exit_rule = exit_rule_post_processing(proto_exit_rule, bang_spacing)?;

    exit_rule.wipe();
    exit_rule.structure = new_exit_rule.structure;
    exit_rule.statement.extend(new_exit_rule.statement);

    return Ok(caravan)
}

pub(crate) enum ExitStructure{
    IsErrMatch,
    IsLetElse
}

pub(crate) fn exit_rule_post_processing(
    proto_exit_rule: Vec<TokenTree>,
    bang_spacing: Spacing,
) -> Result<ExitRule, ()> {
    // PSEUDCODE
    // If connected token, check if connected is a ?, if so then save the fact that it is an err_match structure.
    // If next token is a group, validate the delimiter, and save group as exit_rule.
    // Check the first token of the group, if it is an "r" then expand that to a "return".
    // If no group is found, then check the token to see if it is an "r" then expand that to a "return" also save the exit_rule.
    // Return data.

    let len = proto_exit_rule.len(); // It is known that the new exit_rule wont exceed the length of the original.

    let mut iter = proto_exit_rule.into_iter();
    let structure = check_for_err_match_notation(&mut iter, bang_spacing);

    let Some(token) = iter.next() else { // If there is no token, use the default for the respective rules
        match structure {
            ExitStructure::IsErrMatch => return Ok(ExitRule::new(exit_rule_err_default(), structure)),
            ExitStructure::IsLetElse => return Ok(ExitRule::new(exit_rule_default(), structure)),
        }
    };

    // Enter group and expand first token "r" to return
    let exit_rule = match token {
        TokenTree::Group(group) => {
            if group.delimiter() != EXIT_RULE_DELIMITER { return Err(()) }

            let mut iter = group.stream().into_iter();

            let Some(token) = iter.next() else { // If there is no token, use the default for the respective rules
                match structure {
                    ExitStructure::IsErrMatch => return Ok(ExitRule::new(exit_rule_err_default(), structure)),
                    ExitStructure::IsLetElse => return Ok(ExitRule::new(exit_rule_default(), structure)),
                }
            };

            let token = check_and_expand_r(token);
            
            let mut exit_rule = Vec::with_capacity(len); // Could be an array
            exit_rule.push(token);
            for token in iter { exit_rule.push(token) }
            exit_rule
        },
        _ => {
            let token = check_and_expand_r(token);

            let mut exit_rule = Vec::with_capacity(len); // Could be an array
            exit_rule.push(token);
            for token in iter { exit_rule.push(token) }
            exit_rule
        },
    };

    let statement = TokenStream::from_iter(exit_rule.into_iter());

    return Ok(ExitRule::new(statement, structure))
}

fn check_and_expand_r(
    token: TokenTree,
) -> TokenTree {
    if token.to_string() == ABBREVIATED_RETURN {
        return TokenTree::Ident(Ident::new("return", token.span()));
    }

    return token
}

fn check_for_err_match_notation(
    iter: &mut std::vec::IntoIter<TokenTree>,
    bang_spacing: Spacing,
) -> ExitStructure {
    match bang_spacing {
        Spacing::Joint => {
            let Some(combo) = iter.next() else {
                return ExitStructure::IsLetElse // This implies that the following token was a symbol that ended the collection of the exit_rule (i.e. probably a line break)
            };

            if combo.to_string() == ERR_MATCH_NOTATION.to_string() {
                return ExitStructure::IsErrMatch
            };

            return ExitStructure::IsLetElse
        },
        Spacing::Alone => return ExitStructure::IsLetElse,
    }
}