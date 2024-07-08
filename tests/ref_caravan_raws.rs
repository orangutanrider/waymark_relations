use bevy_caravan::*;

#[test]
fn basic_ref_caravan() {
    assert_ref_caravan!((
        [Type::static_method_entity()] :: query = bindings;
    ) (
        let Ok(bindings) = query.get(Type :: static_method_entity()) else {
            continue;
        };
    ));
}