use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

pub(crate) enum PunctMatch{
    Matching,
    NotMatching,
    //ConnectedMatch,
}

pub(crate) enum ExactComboFound {
    WasFound,
    WasNeverFound,
}

/* 
/// Does not search recursively into groups.
/// (Result, Input iter, Collected tokens)
pub(crate) fn until_matching_punct(
    punct: char, 
    iter: TokenIter
) -> (PunctMatch, TokenIter, Vec<TokenTree>) {
    return collect_until_matching_punct(punct, iter, Vec::new())
}
*/

/// Does not search recursively into groups.
/// (Result, Input iter, Collected tokens)
pub(crate) fn collect_until_matching_punct(
    punct: char, 
    mut iter: TokenIter,
    mut output: Vec<TokenTree>,
) -> (PunctMatch, TokenIter, Vec<TokenTree>) {
    // Next token.
    let token: Option<TokenTree> = iter.next();
    let Some(token) = token else {
        return (PunctMatch::NotMatching, iter, output); // Match was never found.
    };

    let TokenTree::Punct(token) = token else { // Is punct?
        output.push(token);
        return collect_until_matching_punct(punct, iter, output); // If not add to output and continue.
    };

    if token != punct { // Is match?
        output.push(TokenTree::Punct(token));
        return collect_until_matching_punct(punct, iter, output); // If not add to output and continue.
    }
    
    return (PunctMatch::Matching, iter, output);

    // // Is connected match?
    // match token.spacing() {
    //     Spacing::Joint => return (PunctMatch::ConnectedMatch, iter, output),
    //     Spacing::Alone => return (PunctMatch::Matching, iter, output),
    // }
}

/* 
/// Iterates until a matching punct combo is found.
/// Does not count matches that have additional connected symbols.
/// Does not search recursively into groups.
/// (Result, Input iter, Collected tokens)
pub(crate) fn until_exact_punct_combo(
    punct_combo: Vec<char>,
    iter: TokenIter,
) -> (ExactComboFound, TokenIter, Vec<TokenTree>) {
    return until_exact_combo(punct_combo, iter, Vec::new())
}
*/

/// Iterates until a matching punct combo is found.
/// Does not count matches that have additional connected symbols.
/// Does not search recursively into groups.
/// (Result, Input iter, Collected tokens)
pub(crate) fn until_exact_combo(
    punct_combo: Vec<char>,
    mut iter: TokenIter,
    mut output: Vec<TokenTree>,
) -> (ExactComboFound, TokenIter, Vec<TokenTree>) {
    // Next token.
    // If matching combo, enter a loop that iterates, checking that each following token is valid.
    // If invalid, add to output and recur.
    // If reach the end of iter, without finding combo, return nothing.

    // Next token.
    let token: Option<TokenTree> = iter.next();
    let Some(token) = token else {
        return (ExactComboFound::WasNeverFound, iter, output); // Punct combo was never found.
    };

    let TokenTree::Punct(token) = token else { // Is punct?
        output.push(token);
        return until_exact_combo(punct_combo, iter, output); // If not add to output and continue.
    };

    //match token.spacing() { // Is apart of a punct combo?
    //    Spacing::Joint => {/* Proceed */},
    //    Spacing::Alone => {
    //        output.push(TokenTree::Punct(token));
    //        return until_exact_combo(punct_combo, iter, output); // If not add to output and continue.
    //    },
    //}

    let combo_iter = punct_combo.iter();
    let (result, iter, output) = match_one_punct_combo(combo_iter, iter, token, output);
    match result {
        PunctMatch::Matching => return (ExactComboFound::WasFound, iter, output), // Exact combo found, exit.
        PunctMatch::NotMatching => return until_exact_combo(punct_combo, iter, output), // Recur.
        //PunctMatch::ConnectedMatch => return until_exact_combo(punct_combo, iter, output), // Recur.
    }
}
 
/// Iterates across the iter, matching against the combo, until the end of the combo, or a match fail.
/// Will not continue to iterate after the first combo.
/// Does not search recursively into groups.
/// (Result, Input iter, Collected tokens).
pub(crate) fn match_one_punct_combo(
    mut punct_combo: core::slice::Iter<char>,
    iter: TokenIter,
    current: Punct,
    mut output: Vec<TokenTree>,
) -> (PunctMatch, TokenIter, Vec<TokenTree>) {
    match current.spacing() {
        Spacing::Joint => {/* Proceed */},
        Spacing::Alone => { // Is combo?
            output.push(TokenTree::Punct(current));
            return (PunctMatch::NotMatching, iter, output); // If not, add to output and terminate.
        },
    }

    let Some(combo1) = punct_combo.next() else { // Get combo 1st element.
        output.push(TokenTree::Punct(current));
        return (PunctMatch::NotMatching, iter, output); // If not, add to output and terminate.
    };

    if current != *combo1 { // Is first token matching the combo?
        output.push(TokenTree::Punct(current));
        return (PunctMatch::NotMatching, iter, output); // If not, add to output and terminate.
    }

    return combo_until_fail(punct_combo, iter, output);
}

/// Iterates across the iter, matching against the combo, until the end of the combo, or a match fail.
/// Will not continue to iterate after the first combo.
/// Does not search recursively into groups.
/// (Result, Input iter, Collected tokens).
fn combo_until_fail(
    mut punct_combo: core::slice::Iter<char>,
    mut iter: TokenIter,
    mut output: Vec<TokenTree>,
) -> (PunctMatch, TokenIter, Vec<TokenTree>) {
    let Some(combo) = punct_combo.next() else { 
        return (PunctMatch::NotMatching, iter, output) // Potentially, the combo is a connected match, but this function doesn't detect for that, and treats them as invalid.
    };

    let Some(token) = iter.next() else {
        return (PunctMatch::NotMatching, iter, output) // End of input iter reached, and a match has not been found.
    };

    let TokenTree::Punct(token) = token else { // Is punct?
        output.push(token);
        return (PunctMatch::NotMatching, iter, output) // Was not punct, the combo has been broken.
    };

    if token != *combo {
        output.push(TokenTree::Punct(token));
        return (PunctMatch::NotMatching, iter, output) // Combo does not match the combo given.
    }

    match token.spacing() {
        Spacing::Joint => {/* Proceed */},
        Spacing::Alone => {
            if punct_combo.len() == 0 { // Is at end of combo? If is an alone token and at end of combo, then it is valid.
                return (PunctMatch::Matching, iter, output) // Combo success
            }

            // If alone and not end of combo, combo fail.
            output.push(TokenTree::Punct(token));
            return (PunctMatch::NotMatching, iter, output)
        },
    }

    // Recur.
    output.push(TokenTree::Punct(token));
    return combo_until_fail(punct_combo, iter, output);
}