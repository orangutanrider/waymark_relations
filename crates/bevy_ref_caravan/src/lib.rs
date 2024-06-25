mod common; use common::compile_error_stream;

mod syntax_in; 
mod syntax_out; use syntax_out::exit_rule_default;

// Caravan.
mod root_step; use root_step::root_step;
mod exit_rule_step;
mod entity_step;
mod into_next;
mod nesting_exit;
mod query_step;
mod bindings_step;
mod exit_rule_override_step;
mod construction_step;

use proc_macro::*;
// use proc_macro::token_stream::IntoIter as TokenIter;
use std::str::FromStr;

#[proc_macro]
pub fn ref_caravan(input: TokenStream) -> TokenStream {
    let caravan = input.into_iter();
    let package = TokenStream::new();
    let exit_rule = exit_rule_default();
    let (_, package, _) = match root_step(caravan, package, exit_rule) {
        Ok(ok) => ok,
        Err(err) => {
            return compile_error_stream("Undefined")
        },
    };

    return package
}

#[proc_macro]
#[cfg(all(debug_assertions, not(feature = "no_assertions")))]
pub fn assert_ref_caravan(input: TokenStream) -> TokenStream {
    let mut caravan = input.into_iter();

    // Get groups
    let Some(macro_group) = caravan.next() else {
        return compile_error_stream("Failed to get macro group.")
    };
    let Some(expansion_group) = caravan.next() else {
        return compile_error_stream("Failed to get asserted expansion group.")
    };
    let TokenTree::Group(macro_group) = macro_group else {
        return compile_error_stream("Failed to get macro group.")
    };
    let TokenTree::Group(expansion_group) = expansion_group else {
        return compile_error_stream("Failed to get asserted expansion grouo.")
    };

    // Macro group to ref_caravan
    let macro_group = macro_group.stream();
    let macro_group = ref_caravan(macro_group);

    // Compare groups
    let macro_group = macro_group.to_string();
    let expansion_group = expansion_group.stream();
    let expansion_group = expansion_group.to_string();

    match macro_group == expansion_group {
        true => {
            return TokenStream::new() // Succesful assertion
        },
        false => {
            return assertion_panic(macro_group, expansion_group) // The assertion doesn't match
        },
    }
}

fn assertion_panic(macro_group: String, expansion_group: String) -> TokenStream {
    // Create strings
    let macro_stringify = "let macro_group = stringify!(".to_owned() + &macro_group + ");";
    let expansion_stringify = "let expansion_group = stringify!(".to_owned() + &expansion_group + ");";
    let panic = "panic!(\"Expected expansion did not match the assertion. \\n    macro_expansion: \\n{} \\n    asserted_expansion: \\n{}\", macro_group, expansion_group);";

    // Create token streams
    let Ok(macro_stringify) = TokenStream::from_str(&macro_stringify) else {
        panic!("Unexpected lex error inside assertion_panic, when converting macro_stringify to token stream.")
    };
    let Ok(expansion_stringify) = TokenStream::from_str(&expansion_stringify) else {
        panic!("Unexpected lex error inside assertion_panic, when converting expansion_stringify to token stream.")
    };
    let Ok(panic) = TokenStream::from_str(panic) else {
        panic!("Unexpected lex error inside assertion_panic, when converting panic to token stream.")
    };

    // Assemble tokens
    let mut assembly = TokenStream::new();
    assembly.extend(macro_stringify);
    assembly.extend(expansion_stringify);
    assembly.extend(panic);

    return assembly;
}

/* 
/// Insertion vulnerable. Input message is flanked by " ", if the input message contains quotes, then it must also contain extra \ to flag those quotes.
#[cfg(all(debug_assertions, not(feature = "no_assertions")))]
fn panic_stream(msg_insert: &str) -> TokenStream {
    use std::str::FromStr;
    let Ok(stream) = TokenStream::from_str(&("panic!(\"".to_owned() + msg_insert + "\")")) else {
        panic!("Unexpected lex error while trying to create a panic! token stream.")
    };

    return stream;
}
*/