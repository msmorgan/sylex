//! Readings of the authoring constructors whose meaning is a normalization the checker
//! makes downstream, mirrored from the Lean checker (`plugins-v3-first-order-templates`).
use deckmaste_semantics_v3::ast::Ability;
use deckmaste_semantics_v3::ast::AbilityLost;
use deckmaste_semantics_v3::ast::AbilitySelection;
use deckmaste_semantics_v3::ast::Action;
use deckmaste_semantics_v3::ast::CharacteristicEdit;
use deckmaste_semantics_v3::ast::CharacteristicStat;
use deckmaste_semantics_v3::ast::Characteristics;
use deckmaste_semantics_v3::ast::ColorSpec;
use deckmaste_semantics_v3::ast::CopyExcept;
use deckmaste_semantics_v3::ast::Cost;
use deckmaste_semantics_v3::ast::Delta;
use deckmaste_semantics_v3::ast::DetPhrase;
use deckmaste_semantics_v3::ast::KeywordTerm;
use deckmaste_semantics_v3::ast::NounPhrase;
use deckmaste_semantics_v3::ast::PayTimes;
use deckmaste_semantics_v3::ast::PaymentOutcome;
use deckmaste_semantics_v3::ast::QualityOp;
use deckmaste_semantics_v3::ast::QualityPayload;
use deckmaste_semantics_v3::ast::TypeLineChanges;

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

/// `Characteristics.typeChanges`: only the written type-line axes.
fn type_changes(c: &Characteristics) -> TypeLineChanges {
    TypeLineChanges {
        supertypes: (!c.supertypes.is_empty()).then(|| c.supertypes.clone()),
        types: (!c.types.is_empty()).then(|| c.types.clone()),
        subtypes: (!c.subtypes.is_empty()).then(|| c.subtypes.clone()),
    }
}

/// `Characteristics.valueEdits`: the value characteristics `c` writes, each with `op`; a
/// removal lowers a number rather than removing it.
fn value_edits(c: &Characteristics, op: &QualityOp) -> Vec<CharacteristicEdit> {
    let value_op = if *op == QualityOp::Loses { QualityOp::Loses } else { QualityOp::Sets };
    let mut edits = Vec::new();
    if let Some(name) = c.name {
        edits.push(CharacteristicEdit::Name {
            op: value_op.clone(),
            name,
        });
    }
    if let Some(cost) = &c.cost {
        edits.push(CharacteristicEdit::ManaCost {
            cost: (*op != QualityOp::Loses).then(|| cost.clone()),
        });
    }
    if !c.colors.is_empty() {
        edits.push(CharacteristicEdit::Colors {
            op: value_op.clone(),
            colors: ColorSpec::Some {
                colors: c.colors.clone(),
            },
        });
    }
    for (axis, amount) in [
        (CharacteristicStat::Power, &c.power),
        (CharacteristicStat::Toughness, &c.toughness),
        (CharacteristicStat::Loyalty, &c.loyalty),
        (CharacteristicStat::Defense, &c.defense),
    ] {
        if let Some(amount) = amount {
            let amount = amount.clone();
            edits.push(CharacteristicEdit::Stat {
                axis,
                change: if *op == QualityOp::Loses {
                    Delta::Down { amount }
                } else {
                    Delta::Set { amount }
                },
            });
        }
    }
    if !c.text.is_empty() {
        edits.push(if *op == QualityOp::Loses {
            CharacteristicEdit::RemovedAbilities {
                selection: Box::new(AbilitySelection::Specified {
                    abilities: c
                        .text
                        .iter()
                        .map(|a| AbilityLost::Written {
                            ability: Box::new(a.clone()),
                        })
                        .collect(),
                }),
            }
        } else {
            CharacteristicEdit::AddedAbilities {
                abilities: c.text.clone(),
            }
        });
    }
    edits
}

/// `QualityPayload.edits`: the typed edits `op` written once over quality wording makes.
pub trait QualityPayloadEditsExt {
    fn edits(&self, op: &QualityOp) -> Vec<CharacteristicEdit>;
}
impl QualityPayloadEditsExt for QualityPayload {
    fn edits(&self, op: &QualityOp) -> Vec<CharacteristicEdit> {
        match self {
            Self::Bundle {
                characteristics,
                retained,
            } => {
                let mut edits = vec![CharacteristicEdit::TypeLine {
                    op: if retained.is_some() { QualityOp::Adds } else { op.clone() },
                    changes: type_changes(&characteristics.characteristics),
                }];
                edits.extend(value_edits(&characteristics.characteristics, op));
                edits.extend(characteristics.qualities.iter().map(|q| {
                    CharacteristicEdit::Quality {
                        op: op.clone(),
                        quality: Box::new(q.clone()),
                    }
                }));
                edits
            }
            Self::Quality { quality } => vec![CharacteristicEdit::Quality {
                op: op.clone(),
                quality: quality.clone(),
            }],
            Self::Colored { colors } => vec![CharacteristicEdit::Colors {
                op: op.clone(),
                colors: colors.clone(),
            }],
        }
    }
}

/// `CharacteristicEdit.expanded`: a payload is the edits it writes; a keyword term is the keyword
/// added, or, for a family, its variants selected from all the objects `holders` describes
/// [CR#702.1c]; any other edit is itself.
pub trait CharacteristicEditExpansionExt {
    fn expanded(&self) -> Vec<CharacteristicEdit>;
}
impl CharacteristicEditExpansionExt for CharacteristicEdit {
    fn expanded(&self) -> Vec<CharacteristicEdit> {
        match self {
            Self::Payload { op, payload } => payload.edits(op),
            Self::AddedKeywordTerm {
                term: KeywordTerm::The { keyword },
                ..
            } => vec![Self::AddedAbilities {
                abilities: vec![Ability::Keyword {
                    keyword: keyword.clone(),
                    params: Vec::new(),
                    body: Vec::new(),
                }],
            }],
            Self::AddedKeywordTerm { term, holders } => vec![Self::AddedSelectionOf {
                source: Box::new(NounPhrase::Described {
                    determiner: Box::new(DetPhrase::All),
                    predicate: holders.clone(),
                }),
                selection: Box::new(AbilitySelection::Specified {
                    abilities: vec![AbilityLost::Term {
                        keyword: term.clone(),
                    }],
                }),
            }],
            edit => vec![edit.clone()],
        }
    }
}

/// `CharacteristicEdit.expandAll`: an edit list as the edits it writes.
#[must_use]
pub fn expand_edits(edits: &[CharacteristicEdit]) -> Vec<CharacteristicEdit> {
    edits
        .iter()
        .flat_map(CharacteristicEditExpansionExt::expanded)
        .collect()
}

/// `CopyExcept.expanded`: "except it has <values>" is a type line when any type is written,
/// the written values set, and each written ability added; any other exception is itself.
pub trait CopyExceptExpansionExt {
    fn expanded(&self) -> Vec<CopyExcept>;
}
impl CopyExceptExpansionExt for CopyExcept {
    fn expanded(&self) -> Vec<CopyExcept> {
        let Self::Characteristics {
            values,
            types_added,
        } = self
        else {
            return vec![self.clone()];
        };
        let mut edits = Vec::new();
        if !(values.supertypes.is_empty() && values.types.is_empty() && values.subtypes.is_empty())
        {
            edits.push(CharacteristicEdit::TypeLine {
                op: if *types_added { QualityOp::Adds } else { QualityOp::Sets },
                changes: type_changes(values),
            });
        }
        let untexted = Characteristics {
            text: Vec::new(),
            ..values.as_ref().clone()
        };
        edits.extend(value_edits(&untexted, &QualityOp::Sets));
        let mut exceptions = Vec::new();
        if !edits.is_empty() {
            exceptions.push(CopyExcept::Edits { edits });
        }
        exceptions.extend(values.text.iter().map(|ability| CopyExcept::Ability {
            ability: Box::new(ability.clone()),
        }));
        exceptions
    }
}
