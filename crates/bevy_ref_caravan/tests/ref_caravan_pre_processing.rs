use bevy_ref_caravan::*;

#[test]
fn pre_processing_test() {
    ref_caravan!(
        £ peeled {
            let £ = Some(%) else {
                continue;
            }
        }
        oranges :: oranges_q = oranges;
    )

    assert_ref_caravan!((
        £ peeled {
            let £ = Some(%) else {
                continue;
            }
        }
        oranges :: oranges_q = oranges;
    ) (

    ));
}