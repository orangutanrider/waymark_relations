use proc_macro::*;

// Entity clause wildcards
pub(crate) const DIRECT: char = '|'; // Default option, given there is no binding. Dennotes a component pointing to an entity, the component is used to feed its entity data into the query, immediatley.
pub(crate) const LITERAL: char = '@'; // Dennotes a literal entity binding. The literal entity is used with the query.
pub(crate) const DE_REF_LITERAL: char = '*'; // Dennotes a literal entity binding. The literal entity is de-referenced and used with the query.
pub(crate) const LIFT: char = '^'; // Dennotes a component pointing to an entity. The component is used to create an entity binding without shadowing the component binding.
pub(crate) const OVERLAP: char = '~'; // Dennotes a component pointing to an entity. The component is used to create an entity binding that shadows the component binding.

// Delimiters
//pub(crate) const BINDINGS_DELIMITER: Delimiter = Delimiter::Parenthesis;
pub(crate) const EXIT_RULE_DELIMITER: Delimiter = Delimiter::Brace;
pub(crate) const ENTIY_STEP_SCOPABLE_DELIMITER: Delimiter = Delimiter::Brace;
pub(crate) const RAW_INPUT_DELIMITER: Delimiter = Delimiter::Bracket;

// Symbols
pub(crate) const LINE_BREAK: char = ';';
pub(crate) const SCOPED_BREAK: char = ',';

pub(crate) const NEXT: [char; 2] = ['-', '>']; // ->
pub(crate) const INTO_NEXT: [char; 2] = ['=', '>']; // =>
// The system only knows the difference between the above two combo tokens by the character of their first symbol. 
// If they share the same character type for this, the system will break.
pub(crate) const NEXT_BANG: char = NEXT[0]; 
pub(crate) const INTO_BANG: char = INTO_NEXT[0];

pub(crate) const EXIT_RULE_NOTATION: char = '?';
pub(crate) const ERR_MATCH_NOTATION: char = '?'; // If this token is joined to the exit_rule notation as a combo (e.g. '??') then it'll do a match statement with err instead of a let else.
pub(crate) const ABBREVIATED_RETURN: &str = "r";

pub(crate) const ENTITY_TO_QUERY_PUNCT: [char; 2] = [':', ':']; // ::
pub(crate) const QUERY_TO_BINDINGS_PUNCT: char = '=';

pub(crate) const ENTITY_PRE_PROCESS_NOTATION: char = '$';
pub(crate) const ENTITY_PRE_PROCESS_VAR: char = '%';