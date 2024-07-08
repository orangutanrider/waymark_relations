use bevy_caravan::*;

#[test]
fn basic_ref_caravan() {
    ref_caravan!([Type::static_method_entity()] :: query = bindings;)
    assert_ref_caravan!((
        [Type::static_method_entity()] :: query = bindings;
    ) (

    ));
}
