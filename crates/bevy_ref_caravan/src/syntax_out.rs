use proc_macro::TokenStream;
use std::str::FromStr;
use crate::common::compile_error_stream;

pub(crate) const TO_ENTITY_FN: &str = ".go()"; // The expected function call on components, to get their stored entity destination.

// When a lift is used, the resulting entity binding from the component derives its name from the component's binding name.
pub(crate) const LIFT_PREFIX_REMOVE: &str = "to_"; // If the component's binding name has this at the start, that will be removed, to create a distinct name for the new entity binding.
pub(crate) const LIFT_SUFFIX_ADD: &str = "_dest"; // Otherwise, this is added to the end, to create a distinct name for the new entity binding.

pub(crate) const EXIT_RULE_DEFAULT: &str = "continue;";  // In the else statement for the query.get(), this is inserted by default, customisable via exit rules.

pub(crate) fn exit_rule_default() -> TokenStream {
    let Ok(default) = TokenStream::from_str(EXIT_RULE_DEFAULT) else {
        // An unexpected internal error: The default exit rule "EXIT_RULE_DEFAULT", defined within ref_caravan, has failed to be parsed into a token stream, creating a lex error.
        // An unexpected internal error: The default exit rule \"EXIT_RULE_DEFAULT\", defined within ref_caravan, has failed to be parsed into a token stream, creating a lex error.
        let err_msg = "An unexpected internal error: The default exit rule \\".to_owned() + "\"" + EXIT_RULE_DEFAULT + "\\" + "\", defined within ref_caravan, has failed to be parsed into a token stream, creating a lex error.";
        
        return compile_error_stream(&err_msg);
    };
    return default;
}