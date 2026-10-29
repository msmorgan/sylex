//! Characteristic computations mirrored from `Semantics/Primitives.lean`.
use crate::ast::AbilityLost;
use crate::ast::AbilitySelection;
use crate::ast::CharacteristicEdit;
use crate::ast::CharacteristicStat;
use crate::ast::Characteristics;
use crate::ast::ColorSpec;
use crate::ast::Delta;
use crate::ast::QualityOp;
use crate::ast::TypeLineChanges;

pub(super) fn type_changes(c: &Characteristics) -> TypeLineChanges {
    TypeLineChanges {
        supertypes: (!c.supertypes.is_empty()).then(|| c.supertypes.clone()),
        types: (!c.types.is_empty()).then(|| c.types.clone()),
        subtypes: (!c.subtypes.is_empty()).then(|| c.subtypes.clone()),
    }
}
pub(super) fn value_edits(c: &Characteristics, op: &QualityOp) -> Vec<CharacteristicEdit> {
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

#[derive(Debug, serde::Deserialize)]
pub(super) enum QualityPayload {
    Bundle {
        value: crate::ast::CharacteristicBundle,
        retained: Option<crate::ast::CardType>,
    },
    Quality {
        value: crate::ast::WrittenQuality,
    },
    Colored {
        value: ColorSpec,
    },
}
impl QualityPayload {
    pub(super) fn edits(self, op: QualityOp) -> Vec<CharacteristicEdit> {
        match self {
            Self::Bundle { value, retained } => {
                let mut edits = vec![CharacteristicEdit::TypeLine {
                    op: if retained.is_some() { QualityOp::Adds } else { op.clone() },
                    changes: type_changes(&value.characteristics),
                }];
                edits.extend(value_edits(&value.characteristics, &op));
                edits.extend(
                    value
                        .qualities
                        .into_iter()
                        .map(|q| CharacteristicEdit::Quality {
                            op: op.clone(),
                            quality: Box::new(q),
                        }),
                );
                edits
            }
            Self::Quality { value } => vec![CharacteristicEdit::Quality {
                op,
                quality: Box::new(value),
            }],
            Self::Colored { value } => vec![CharacteristicEdit::Colors { op, colors: value }],
        }
    }
}
