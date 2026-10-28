//! Definite, participial, and contrast-anchor resolution from the Lean checker.
use deckmaste_semantics_v3::ast::CardClass;
use deckmaste_semantics_v3::ast::CardType;
use deckmaste_semantics_v3::ast::Kind;
use deckmaste_semantics_v3::ast::NounPhrase;
use deckmaste_semantics_v3::ast::NounWord;
use deckmaste_semantics_v3::ast::Plurality;
use deckmaste_semantics_v3::ast::PossessorAxis;
use deckmaste_semantics_v3::ast::Predicate;
use deckmaste_semantics_v3::ast::Subtype;
use deckmaste_semantics_v3::ast::Zone;

use super::ContrastAnchor;
use super::DRS;
use super::Determiner;
use super::Origin;
use super::ParticipialAnchor;
use super::Payload;
use super::Referent;
use super::ReferentId;
use super::Stamp;
use super::card_zone;
use crate::reads::KindResolutionExt;
use crate::reads::NounPhrasePredicateFactsExt;
use crate::reads::PredicateFactsExt;

fn compatible<T: PartialEq>(a: Option<&T>, b: Option<&T>) -> bool {
    a.zip(b).is_none_or(|(a, b)| a == b)
}
fn type_fits(head: &[CardType], anchor: &[CardType]) -> bool {
    head.is_empty() || anchor.is_empty() || head.iter().any(|ty| anchor.contains(ty))
}
fn overlaps(a: &Kind, b: &Kind) -> bool {
    a.lte(b) || b.lte(a)
}

impl Referent {
    fn anchors_contrast(
        &self,
        kind: &Kind,
        types: &[CardType],
        zone: Option<&Zone>,
        control: Option<bool>,
    ) -> bool {
        !self.linked
            && !self.default_anchor
            && !self.from_condition
            && overlaps(kind, &self.payload.kind())
            && type_fits(types, &self.payload.types())
            && compatible(zone, self.payload.zone())
            && compatible(control.as_ref(), self.controlled_by_you.as_ref())
    }
    #[must_use]
    pub fn admits_conjunct(&self, predicate: &Predicate) -> bool {
        match predicate {
            Predicate::HasType { type_ } => {
                Kind::Object.lte(&self.payload.kind()) && self.payload.types().contains(type_)
            }
            Predicate::HasPossessor {
                axis: PossessorAxis::Controller,
                possessor,
            } => {
                if **possessor == NounPhrase::You {
                    self.controlled_by_you == Some(true)
                } else {
                    possessor.names_opponent() && self.opponent_controls
                }
            }
            Predicate::Not { predicate } if matches!(predicate.as_ref(), Predicate::HasPossessor { axis: PossessorAxis::Controller, possessor } if **possessor == NounPhrase::You) => {
                self.controlled_by_you == Some(false)
            }
            _ => false,
        }
    }
    #[must_use]
    pub fn anchored_by(&self, anchor: &ParticipialAnchor) -> bool {
        let Some(stamp) = self.payload.provenance() else {
            return false;
        };
        match anchor {
            ParticipialAnchor::Deed { deed } => stamp.deeds.contains(deed),
            ParticipialAnchor::ZoneChange {
                origin,
                destination,
            } => stamp.zone_change.as_ref().is_some_and(|change| {
                change.destination.as_ref() == Some(destination)
                    && origin
                        .as_ref()
                        .is_none_or(|origin| change.origin.as_ref() == Some(origin))
            }),
        }
    }
}
impl DRS {
    #[must_use]
    pub fn contrast_anchor(
        &self,
        kind: &Kind,
        types: &[CardType],
        zone: Option<&Zone>,
        control: Option<bool>,
    ) -> Option<ContrastAnchor> {
        if let Some(id) = self.coordinated_self {
            return Some(ContrastAnchor::Mention { id });
        }
        if let Some(group) = self
            .distributive_subject
            .filter(|id| self.is_visible(*id) && !self.contrast_excluded.contains(id))
            .and_then(|id| self.find_by_id(id))
            && group.anchors_contrast(kind, types, zone, control)
        {
            return Some(ContrastAnchor::Member { id: group.id });
        }
        if let Some(nearest) = self
            .visible_referents()
            .filter(|r| {
                !self.contrast_excluded.contains(&r.id)
                    && r.anchors_contrast(kind, types, zone, control)
            })
            .max_by_key(|r| (r.clause, r.id))
        {
            return Some(ContrastAnchor::Mention { id: nearest.id });
        }
        if let Some(participant) = self
            .clause_participants
            .iter()
            .find(|r| r.anchors_contrast(kind, types, zone, control))
        {
            return Some(ContrastAnchor::Participant {
                facts: participant.payload.clone(),
            });
        }
        if *kind == Kind::Player {
            return Some(ContrastAnchor::You);
        }
        let source_zone = if !self.source_types.is_empty()
            && self
                .source_types
                .iter()
                .all(|t| t.card_class == Some(CardClass::InstantOrSorceryCard))
        {
            Some(Zone::Stack)
        } else {
            None
        };
        if overlaps(kind, &Kind::Object)
            && compatible(zone, source_zone.as_ref())
            && compatible(control.as_ref(), Some(&true))
        {
            Some(ContrastAnchor::Source)
        } else {
            None
        }
    }
    #[must_use]
    pub fn definite_antecedents(&self, predicate: &Predicate) -> Vec<&Referent> {
        let conjuncts = predicate.flattened();
        if conjuncts.is_empty() {
            return vec![];
        }
        self.visible_referents()
            .filter(|r| {
                !r.linked
                    && !r.default_anchor
                    && !r.from_condition
                    && r.moved_from.is_empty()
                    && r.plur == Plurality::One
                    && matches!(r.det, Determiner::Target | Determiner::A | Determiner::The)
                    && conjuncts.iter().all(|p| r.admits_conjunct(p))
            })
            .collect()
    }
    #[must_use]
    pub fn chosen_antecedents(
        &self,
        plurality: &Plurality,
        predicate: &Predicate,
    ) -> Vec<&Referent> {
        let conjuncts = predicate.flattened();
        self.visible_referents()
            .filter(|r| {
                r.chosen_object
                    && !r.linked
                    && r.moved_from.is_empty()
                    && r.plur == *plurality
                    && conjuncts.iter().all(|p| match p {
                        Predicate::Chosen { .. } => r.chosen_object,
                        Predicate::IsCard => Kind::Object.lte(&r.payload.kind()),
                        p => r.admits_conjunct(p),
                    })
            })
            .collect()
    }
    #[must_use]
    pub fn participial_candidates(
        &self,
        anchor: &ParticipialAnchor,
        plurality: Option<&Plurality>,
        word: Option<&NounWord>,
        this_way: bool,
        zones: &[Zone],
    ) -> Vec<&Referent> {
        let mut found = Vec::new();
        for r in self
            .participial_only
            .iter()
            .filter(|r| self.is_visible(r.id))
            .chain(self.visible_referents())
        {
            let number_ok = match plurality {
                None => true,
                Some(Plurality::One) => r.plur == Plurality::One || (!this_way && r.linked),
                Some(Plurality::Many) => r.plur == Plurality::Many,
            };
            let word_ok = match (word, r.payload.as_ref()) {
                (None, _) => true,
                (
                    Some(word),
                    Payload::Object {
                        ty,
                        zone,
                        prov: Some(stamp),
                        orig,
                        ..
                    },
                ) => stamped_word(word, stamp, ty, zone.as_ref(), orig.as_ref()),
                _ => false,
            };
            if Kind::Object.lte(&r.payload.kind())
                && r.anchored_by(anchor)
                && (!this_way || !r.linked)
                && number_ok
                && word_ok
                && zones.iter().all(|zone| r.payload.zone() == Some(zone))
                && !found.iter().any(|old: &&Referent| old.id == r.id)
            {
                found.push(r);
            }
        }
        found
    }
}
fn stamped_word(
    word: &NounWord,
    stamp: &Stamp,
    types: &[CardType],
    zone: Option<&Zone>,
    origin: Option<&Origin>,
) -> bool {
    match word {
        NounWord::OfType { word, type_ } => {
            stamped_word(word, stamp, types, zone, origin) && types.contains(type_)
        }
        NounWord::Copied { word } => {
            stamped_word(word, stamp, types, zone, origin) && origin == Some(&Origin::Copy)
        }
        NounWord::Type { type_ }
        | NounWord::Subtype {
            subtype: Subtype::Of { host: type_, .. },
        } => (stamp.was_field || zone == Some(&Zone::Battlefield)) && types.contains(type_),
        NounWord::Card => card_zone(zone),
        NounWord::Spell
        | NounWord::Subtype {
            subtype: Subtype::Spell { .. },
        } => zone == Some(&Zone::Stack),
        NounWord::Permanent => stamp.was_field || zone == Some(&Zone::Battlefield),
        _ => false,
    }
}
#[must_use]
pub fn whole_groups<'a>(refs: &[&'a Referent]) -> Vec<&'a Referent> {
    refs.iter()
        .copied()
        .filter(|r| {
            !r.part_of
                .is_some_and(|key| refs.iter().any(|r| r.group_key == Some(key)))
        })
        .collect()
}
/// Resolve a participial description, returning the candidate count on
/// ambiguity.
///
/// # Errors
/// Refuses a missing or ambiguous description, counting mutually exclusive
/// linked arms at one publication site as one place.
pub fn participial_read<'a>(
    plurality: Option<&Plurality>,
    found: &[&'a Referent],
) -> Result<Vec<&'a Referent>, usize> {
    if found.is_empty() {
        return Err(0);
    }
    let found = whole_groups(found);
    if plurality.is_none() {
        return Ok(found);
    }
    let mut sites: Vec<(bool, ReferentId)> = Vec::new();
    for r in &found {
        let site = (r.linked, if r.linked { r.link_site } else { r.id });
        if !sites.contains(&site) {
            sites.push(site);
        }
    }
    let alternatives = found.iter().all(|a| {
        found.iter().all(|b| {
            a.id == b.id
                || a.link_arms.iter().any(|(point, arm)| {
                    b.link_arms
                        .iter()
                        .any(|(other, other_arm)| point == other && arm != other_arm)
                })
        })
    });
    match sites.as_slice() {
        [(false, _)] => Ok(found),
        [(true, _)] if alternatives => Ok(found),
        [(true, _)] => Err(found.len()),
        _ => Err(sites.len()),
    }
}
