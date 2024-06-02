use std::str::*;
use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

pub(crate) fn compile_error_stream(msg: &str) -> TokenStream {
    let Ok(stream) = TokenStream::from_str(&("compile_error!(\"".to_owned() + msg + "\")")) else {
        panic!("Unexpected lex error while trying to create a compile_error! token stream.")
    };

    return stream;
}

pub(crate) fn collect_until_punct_combo(
    punct_combo: Vec<char>,
    iter: TokenIter,
) -> ComboFound {
    return until_matching_punct_combo(punct_combo, iter, Vec::new())
}

/// (Input iter, Collected tokens)
pub(crate) enum ComboFound {
    WasFound((TokenIter, Vec<TokenTree>)),
    WasNeverFound((TokenIter, Vec<TokenTree>)),
}

/// Does not search recursively into groups.
/// (Input iter, Collected tokens)
fn until_matching_punct_combo(
    punct_combo: Vec<char>,
    mut iter: TokenIter,
    mut output: Vec<TokenTree>,
) -> ComboFound {
    // Next token.
    // If matching combo, enter a loop that iterates, checking that each following token is valid.
    // If invalid, add to output and recur.
    // If reach the end of iter, without finding combo, return nothing.

    // Next token.
    let token: Option<TokenTree> = iter.next();
    let Some(token) = token else {
        return ComboFound::WasNeverFound((iter, output)); // Punct combo was never found.
    };

    let TokenTree::Punct(token) = token else { // Is punct?
        output.push(token);
        return until_matching_punct_combo(punct_combo, iter, output); // If not add to output and continue.
    };

    match token.spacing() { // Is apart of a punct combo?
        Spacing::Joint => { /* Proceed */},
        Spacing::Alone => {
            output.push(TokenTree::Punct(token));
            return until_matching_punct_combo(punct_combo, iter, output); // If not add to output and continue.
        },
    }

    let combo_iter = punct_combo.iter();
    match validate_punct_combo(combo_iter, iter) { // Validate punct combo.
        Ok((iter, _)) => { // Found match, finish.
            // Collection of combo is not needed.
            return ComboFound::WasFound((iter, output))
        }, 
        Err((iter, mut collection)) => { // Recur.
            output.append(&mut collection); // Collect non-matching combo results.
            return until_matching_punct_combo(punct_combo, iter, output) 
        }, 
    }
}

/// Does not search recursively into groups.
/// (Input iter, Collected tokens)
pub(crate) fn validate_punct_combo(
    combo_iter: core::slice::Iter<char>,
    iter: TokenIter,
) -> Result<(TokenIter, Vec<TokenTree>), (TokenIter, Vec<TokenTree>)> {
    match collect_until_punct_combo_match_fail(combo_iter, iter, Vec::new(), Spacing::Joint) {
        PunctComboMatch::Matching((iter, collection)) => return Ok((iter, collection)),
        PunctComboMatch::NotMatching((iter, collection)) => return Err((iter, collection)),
        PunctComboMatch::ConnectedMatch((iter, collection)) => return Err((iter, collection)),
    }
}

/// (Input iter, Collected tokens)
enum PunctComboMatch{
    Matching((TokenIter, Vec<TokenTree>)),
    NotMatching((TokenIter, Vec<TokenTree>)),
    ConnectedMatch((TokenIter, Vec<TokenTree>)),
}

fn collect_until_punct_combo_match_fail(
    mut combo_iter: core::slice::Iter<char>,
    mut iter: TokenIter,
    mut output: Vec<TokenTree>,
    previous_spacing: Spacing,
) -> PunctComboMatch {
    let Some(combo) = combo_iter.next() else { 
        // If none, then a potential match has been found. However, it checks to see if there are following connected puncts, as this may not be desired.
        match previous_spacing {
            Spacing::Joint => return PunctComboMatch::ConnectedMatch((iter, output)),
            Spacing::Alone => return PunctComboMatch::Matching((iter, output)),
        }
    };

    match previous_spacing {
        Spacing::Joint => {/* Proceed */},
        Spacing::Alone => return PunctComboMatch::NotMatching((iter, output)), // Spacing has been broken in the previous iteration, and the combo has not been found.
    }

    let Some(token) = iter.next() else {
        return PunctComboMatch::NotMatching((iter, output)) // End of input iter reached, and a match has not been found.
    };

    let TokenTree::Punct(token) = token else { // Is punct?
        output.push(token);
        return PunctComboMatch::NotMatching((iter, output)) // Was not punct, the combo has been broken.
    };

    if token != *combo {
        output.push(TokenTree::Punct(token));
        return PunctComboMatch::NotMatching((iter, output)) // Combo does not match the combo given.
    }

    match token.spacing() {
        Spacing::Joint => {/* Proceed */},
        Spacing::Alone => {
            // It might be the end of the combo, so recur to check that.
            output.push(TokenTree::Punct(token));
            return collect_until_punct_combo_match_fail(combo_iter, iter, output, Spacing::Alone)
        },
    }

    // Recur.
    output.push(TokenTree::Punct(token));
    return collect_until_punct_combo_match_fail(combo_iter, iter, output, previous_spacing);
}

/// (Input stream, Processed tokens, Found ident)
pub(crate) fn until_ident(iter: TokenIter) -> (TokenIter, Vec<TokenTree>, Option<Ident>) {
    return collect_until_ident(iter, Vec::new())
}

fn collect_until_ident(mut iter: TokenIter, mut collection: Vec<TokenTree>) -> (TokenIter, Vec<TokenTree>, Option<Ident>) {
    let Some(token) = iter.next() else {
        return (iter, collection, None) // Ident has not been found, and the entire input has been processed.
    };

    match token {
        TokenTree::Ident(ident) => return (iter, collection, Some(ident)),
        _ => { // Non ident
            collection.push(token); // Add to collection
            return collect_until_ident(iter, collection) // And recur.
        },
    }
}