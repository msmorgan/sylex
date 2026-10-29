//! Readings of the authoring constructors whose meaning is a normalization the checker
//! makes downstream, mirrored from the Lean checker (`plugins-v3-first-order-templates`).
use deckmaste_semantics_v3::ast::Ability;
use deckmaste_semantics_v3::ast::StaticSpec;
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
pub trait CostPaidExt {
    fn paid_cost(&self) -> Cost;
}
impl CostPaidExt for Cost {
    fn paid_cost(&self) -> Cost {
        match self {
            Self::Perform { action } => match action.as_ref() {
                Action::Pay {
                    cost,
                    times: PayTimes::Once,
                    outcome: PaymentOutcome::Paid,
                    whose: None,
                    keyword: None,
                } => cost.paid_cost(),
                _ => self.clone(),
            },
            Self::Payment { cost } => cost.paid_cost(),
            Self::InCaller { body } => Self::InCaller {
                body: Box::new(body.paid_cost()),
            },
            Self::Scaled { cost, amount } => Self::Scaled {
                cost: Box::new(cost.paid_cost()),
                amount: amount.clone(),
            },
            Self::Compound { costs } => Self::Compound {
                costs: costs.iter().map(CostPaidExt::paid_cost).collect(),
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

/// `CharacteristicEdit.unfolded`: a payload is the edits it writes; a keyword term is the keyword
/// added, or, for a family, its variants selected from all the objects `holders` describes
/// [CR#702.1c]; any other edit is itself.
pub trait CharacteristicEditUnfoldExt {
    fn expanded(&self) -> Vec<CharacteristicEdit>;
}
impl CharacteristicEditUnfoldExt for CharacteristicEdit {
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

/// `CharacteristicEdit.unfoldAll`: an edit list as the edits it writes.
#[must_use]
pub fn unfold_edits(edits: &[CharacteristicEdit]) -> Vec<CharacteristicEdit> {
    edits
        .iter()
        .flat_map(CharacteristicEditUnfoldExt::expanded)
        .collect()
}

/// `CopyExcept.unfolded`: "except it has <values>" is a type line when any type is written,
/// the written values set, and each written ability added; any other exception is itself.
pub trait CopyExceptUnfoldExt {
    fn expanded(&self) -> Vec<CopyExcept>;
}
impl CopyExceptUnfoldExt for CopyExcept {
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
            exceptions.push(CopyExcept::CharacteristicEdits { edits });
        }
        exceptions.extend(values.text.iter().map(|ability| CopyExcept::Ability {
            ability: Box::new(ability.clone()),
        }));
        exceptions
    }
}

/// One keyword's reading of a Same-Is-True-For frame: `frame` with `keyword` for each
/// `thatKeyword` [CR#702.1c].
///
/// # Panics
/// Never for a well-formed frame: substitution keeps every value of its own type.
#[must_use]
pub fn frame_for(frame: &Ability, keyword: &KeywordTerm) -> Ability {
    fn substitute(value: &mut serde_json::Value, keyword: &serde_json::Value) {
        match value {
            serde_json::Value::String(word) if word == "ThatKeyword" => {
                *value = keyword.clone();
            }
            serde_json::Value::Array(items) => {
                for item in items {
                    substitute(item, keyword);
                }
            }
            serde_json::Value::Object(fields) => {
                for field in fields.values_mut() {
                    substitute(field, keyword);
                }
            }
            _ => {}
        }
    }
    let mut value = serde_json::to_value(frame).expect("an Ability serializes");
    substitute(
        &mut value,
        &serde_json::to_value(keyword).expect("a KeywordTerm serializes"),
    );
    serde_json::from_value(value).expect("substitution keeps the frame's type")
}

/// The reading of a Same-Is-True-For Ability the checker distributes: each keyword's reading of
/// its frame, joined into one ability [CR#113.2c,702.1c]. A triggered first reading triggers on
/// its own trigger and does each reading's effect in turn; a static first reading is a static
/// ability whose parts are each reading's. A frame of any other kind, or no keyword, joins
/// nothing: an empty conjunction.
pub trait AbilityDistributionExt {
    /// The distributed ability, or `None` for an ability that is no Same-Is-True-For Ability.
    fn distributed(&self) -> Option<Ability>;
}
impl AbilityDistributionExt for Ability {
    fn distributed(&self) -> Option<Ability> {
        let Self::SameIsTrueFor { frame, keywords } = self else {
            return None;
        };
        let parts: Vec<Ability> = keywords.iter().map(|k| frame_for(frame, k)).collect();
        let nothing = || Self::Static {
            spec: Box::new(StaticSpec::Conjunction {
                subject: None,
                parts: Vec::new(),
            }),
        };
        Some(match parts.first() {
            Some(Self::Triggered {
                event,
                alternatives,
                while_,
                joins,
                timing,
                limit,
                intervening,
                ..
            }) => {
                let steps: Vec<Action> = parts
                    .iter()
                    .filter_map(|part| match part {
                        Self::Triggered { action, .. } => Some(action.as_ref().clone()),
                        _ => None,
                    })
                    .collect();
                if steps.len() == parts.len() {
                    Self::Triggered {
                        event: event.clone(),
                        alternatives: alternatives.clone(),
                        while_: while_.clone(),
                        joins: joins.clone(),
                        timing: timing.clone(),
                        limit: limit.clone(),
                        intervening: intervening.clone(),
                        action: Box::new(Action::Sequentially { steps }),
                    }
                } else {
                    nothing()
                }
            }
            Some(Self::Static { .. }) => {
                let specs: Vec<StaticSpec> = parts
                    .iter()
                    .filter_map(|part| match part {
                        Self::Static { spec } => Some(spec.as_ref().clone()),
                        _ => None,
                    })
                    .collect();
                Self::Static {
                    spec: Box::new(StaticSpec::Conjunction {
                        subject: None,
                        parts: if specs.len() == parts.len() { specs } else { Vec::new() },
                    }),
                }
            }
            _ => nothing(),
        })
    }
}
