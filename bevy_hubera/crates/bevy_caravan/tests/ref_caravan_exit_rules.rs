use bevy_caravan::*;

// Expected valid statements.
#[test]
fn single_line_exit_rule_ref_carvan() {
    assert_ref_caravan!((
        ? return;
        to_oranges :: oranges_q = oranges;
    ) (
        let Ok(oranges) = oranges_q.get(to_oranges.go()) else {
            return
        };
    ));

    // It is specifically without the line break at the end
}

#[test]
fn scoped_exit_rule_ref_carvan() {
    assert_ref_caravan!((
        ? {
            foobar = foobar + 1;
            return;
        };
        to_oranges :: oranges_q = oranges;
    ) (
        let Ok(oranges) = oranges_q.get(to_oranges.go()) else {
            foobar = foobar + 1;
            return;
        };
    ));
}

#[test]
fn multi_exit_rule_ref_carvan() {
    assert_ref_caravan!((
        to_oranges :: oranges_q = oranges;
        to_bananas :: bananas_q = bananas;
        ? return;
        to_apples :: apples_q = apples;
        to_carrots :: carrots_q = carrots;
        ? {
            foobar = foobar + 1;
            return;
        };
        to_lemons :: lemons_q = lemons;
        to_grapes :: grapes_q = grapes;
    ) (
        let Ok(oranges) = oranges_q.get(to_oranges.go()) else {
            continue;
        };
        let Ok(bananas) = bananas_q.get(to_bananas.go()) else {
            continue;
        };
        let Ok(apples) = apples_q.get(to_apples.go()) else {
            return
        };
        let Ok(carrots) = carrots_q.get(to_carrots.go()) else {
            return
        };
        let Ok(lemons) = lemons_q.get(to_lemons.go()) else {
            foobar = foobar + 1;
            return;
        };
        let Ok(grapes) = grapes_q.get(to_grapes.go()) else {
            foobar = foobar + 1;
            return;
        };
    ));
}

// Expected invalid statements.
#[test]
fn incorrect_delimiter() {
    assert_ref_caravan!((
        ? (
            foobar = foobar + 1;
            return;
        );
        to_oranges :: oranges_q = oranges;
    ) (
        compile_error!("Undefined")
    ));
}