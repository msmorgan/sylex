//! The downstream readings of the authoring constructors that replaced computed Template bodies
//! (`plugins-v3-first-order-templates`): each reads to what the Template computed before.
use std::path::Path;

use deckmaste_lowering_v3::reads::CostPaymentExt;
use deckmaste_semantics_v3::ast::Action;
use deckmaste_semantics_v3::ast::Cost;
use deckmaste_semantics_v3::bundle::Bundle;
use deckmaste_semantics_v3::lower::Elaborator;

fn builtin() -> Bundle {
    static BUNDLE: std::sync::OnceLock<Bundle> = std::sync::OnceLock::new();
    BUNDLE
        .get_or_init(|| {
            Bundle::load(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v3/builtin"))
                .unwrap()
        })
        .clone()
}

fn paid_cost(reader: &Elaborator<'_>, action: &str) -> Cost {
    let Action::Pay { cost, .. } = reader.read::<Action>(action).unwrap() else {
        panic!("{action} is not a payment")
    };
    assert!(matches!(cost.as_ref(), Cost::AsPaid { .. }), "{action}");
    *cost
}

#[test]
fn a_payment_of_a_plain_payment_reads_as_that_payment() {
    let bundle = builtin();
    let reader = Elaborator::new(&bundle);
    for (action, payment) in [
        ("pay (payLife 1) once", "perform (loseLife 1)"),
        ("pay (inCaller (payLife 5)) once", "inCaller (perform (loseLife 5))"),
        (
            "pay (scaled (inCaller (payLife 2)) 1) once",
            "scaled (inCaller (perform (loseLife 2))) 1",
        ),
        ("pay [[1], payLife 2] once", "[[1], perform (loseLife 2)]"),
        (
            "pay (perform (sacrifice (a isCreature))) once",
            "perform (sacrifice (a isCreature))",
        ),
    ] {
        assert_eq!(
            paid_cost(&reader, action).payment(),
            reader.read::<Cost>(payment).unwrap(),
            "{action}"
        );
    }
}
