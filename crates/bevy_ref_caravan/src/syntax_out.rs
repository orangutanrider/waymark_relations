/// INPUT FORMAT

/// ?(exit_rule);
/// entity::query(bindings) ?(exit_rule_override)-> ...;

/// OUTPUT FORMAT

/// let bindings = query.get(entity) else {
///     exit_rule
/// }

const TO_ENTITY_FN: &str = ".go()"; // The expected function call on components, to get their stored entity destination.

// When a lift is used, the resulting entity binding from the component derives its name from the component's binding name.
const LIFT_REMOVE: &str = "to_"; // If the component's binding name has this at the start, that will be removed, to create a distinct name for the new entity binding.
const LIFT_ADD: &str = "_dest"; // Otherwise, this is added to the end, to create a distinct name for the new entity binding.

const EXIT_RULE_DEFAULT: &str = "continue";  // In the else statement for the query.get(), this is inserted by default, customisable via exit rules.