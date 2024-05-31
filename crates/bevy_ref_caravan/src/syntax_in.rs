/// INPUT FORMAT

/// ?(exit_rule);
/// entity::query(bindings) ?(exit_rule_override)-> ...;

/// INPUT DETAILS

/// entity is scopable/repeatable via "{ }" 
/// query is scopabalbe/repeatable via "{ }"

/// The exit_rule is applied to each pattern following its decleration. New exit rules can be declared to replace it for following patterns.
/// Optionally, you can overrided it within a pattern aswell. If none, remove "?(exit_rule_override)" and leave "->".

/// Before the entity, you can use a wildcard (e.g. ^, ~, @). These allow you to describe the entity input and control the shadowing of the binding.

use proc_macro::*;

// Entity clause wildcards
const LIFT: &str = "^"; // Dennotes a component pointing to an entity. The component is used to create an entity binding without shadowing the component binding.
const OVERLAP: &str = "~"; // Dennotes a component pointing to an entity. The component is used to create an entity binding that shadows the component binding.
const LITERAL: &str = "@"; // Dennotes a literal entity. The literal entity is used with the query.

// Delimiters
const BINDINGS_DELIMITER: Delimiter = Delimiter::Parenthesis;
const EXIT_RULE_DELIMITER: Delimiter = Delimiter::Parenthesis;
const QUERY_STEP_SCOPABLE_DELIMITER: Delimiter = Delimiter::Brace;
const ENTIY_STEP_SCOPABLE_DELIMITER: Delimiter = Delimiter::Brace;

// Symbols
const INPUT_LINE_BREAK: &str = ";";
const EXIT_RULE_NOTATION: &str = "?";
const ENTITY_TO_QUERY_PUNCT: &str = "::"; // Expected two characters long (joint symbols) in the code.
const NEXT_PATTERN_PUNCT: &str = "->"; // Expected two characters long (joint symbols) in the code.