use proc_macro::*;

// Entity clause wildcards
//pub(crate) const LIFT: char = '^'; // Dennotes a component pointing to an entity. The component is used to create an entity binding without shadowing the component binding.
//pub(crate) const OVERLAP: char = '~'; // Dennotes a component pointing to an entity. The component is used to create an entity binding that shadows the component binding.
//pub(crate) const LITERAL: char = '@'; // Dennotes a literal entity binding. The literal entity is used with the query.

// Delimiters
pub(crate) const BINDINGS_DELIMITER: Delimiter = Delimiter::Parenthesis;
pub(crate) const EXIT_RULE_DELIMITER: Delimiter = Delimiter::Parenthesis;
//pub(crate) const QUERY_STEP_SCOPABLE_DELIMITER: Delimiter = Delimiter::Brace;
//pub(crate) const ENTIY_STEP_SCOPABLE_DELIMITER: Delimiter = Delimiter::Brace;

// Symbols
pub(crate) const LINE_BREAK: char = ';';
pub(crate) const NEXT: char = ',';
pub(crate) const INTO_NEXT: [char; 2] = ['=', '>']; // =>

pub(crate) const EXIT_RULE_NOTATION: char = '?';

pub(crate) const ENTITY_TO_QUERY_PUNCT: [char; 2] = [':', ':']; // \\
pub(crate) const QUERY_TO_BINDINGS_PUNCT: char = '=';
