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
pub(crate) const LIFT: char = '^'; // Dennotes a component pointing to an entity. The component is used to create an entity binding without shadowing the component binding.
pub(crate) const OVERLAP: char = '~'; // Dennotes a component pointing to an entity. The component is used to create an entity binding that shadows the component binding.
pub(crate) const LITERAL: char = '@'; // Dennotes a literal entity binding. The literal entity is used with the query.

// Delimiters
pub(crate) const BINDINGS_DELIMITER: Delimiter = Delimiter::Parenthesis;
pub(crate) const EXIT_RULE_DELIMITER: Delimiter = Delimiter::Parenthesis;
pub(crate) const QUERY_STEP_SCOPABLE_DELIMITER: Delimiter = Delimiter::Brace;
pub(crate) const ENTIY_STEP_SCOPABLE_DELIMITER: Delimiter = Delimiter::Brace;

// Symbols
pub(crate) const INPUT_LINE_BREAK: char = ';';
pub(crate) const EXIT_RULE_NOTATION: char = '?';
pub(crate) const ENTITY_TO_QUERY_PUNCT_1: char = ':'; // ::
pub(crate) const ENTITY_TO_QUERY_PUNCT_2: char = ':';
pub(crate) const NEXT_PATTERN_PUNCT_1: char = '-'; // ->
pub(crate) const NEXT_PATTERN_PUNCT_2: char = '>'; 