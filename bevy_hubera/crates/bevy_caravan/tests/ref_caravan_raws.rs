use bevy_caravan::*;

#[test]
fn entity_raw_caravan() {
    assert_ref_caravan!((
        [Type::static_method_entity()] :: query = bindings;
    ) (
        let Ok(bindings) = query.get(Type :: static_method_entity()) else {
            continue;
        };
    ));
}

#[test]
fn query_raw_caravan() {
    assert_ref_caravan!((
        entity :: [query] = bindings;
    ) (
        let Ok(bindings) = query.get(entity.go()) else {
            continue;
        };
    ));
}

#[test]
fn bindings_raw_caravan() {
    assert_ref_caravan!((
        entity :: query = [bindings];
    ) (
        let Ok(bindings) = query.get(entity.go()) else {
            continue;
        };
    ));
}

#[test]
fn entity_raw_caravan_with_wildcard() {
    assert_ref_caravan!((
        |[Type::static_method_waymark()] :: query = bindings;
    ) (
        let Ok(bindings) = query.get(Type :: static_method_waymark().go()) else {
            continue;
        };
    ));
}

#[test]
fn nested_entity_raw_caravan() {
    assert_ref_caravan!((
        to_hub :: hub_q = bindings -> {
            [Type::static_method_entity()] :: q1 = bindings,
            waymark :: q2 = bindings, 
        }
    ) (
        let Ok(bindings) = hub_q.get(to_hub.go()) else {
            continue;
        };
        let Ok(bindings) = q1.get(Type :: static_method_entity()) else {
            continue;
        };
        let Ok(bindings) = q2.get(waymark.go()) else {
            continue;
        };
    ));
}