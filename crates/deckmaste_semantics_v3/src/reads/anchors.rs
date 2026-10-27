//! Structural anchors for event and participial clauses.
use super::GramRole;
use super::ParticipialAnchor;
use crate::ast::Action;
use crate::ast::CardType;
use crate::ast::Coordinator;
use crate::ast::CoreDeed;
use crate::ast::Deed;
use crate::ast::DetPhrase;
use crate::ast::DidClause;
use crate::ast::KeywordLabel;
use crate::ast::Kind;
use crate::ast::LookbackSpan;
use crate::ast::NounPhrase;
use crate::ast::NounWord;
use crate::ast::Plurality;
use crate::ast::Predicate;
use crate::ast::Reach;
use crate::ast::Subject;
use crate::ast::Subtype;
use crate::ast::Zone;
impl NounWord {
    #[must_use]
    pub fn kind(&self) -> Kind {
        match self {
            Self::OfType { word, .. } | Self::Copied { word } => word.kind(),
            Self::Player => Kind::Player,
            Self::Join => Kind::Join {
                left: Box::new(Kind::Object),
                right: Box::new(Kind::Player),
            },
            Self::Pile => Kind::Pile,
            _ => Kind::Object,
        }
    }
    #[must_use]
    pub fn is_head(&self) -> bool {
        matches!(
            self,
            Self::Type { .. }
                | Self::Subtype { .. }
                | Self::Card
                | Self::Spell
                | Self::Permanent
                | Self::Token
                | Self::Emblem
                | Self::Ability
        )
    }
}
impl Reach {
    #[must_use]
    pub fn kind(&self) -> Kind {
        match self {
            Self::Word { word } | Self::UnionHalf { word } => word.kind(),
            _ => Kind::Object,
        }
    }
    #[must_use]
    pub fn is_bare_pronoun(&self) -> bool {
        matches!(self, Self::Bare | Self::AtSlot { .. })
    }
}
impl Subject {
    #[must_use]
    pub fn patient_role(&self) -> GramRole {
        match self {
            Self::Actor { .. } => GramRole::Obj,
            Self::Any | Self::By { .. } => GramRole::Subj,
        }
    }
}
impl Action {
    #[must_use]
    pub fn named_deed(&self) -> Option<Deed> {
        let mut action = self;
        for _ in 0..64 {
            match action {
                Self::InCaller { body } | Self::OwnScope { deed: None, body } => action = body,
                Self::OwnScope {
                    deed: Some(deed), ..
                } => return Some(deed.clone()),
                Self::Draw { .. } => {
                    return Some(Deed::Core {
                        deed: CoreDeed::Draw,
                    });
                }
                Self::Trigger => {
                    return Some(Deed::Core {
                        deed: CoreDeed::Trigger,
                    });
                }
                Self::Crew { .. } => {
                    return Some(Deed::OfAbility {
                        keyword: KeywordLabel::Named {
                            name: "crew".into(),
                        },
                    });
                }
                Self::Saddle { .. } => {
                    return Some(Deed::OfAbility {
                        keyword: KeywordLabel::Named {
                            name: "saddle".into(),
                        },
                    });
                }
                Self::FullyUnlock { .. } => {
                    return Some(Deed::Core {
                        deed: CoreDeed::FullyUnlock,
                    });
                }
                Self::Target { .. } => {
                    return Some(Deed::Core {
                        deed: CoreDeed::Target,
                    });
                }
                Self::ChangeControl { .. } => {
                    return Some(Deed::Core {
                        deed: CoreDeed::GainControl,
                    });
                }
                _ => return None,
            }
        }
        None
    }
    #[must_use]
    pub fn written_zone_change(&self) -> Option<(Option<Zone>, Zone)> {
        let mut action = self;
        for _ in 0..64 {
            match action {
                Self::InCaller { body } => action = body,
                Self::Move {
                    from_,
                    to: Some(to),
                    ..
                } => return to.sort().map(|to| (from_.sort(), to)),
                _ => return None,
            }
        }
        None
    }
    #[must_use]
    pub fn participial_anchor(&self) -> Option<ParticipialAnchor> {
        self.named_deed()
            .map(|deed| ParticipialAnchor::Deed { deed })
            .or_else(|| {
                self.written_zone_change().map(|(origin, destination)| {
                    ParticipialAnchor::ZoneChange {
                        origin,
                        destination,
                    }
                })
            })
    }
}
impl NounPhrase {
    #[must_use]
    pub fn is_which(&self) -> bool {
        let mut phrase = self;
        for _ in 0..64 {
            match phrase {
                Self::InCaller { body }
                | Self::FirstMention { phrase: body, .. }
                | Self::LaterMention { phrase: body, .. }
                | Self::Headed { subject: body, .. } => phrase = body,
                Self::Which => return true,
                _ => return false,
            }
        }
        false
    }
}
impl DidClause {
    #[must_use]
    pub fn anchor(&self) -> Option<ParticipialAnchor> {
        let subject = match self.subj.as_ref() {
            Subject::Actor { np } | Subject::By { np } => np.is_which(),
            Subject::Any => false,
        };
        if !subject && matches!(self.within.as_deref(), None | Some(LookbackSpan::ThisWay)) {
            self.action.participial_anchor()
        } else {
            None
        }
    }
    #[must_use]
    pub fn participial_agent_ok(&self, anchor: &ParticipialAnchor) -> bool {
        match self.subj.as_ref() {
            Subject::Any => true,
            Subject::Actor { np } | Subject::By { np } => {
                np.is_you()
                    || match (anchor, np.as_ref()) {
                        (_, NounPhrase::Actor | NounPhrase::This)
                        | (
                            ParticipialAnchor::Deed {
                                deed: Deed::OfAbility { .. },
                            },
                            NounPhrase::Pro { .. },
                        ) => true,
                        (_, NounPhrase::Headed { subject, .. }) => **subject == NounPhrase::This,
                        _ => false,
                    }
            }
        }
    }
}
impl Predicate {
    #[must_use]
    pub fn participial_anchor(&self) -> Option<(&DidClause, ParticipialAnchor)> {
        self.conjuncts().into_iter().find_map(|p| match p {
            Self::Did { clause } => clause.anchor().map(|anchor| (clause.as_ref(), anchor)),
            _ => None,
        })
    }
    #[must_use]
    pub fn participial_head(conjuncts: &[&Self], group: bool) -> (Option<NounWord>, Vec<Zone>) {
        let card = conjuncts.contains(&&Self::IsCard);
        let mut zones: Vec<_> = conjuncts
            .iter()
            .filter_map(|p| match p {
                Self::In { loc } => loc.sort(),
                _ => None,
            })
            .collect();
        let mut word = if card {
            Some(NounWord::Card)
        } else if zones.contains(&Zone::Battlefield) {
            Some(NounWord::Permanent)
        } else {
            None
        };
        if !group {
            for p in conjuncts {
                if let Self::HasType { type_ } = p {
                    word = Some(match word {
                        None => NounWord::Type {
                            type_: type_.clone(),
                        },
                        Some(word) => NounWord::OfType {
                            word: Box::new(word),
                            type_: type_.clone(),
                        },
                    });
                }
            }
        }
        if !card {
            zones.retain(|zone| *zone != Zone::Battlefield);
        }
        (word, zones)
    }
    #[must_use]
    pub fn with_anchor_span(&self) -> Self {
        self.with_anchor_span_at(64)
    }
    fn with_anchor_span_at(&self, fuel: u64) -> Self {
        match (fuel, self) {
            (1.., Self::InCaller { body }) => Self::InCaller {
                body: Box::new(body.with_anchor_span_at(fuel - 1)),
            },
            (
                1..,
                Self::Coord {
                    coordinator: Coordinator::And,
                    items,
                },
            ) => Self::Coord {
                coordinator: Coordinator::And,
                items: items
                    .iter()
                    .map(|p| p.with_anchor_span_at(fuel - 1))
                    .collect(),
            },
            (1.., Self::Did { clause }) if clause.within.is_none() && clause.anchor().is_some() => {
                let mut clause = clause.clone();
                clause.within = Some(Box::new(LookbackSpan::ThisWay));
                Self::Did { clause }
            }
            _ => self.clone(),
        }
    }
}
impl DetPhrase {
    #[must_use]
    pub fn participial_number(&self) -> Option<Option<&Plurality>> {
        match self {
            Self::The { plur } => Some(Some(plur)),
            Self::All | Self::Each => Some(None),
            _ => None,
        }
    }
}

impl NounWord {
    #[must_use]
    pub fn attach_host_zone(&self) -> Option<Zone> {
        match self {
            Self::OfType { word, .. } | Self::Copied { word } => word.attach_host_zone(),
            Self::Type { .. }
            | Self::Subtype {
                subtype: Subtype::Of { .. },
            }
            | Self::Permanent
            | Self::Token => Some(Zone::Battlefield),
            Self::Spell | Self::Copy => Some(Zone::Stack),
            _ => None,
        }
    }
    #[must_use]
    pub fn attach_host_types(&self) -> Vec<CardType> {
        match self {
            Self::OfType { word, type_ } => {
                super::merge_types(&word.attach_host_types(), std::slice::from_ref(type_))
            }
            Self::Copied { word } => word.attach_host_types(),
            Self::Type { type_ }
            | Self::Subtype {
                subtype: Subtype::Of { host: type_, .. },
            } => vec![type_.clone()],
            _ => Vec::new(),
        }
    }
    #[must_use]
    pub fn read_payload(&self, payload: &super::Payload) -> super::Payload {
        use super::Origin;
        use super::Payload;
        let Payload::Object {
            ty,
            zone,
            prov,
            orig,
            size,
        } = payload
        else {
            return payload.clone();
        };
        let object = |ty, zone, orig| Payload::Object {
            ty,
            zone,
            prov: prov.clone(),
            orig,
            size: *size,
        };
        match self {
            Self::Type { type_ }
            | Self::Subtype {
                subtype: Subtype::Of { host: type_, .. },
            } => object(
                super::merge_types(ty, std::slice::from_ref(type_)),
                zone.clone()
                    .or_else(|| type_.permanent.then_some(Zone::Battlefield)),
                orig.clone(),
            ),
            Self::Ability => Payload::Ability {
                orig: orig.clone(),
                zone: Some(Zone::Stack),
            },
            Self::Spell
            | Self::Subtype {
                subtype: Subtype::Spell { .. },
            } => object(ty.clone(), Some(Zone::Stack), orig.clone()),
            Self::Token => object(ty.clone(), Some(Zone::Battlefield), Some(Origin::Token)),
            Self::Permanent => object(ty.clone(), Some(Zone::Battlefield), orig.clone()),
            Self::Emblem => object(ty.clone(), Some(Zone::Command), orig.clone()),
            _ => payload.clone(),
        }
    }
}
