use bevy_ref_caravan::*;

// Expected valid statements.
#[test]
fn literal_ref_caravan() {
    assert_ref_caravan!((
        @entity :: oranges_q = oranges
    ) (
        let Ok(oranges) = oranges_q.get(entity) else {
            continue;
        };
    ));
}

#[test]
fn direct_ref_caravan() {
    assert_ref_caravan!((
        |to_oranges :: oranges_q = oranges
    ) (
        let Ok(oranges) = oranges_q.get(to_oranges.go()) else {
            continue;
        };
    ));
}

#[test]
fn overlap_ref_caravan() {
    assert_ref_caravan!((
        ~oranges_entity :: oranges_q = oranges
    ) (
        let oranges_entity = oranges_entity.go();
        let Ok(oranges) = oranges_q.get(oranges_entity) else {
            continue;
        };
    ));
}

#[test]
fn prefix_lift_ref_caravan() {
    assert_ref_caravan!((
        ^to_oranges_entity :: oranges_q = oranges
    ) (
        let oranges_entity = to_oranges_entity.go();
        let Ok(oranges) = oranges_q.get(oranges_entity) else {
            continue;
        };
    ));
}

#[test]
fn suffix_lift_ref_caravan() {
    assert_ref_caravan!((
        ^oranges_entity_waymark :: oranges_q = oranges
    ) (
        let oranges_entity_waymark_dest = oranges_entity_waymark.go();
        let Ok(oranges) = oranges_q.get(oranges_entity_waymark_dest) else {
            continue;
        };
    ));
}

#[test]
fn nested_wildcards_ref_caravan() {
    assert_ref_caravan!((
        @hub :: hub_q = (to_oranges_entity, apples_entity, to_carrots, to_onions) -> {
            ^to_oranges_entity :: oranges_q = oranges,
            ~apples_entity :: apples_q = apples,
            to_carrots :: carrots_q = carrots,
            |to_onions :: onions_q = onions,
        }
    ) (
        let Ok((to_oranges_entity, apples_entity, to_carrots, to_onions)) = hub_q.get(hub) else {
            continue;
        };
        let oranges_entity = to_oranges_entity.go();
        let Ok(oranges) = oranges_q.get(oranges_entity) else {
            continue;
        };
        let apples_entity = apples_entity.go();
        let Ok(apples) = apples_q.get(apples_entity) else {
            continue;
        };
        let Ok(carrots) = carrots_q.get(to_carrots.go()) else {
            continue;
        };
        let Ok(onions) = onions_q.get(to_onions.go()) else {
            continue;
        };
    ));
}

// Expected invalid statements.
#[test]
fn wildcards_on_scope() {
    assert_ref_caravan!((
        to_hub :: hub_q = (oranges_entity, apples_entity) -> ~{
            oranges_entity :: oranges_q = oranges,
            apples_entity :: apples_q = apples,
        }
    ) (
        compile_error!("Undefined")
    ));
}