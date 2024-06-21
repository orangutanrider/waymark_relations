use bevy_ref_caravan::*;

#[test]
fn basic_ref_caravan() {
    assert_ref_caravan!((
        to_oranges :: oranges_q = oranges;
    ) (
        let Ok(oranges) = oranges_q.get(to_oranges.go()) else {
            continue;
        };
    ));
}

#[test]
fn no_break_ref_caravan() {
    assert_ref_caravan!((
        to_oranges :: oranges_q = oranges
    ) (
        let Ok(oranges) = oranges_q.get(to_oranges.go()) else {
            continue;
        };
    ));
}

#[test]
fn multi_ref_caravan() {
    assert_ref_caravan!((
        to_oranges :: oranges_q = oranges;
        to_apples :: apples_q = apples;
    ) (
        let Ok(oranges) = oranges_q.get(to_oranges.go()) else {
            continue;
        };

        let Ok(apples) = apples_q.get(to_apples.go()) else {
            continue;
        };
    ));
}

#[test]
fn mut_ref_caravan() {
    assert_ref_caravan!((
        to_oranges :: oranges_q = mut oranges;
    ) (
        let Ok(mut oranges) = oranges_q.get_mut(to_oranges.go()) else {
            continue;
        };
    ));
}

#[test]
fn into_next_ref_caravan() {
    assert_ref_caravan!((
        to_hub :: hub_q = to_oranges -> to_oranges :: oranges_q = oranges;
    ) (
        let Ok(to_oranges) = hub_q.get(to_hub.go()) else {
            continue;
        };

        let Ok(oranges) = oranges_q.get(to_oranges.go()) else {
            continue;
        };
    ));
}

#[test]
fn comments() {
    // Function-like macros inhernetly support comments.
    // Apart for Doc-comments, but there is no reason to add doc comments inside this macro.
    assert_ref_caravan!((
        to_oranges /*
            Foo
        */
        :: oranges_q = /* Foo */ oranges // Bar
        // FooBar
    ) (
        let Ok(oranges) = oranges_q.get(to_oranges.go()) else {
            continue;
        };
    ));
}