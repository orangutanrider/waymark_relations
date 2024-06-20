use bevy_ref_caravan::*;

#[test]
fn single_line_override_ref_caravan() {
    assert_ref_caravan!((
        to_oranges :: oranges_q = oranges ? return;
    ) (
        let Ok(oranges) = oranges_q.get(to_oranges.go()) else {
            return
        };
    ));
}

#[test]
fn scoped_override_ref_caravan() {
    assert_ref_caravan!((
        to_oranges :: oranges_q = oranges ? { foo = bar + 1; return; };
    ) (
        let Ok(oranges) = oranges_q.get(to_oranges.go()) else {
            foo = bar + 1; 
            return;
        };
    ));
}

#[test]
fn override_declared_ref_caravan() {
    assert_ref_caravan!((
        ? return;
        to_oranges :: oranges_q = oranges;
        to_apples :: apples_q = apples ? continue;
        bananas :: bananas_q = bananas;
    ) (
        let Ok(oranges) = oranges_q.get(to_oranges.go()) else {
            return
        };
        let Ok(apples) = apples_q.get(to_apples.go()) else {
            continue
        };
        let Ok(bananas) = bananas_q.get(to_bananas.go()) else {
            return
        };
    ));
}

#[test]
fn override_into_nested_ref_caravan() {
    assert_ref_caravan!((
        to_hub :: hub_q = (to_oranges, to_apples) ? return => {
            to_oranges :: oranges_q = oranges,
            to_apples :: apples_q = apples,
        }
    ) (
        let Ok((to_oranges, to_apples)) = hub_q.get(to_hub.go()) else {
            return
        };
        let Ok(oranges) = oranges_q.get(to_oranges.go()) else {
            continue;
        };
        let Ok(apples) = apples_q.get(to_apples.go()) else {
            continue;
        };
    ));
}

#[test]
fn override_nested_ref_caravan() {
    assert_ref_caravan!((
        to_hub :: hub_q = (to_oranges, to_apples) ? return => {
            to_oranges :: oranges_q = oranges,
            to_apples :: apples_q = apples ? { foo = bar + 1; return; },
        }
    ) (
        let Ok((to_oranges, to_apples)) = hub_q.get(to_hub.go()) else {
            return
        };
        let Ok(oranges) = oranges_q.get(to_oranges.go()) else {
            continue;
        };
        let Ok(apples) = apples_q.get(to_apples.go()) else {
            foo = bar + 1; 
            return;
        };
    ));
}