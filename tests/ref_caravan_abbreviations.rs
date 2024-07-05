use bevy_caravan::*;

#[test]
fn abbreviated_return_exit_rule() {
    assert_ref_caravan!((
        ? r;
        to_oranges :: oranges_q = oranges;
    ) (
        let Ok(oranges) = oranges_q.get(to_oranges.go()) else {
            return
        };
    ));
}

#[test]
fn override_abbreviated_return_exit_rule() {
    assert_ref_caravan!((
        to_oranges :: oranges_q = oranges ?r;
    ) (
        let Ok(oranges) = oranges_q.get(to_oranges.go()) else {
            return
        };
    ));
}

#[test]
fn nothing_exit_rule() {
    assert_ref_caravan!((
        ? return;
        to_apples :: apples_q = apples;
        ?;
        to_oranges :: oranges_q = oranges;
    ) (
        let Ok(apples) = apples_q.get(to_apples.go()) else {
            return
        };

        let Ok(oranges) = oranges_q.get(to_oranges.go()) else {
            continue;
        };
    ));
}

#[test]
fn override_nothing_exit_rule() {
    assert_ref_caravan!((
        ? return;
        to_apples :: apples_q = apples;
        to_oranges :: oranges_q = oranges ?;
    ) (
        let Ok(apples) = apples_q.get(to_apples.go()) else {
            return
        };

        let Ok(oranges) = oranges_q.get(to_oranges.go()) else {
            continue;
        };
    ));
}

#[test]
fn err_match_nothing_exit_rule() {
    assert_ref_caravan!((
        ??;
        to_oranges :: oranges_q = oranges;
    ) (
        let oranges = match oranges_q.get(to_oranges.go()) {
            Ok(ok) => ok,
            Err(err) => {return Err(err)},
        };
    ));
}

#[test]
fn override_err_match_nothing_exit_rule() {
    assert_ref_caravan!((
        to_oranges :: oranges_q = oranges ??;
    ) (
        let oranges = match oranges_q.get(to_oranges.go()) {
            Ok(ok) => ok,
            Err(err) => {return Err(err)},
        };
    ));
}