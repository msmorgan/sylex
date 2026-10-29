//! Readings of the authoring constructors whose meaning is a normalization the checker
//! makes downstream, mirrored from the Lean checker (`plugins-v3-first-order-templates`).
use deckmaste_semantics_v3::ast::Action;
use deckmaste_semantics_v3::ast::Cost;
use deckmaste_semantics_v3::ast::PayTimes;
use deckmaste_semantics_v3::ast::PaymentOutcome;

/// `Cost.payment` (`Semantics/Abilities.lean`): what paying a cost pays. Paying a cost
/// carries out its instructions [CR#118.1], so a cost that is itself one plain payment is
/// that payment, made once; a Reference Scope, a scaling and a compound's other parts are
/// kept.
pub trait CostPaymentExt {
    fn payment(&self) -> Cost;
}
impl CostPaymentExt for Cost {
    fn payment(&self) -> Cost {
        match self {
            Self::Perform { action } => match action.as_ref() {
                Action::Pay {
                    cost,
                    times: PayTimes::Once,
                    outcome: PaymentOutcome::Paid,
                    whose: None,
                    keyword: None,
                } => cost.payment(),
                _ => self.clone(),
            },
            Self::AsPaid { cost } => cost.payment(),
            Self::InCaller { body } => Self::InCaller {
                body: Box::new(body.payment()),
            },
            Self::Scaled { cost, amount } => Self::Scaled {
                cost: Box::new(cost.payment()),
                amount: amount.clone(),
            },
            Self::Compound { costs } => Self::Compound {
                costs: costs.iter().map(CostPaymentExt::payment).collect(),
            },
            _ => self.clone(),
        }
    }
}
