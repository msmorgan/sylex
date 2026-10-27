//! Discourse state mirrored from `lean/Semantics/Check/Words.lean`.
mod choices;
mod events;
mod local_shape;
mod selections;
mod words;
pub use selections::run_payloads;
pub use selections::selection_payload;
pub use selections::slice_payload;
pub use words::common_types;
pub use words::merge_types;
pub use words::normalize_types;
mod anchors;
mod circumstances;
mod described;
mod descriptions;
mod groups;
mod header_context;
mod mentions;
mod noun_facts;
mod nouns;
mod phrase_facts;
mod predicate_facts;
mod stepping;
pub use circumstances::step_duration_with;
pub use circumstances::step_location_with;
pub use circumstances::step_lookback_span_with;
pub use circumstances::step_quantity_with;
pub use circumstances::step_timing_with;
pub use circumstances::step_turn_ref_with;
pub use described::step_description_with;
pub use descriptions::participial_read;
pub use descriptions::whole_groups;
pub use events::EventFacts;
pub use mentions::choice_mode_checks;
pub use mentions::mention_this;
pub use mentions::mention_you;
pub use mentions::step_determiner_with;
pub use mentions::step_noun_alternatives_with;
pub use mentions::step_slice_count_with;
pub use nouns::step_noun_with;
pub use stepping::NounResult;
pub use stepping::StepOps;
pub use stepping::StepResult;

use crate::ast::AttachMove;
use crate::ast::BindingId;
use crate::ast::CardType;
use crate::ast::Deed;
use crate::ast::ExposeVerb;
use crate::ast::Kind;
use crate::ast::Letter;
use crate::ast::NounWord;
use crate::ast::OutcomeSort;
use crate::ast::PileFace;
use crate::ast::Plurality;
use crate::ast::Procedure;
use crate::ast::QualitySort;
use crate::ast::Reach;
use crate::ast::SlotCarrier;
use crate::ast::Status;
use crate::ast::Subtype;
use crate::ast::Zone;

pub type ReferentId = u64;

/// Mirrors `Semantics/Check/Words.lean:18`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Determiner {
    Target,
    A,
    Each,
    All,
    The,
    Part,
    Count,
    Self_,
}

/// Mirrors `Semantics/Check/Words.lean:131`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ZoneChange {
    pub origin: Option<Zone>,
    pub destination: Option<Zone>,
}

/// Mirrors `Semantics/Check/Words.lean:145`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Stamp {
    pub deeds: Vec<Deed>,
    pub was_field: bool,
    pub zone_change: Option<ZoneChange>,
    pub left_field: bool,
}

/// Mirrors `Semantics/Check/Words.lean:158`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParticipialAnchor {
    Deed {
        deed: Deed,
    },
    ZoneChange {
        origin: Option<Zone>,
        destination: Zone,
    },
}

/// Mirrors `Semantics/Check/Words.lean:169`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventVerb {
    DamageDealt,
    DamageMarked,
    DamageCleared,
    DamagePrevented,
    DamageRedirected,
    Status { status: Status },
    TurnedOver,
    TurnedFaceDown,
    Combat,
    Attacked,
    Blocked,
    AttackedUnblocked,
    TappedForMana,
    CrimeCommitted,
    Attachment { move_: AttachMove },
    DesignationGained,
    Unlocked,
    Concluded,
    Chose,
    Voted,
    Copied,
    Life { sort: OutcomeSort },

    Exchanged,
    ManaAdded,
    Exposed { verb: ExposeVerb },
    Searched,
    Shuffled,
    CoinsFlipped,
    DiceRolled,
    Created,
    CountersPut,
    CountersRemoved,
    Moved,
    Paid,
    Procedure { kind: Procedure },
    Related,
    Merged,
    UntapSkipped,
    PartSkipped,
    TurnPartAdded,
}

/// Mirrors `Semantics/Check/Words.lean:190`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CausedEvent {
    pub deed: Option<Deed>,
    pub zone_change: Option<ZoneChange>,
    pub verbs: Vec<EventVerb>,
    pub performer: Option<Box<Kind>>,
    pub patient: Option<Box<Kind>>,
    pub permitted: bool,
}

/// Mirrors `Semantics/Check/Words.lean:199`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Origin {
    Token,
    Copy,
}

/// Mirrors `Semantics/Check/Words.lean:223`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GramRole {
    Subj,
    Obj,
    Other,
}

/// Mirrors `Semantics/Check/Words.lean:241`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActorShape {
    pub kind: Box<Kind>,
    pub is_you: bool,
    pub ascribable: bool,
    pub ref_ids: Vec<ReferentId>,
    pub plur: Plurality,
    pub selves: Vec<ReferentId>,
}

/// Mirrors `Semantics/Check/Words.lean:253`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Payload {
    Object {
        ty: Vec<CardType>,
        zone: Option<Zone>,
        prov: Option<Stamp>,
        orig: Option<Origin>,
        size: Option<u64>,
    },
    Player {
        chosen: bool,
    },
    Quality {
        q: QualitySort,
    },
    Outcome {
        sort: OutcomeSort,
    },
    Difference,
    Letter {
        l: Letter,
    },
    TurnRef,
    Ability {
        orig: Option<Origin>,
        zone: Option<Zone>,
    },
    Pile {
        zone: Option<Zone>,
        size: Option<u64>,
        face: Option<PileFace>,
    },
    Join {
        l: Box<Payload>,
        r: Box<Payload>,
    },
}

/// Mirrors `Semantics/Check/Words.lean:347`.
#[derive(Clone, Debug, PartialEq, Eq)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "Preserves the independent discourse facts in the authoritative Lean structure"
)]
pub struct Referent {
    pub det: Determiner,
    pub plur: Plurality,
    pub payload: Box<Payload>,
    pub idents: Vec<BindingId>,
    pub id: ReferentId,
    pub clause: u64,
    pub role: GramRole,
    pub pronoun_resolved: bool,
    pub attachment_host: bool,
    pub announced_target_result: bool,
    pub chosen_object: bool,
    pub created_result: bool,
    pub linked: bool,
    pub link_site: ReferentId,
    pub link_arms: Vec<(u64, u64)>,
    pub spell_target: bool,
    pub group_key: Option<ReferentId>,
    pub part_of: Option<ReferentId>,
    pub member_of: Option<ReferentId>,
    pub default_anchor: bool,
    pub from_condition: bool,
    pub existential_domain: bool,
    pub replacement_domain: bool,
    pub controlled_by_you: Option<bool>,
    pub opponent_controls: bool,
    pub moved_from: Vec<ReferentId>,
    pub introduced_as_source: bool,
}

/// Mirrors `Semantics/Check/Words.lean:440`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectionEvidence {
    pub kind: Box<Kind>,
    pub source_group: Option<ReferentId>,
    pub chosen: bool,
    pub scope: u64,
}

/// Mirrors `Semantics/Check/Words.lean:449`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrantorLinked {
    pub since: ReferentId,
    pub published: Vec<Referent>,
    pub before: Vec<Referent>,
}

/// Mirrors `Semantics/Check/Words.lean:462`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OwnScopeFrame {
    pub pre_entry_ids: Vec<ReferentId>,
    pub masked_ids: Vec<ReferentId>,
    pub caller_clause: u64,
    pub caller_selection_scope: u64,
    pub caller_pending: Vec<(ReferentId, GramRole)>,
    pub caller_referent_ids: Vec<ReferentId>,
    pub entry_pending: Vec<(ReferentId, GramRole)>,
    pub caller_source_types: Vec<CardType>,
    pub deed: Option<Deed>,
}

/// Mirrors `Semantics/Check/Words.lean:484`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContrastAnchor {
    Member { id: ReferentId },
    Mention { id: ReferentId },
    Participant { facts: Box<Payload> },
    Source,
    You,
}

/// Mirrors `Semantics/Check/Words.lean:500`.
#[derive(Clone, Debug, PartialEq, Eq)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "Preserves the independent discourse facts in the authoritative Lean structure"
)]
pub struct DRS {
    pub referents: Vec<Referent>,
    pub next_id: ReferentId,
    pub current_clause: u64,
    pub performers: Vec<ActorShape>,
    pub scope_stack: Vec<OwnScopeFrame>,
    pub outer_referent_ids: Vec<Vec<ReferentId>>,
    pub pending_roles: Vec<(ReferentId, GramRole)>,
    pub resolved_ids: Vec<ReferentId>,
    pub relative_kind: Option<Box<Kind>>,
    pub relative_binder: Option<ReferentId>,
    pub relative_uses: u64,
    pub prospective_token_types: Vec<Vec<CardType>>,
    pub action_pattern: bool,
    pub characteristic_read: bool,
    pub each_kind_quality: bool,
    pub keyword_linked: Option<Vec<Referent>>,
    pub grantor_linked: Option<GrantorLinked>,
    pub historical_referent_ids: Vec<ReferentId>,
    pub reads_last_known: bool,
    pub shuffled_last_known: Vec<Referent>,
    pub handed_origin: Option<Zone>,
    pub group_size_scope: Option<Vec<ReferentId>>,
    pub selection_evidence: Vec<SelectionEvidence>,
    pub selection_scope: u64,
    pub next_selection_scope: u64,
    pub unbound_identities: Vec<BindingId>,
    pub source_types: Vec<CardType>,
    pub sourceless: bool,
    pub relative_types: Vec<CardType>,
    pub relative_zone: Option<Zone>,
    pub relative_control: Option<bool>,
    pub coordinated_self: Option<ReferentId>,
    pub contrast_excluded: Vec<ReferentId>,
    pub clause_participants: Vec<Referent>,
    pub distributive_subject: Option<ReferentId>,
    pub contrast_anchors: Vec<ContrastAnchor>,
    pub participial_only: Vec<Referent>,
    pub delayed_triggers: Vec<Referent>,
    pub caused_events: Vec<CausedEvent>,
    pub trigger_events: Vec<CausedEvent>,
    pub participial_anchor_read: Option<ParticipialAnchor>,
}

impl Referent {
    #[must_use]
    pub fn new(det: Determiner, plur: Plurality, payload: Payload) -> Self {
        Self {
            det,
            plur,
            payload: Box::new(payload),
            idents: Vec::new(),
            id: 0,
            clause: 0,
            role: GramRole::Other,
            pronoun_resolved: false,
            attachment_host: false,
            announced_target_result: false,
            chosen_object: false,
            created_result: false,
            linked: false,
            link_site: 0,
            link_arms: Vec::new(),
            spell_target: false,
            group_key: None,
            part_of: None,
            member_of: None,
            default_anchor: false,
            from_condition: false,
            existential_domain: false,
            replacement_domain: false,
            controlled_by_you: None,
            opponent_controls: false,
            moved_from: Vec::new(),
            introduced_as_source: false,
        }
    }
}

impl Default for DRS {
    fn default() -> Self {
        Self {
            referents: Vec::new(),
            next_id: 0,
            current_clause: 0,
            performers: Vec::new(),
            scope_stack: Vec::new(),
            outer_referent_ids: Vec::new(),
            pending_roles: Vec::new(),
            resolved_ids: Vec::new(),
            relative_kind: None,
            relative_binder: None,
            relative_uses: 0,
            prospective_token_types: Vec::new(),
            action_pattern: false,
            characteristic_read: false,
            each_kind_quality: false,
            keyword_linked: None,
            grantor_linked: None,
            historical_referent_ids: Vec::new(),
            reads_last_known: false,
            shuffled_last_known: Vec::new(),
            handed_origin: None,
            group_size_scope: None,
            selection_evidence: Vec::new(),
            selection_scope: 0,
            next_selection_scope: 1,
            unbound_identities: Vec::new(),
            source_types: Vec::new(),
            sourceless: false,
            relative_types: Vec::new(),
            relative_zone: None,
            relative_control: None,
            coordinated_self: None,
            contrast_excluded: Vec::new(),
            clause_participants: Vec::new(),
            distributive_subject: None,
            contrast_anchors: Vec::new(),
            participial_only: Vec::new(),
            delayed_triggers: Vec::new(),
            caused_events: Vec::new(),
            trigger_events: Vec::new(),
            participial_anchor_read: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepPhase {
    PreEvent,
    PostEvent,
}

impl GramRole {
    #[must_use]
    pub fn rank(&self) -> u8 {
        match self {
            Self::Subj => 0,
            Self::Obj => 1,
            Self::Other => 2,
        }
    }
}

impl DRS {
    #[must_use]
    pub fn is_visible(&self, id: ReferentId) -> bool {
        self.scope_stack
            .iter()
            .all(|frame| !frame.masked_ids.contains(&id))
    }
    pub fn visible_referents(&self) -> impl Iterator<Item = &Referent> {
        self.referents
            .iter()
            .filter(|referent| self.is_visible(referent.id))
    }
    #[must_use]
    pub fn find_by_id(&self, id: ReferentId) -> Option<&Referent> {
        self.referents.iter().find(|referent| referent.id == id)
    }
    #[must_use]
    pub fn is_clause_mate(&self, id: ReferentId) -> bool {
        self.pending_roles.iter().any(|(pending, _)| *pending == id)
    }
    #[must_use]
    pub fn ranking_clause(&self, referent: &Referent) -> u64 {
        if self.is_clause_mate(referent.id) {
            self.current_clause
        } else {
            referent.clause
        }
    }
    #[must_use]
    pub fn ranking_role(&self, referent: &Referent) -> u8 {
        self.pending_roles
            .iter()
            .filter(|(id, _)| *id == referent.id)
            .map(|(_, role)| role.rank())
            .min()
            .unwrap_or_else(|| referent.role.rank())
    }
    pub fn record_mention(
        &mut self,
        id: ReferentId,
        role: GramRole,
        in_arg_slot: bool,
        bare: bool,
    ) {
        if let Some(referent) = self.referents.iter_mut().find(|r| r.id == id) {
            referent.pronoun_resolved |= bare;
            if !in_arg_slot && self.characteristic_read {
                referent.clause = self.current_clause;
                referent.role = GramRole::Other;
            }
        }
        if in_arg_slot {
            self.pending_roles.insert(0, (id, role));
        }
        if !self.resolved_ids.contains(&id) {
            self.resolved_ids.insert(0, id);
        }
    }
    pub fn next_clause(&mut self) {
        self.performers_in_view();
        for referent in &mut self.referents {
            if let Some((_, role)) = self.pending_roles.iter().find(|(id, _)| *id == referent.id) {
                referent.clause = self.current_clause;
                referent.role = *role;
            }
            referent.pronoun_resolved = false;
        }
        self.current_clause += 1;
        self.pending_roles.clear();
    }
    fn performers_in_view(&mut self) {
        for referent in &mut self.referents {
            if referent.det == Determiner::Self_
                && self
                    .performers
                    .iter()
                    .any(|p| p.selves.contains(&referent.id))
            {
                referent.det = Determiner::The;
            }
        }
    }
    pub fn prepend(
        &mut self,
        mut additions: Vec<Referent>,
        role: GramRole,
        in_arg_slot: bool,
    ) -> Vec<ReferentId> {
        let count = additions.len() as u64;
        for (index, referent) in additions.iter_mut().enumerate() {
            referent.id = self.next_id + count - 1 - index as u64;
            referent.clause = self.current_clause;
            if index == 0 && referent.role == GramRole::Other {
                referent.role = role;
            }
        }
        let ids: Vec<_> = additions.iter().map(|r| r.id).collect();
        if in_arg_slot && let Some(id) = ids.first() {
            self.pending_roles.insert(0, (*id, role));
        }
        additions.append(&mut self.referents);
        self.referents = additions;
        self.next_id += count;
        ids
    }
    pub fn enter_own_scope(&mut self, deed: Option<Deed>) {
        let actor_ids = self
            .performers
            .first()
            .map_or(&[][..], |p| p.ref_ids.as_slice());
        let frame = OwnScopeFrame {
            pre_entry_ids: self.referents.iter().map(|r| r.id).collect(),
            masked_ids: self
                .referents
                .iter()
                .filter(|r| !actor_ids.contains(&r.id))
                .map(|r| r.id)
                .collect(),
            caller_clause: self.current_clause,
            caller_selection_scope: self.selection_scope,
            caller_pending: self.pending_roles.clone(),
            caller_referent_ids: Vec::new(),
            entry_pending: self.pending_roles.clone(),
            caller_source_types: self.source_types.clone(),
            deed,
        };
        self.selection_scope = self.next_selection_scope;
        self.next_selection_scope += 1;
        self.performers_in_view();
        self.scope_stack.insert(0, frame);
        self.pending_roles.clear();
        self.current_clause += 1;
        self.source_types.clear();
    }
    pub fn enter_caller(&mut self) {
        if self.scope_stack.is_empty() {
            return;
        }
        let frame = self.scope_stack.remove(0);
        self.current_clause = frame.caller_clause;
        self.selection_scope = frame.caller_selection_scope;
        self.pending_roles = frame.caller_pending;
        self.resolved_ids.clear();
        self.source_types = frame.caller_source_types;
    }
    pub fn leave_caller(&mut self, before: &Self, unmask_ids: &[ReferentId]) {
        let Some(frame) = before.scope_stack.first() else {
            return;
        };
        let mut frame = frame.clone();
        let mut newly_bound = unmask_ids.to_vec();
        newly_bound.extend(&self.resolved_ids);
        newly_bound.extend(
            self.referents
                .iter()
                .filter(|r| {
                    r.id >= before.next_id
                        || before
                            .find_by_id(r.id)
                            .is_none_or(|old| r.idents != old.idents)
                })
                .map(|r| r.id),
        );
        dedup(&mut newly_bound);
        frame.masked_ids.retain(|id| !newly_bound.contains(id));
        let added = self
            .pending_roles
            .len()
            .saturating_sub(frame.caller_pending.len());
        let mut pending = self.pending_roles[..added].to_vec();
        pending.extend_from_slice(&before.pending_roles);
        frame.caller_pending.clone_from(&self.pending_roles);
        frame.caller_referent_ids.extend(newly_bound);
        dedup(&mut frame.caller_referent_ids);
        self.scope_stack.insert(0, frame);
        self.current_clause = before.current_clause;
        self.selection_scope = before.selection_scope;
        self.pending_roles = pending;
        self.resolved_ids.extend_from_slice(&before.resolved_ids);
        dedup(&mut self.resolved_ids);
        self.source_types.clone_from(&before.source_types);
    }
    pub fn in_caller<R>(&mut self, read: impl FnOnce(&mut Self) -> R) -> R {
        let before = self.clone();
        self.enter_caller();
        let result = read(self);
        self.leave_caller(&before, &[]);
        result
    }
    pub fn leave_own_scope(&mut self) {
        if self.scope_stack.is_empty() {
            return;
        }
        let frame = self.scope_stack.remove(0);
        self.referents.retain_mut(|r| {
            if frame.pre_entry_ids.contains(&r.id) || frame.caller_referent_ids.contains(&r.id) {
                return true;
            }
            if r.det == Determiner::Part || matches!(*r.payload, Payload::Player { .. }) {
                return false;
            }
            r.clause = frame.caller_clause;
            r.role = GramRole::Obj;
            r.pronoun_resolved = false;
            true
        });
        self.pending_roles = frame.entry_pending.clone();
        self.pending_roles.extend(
            frame
                .caller_referent_ids
                .iter()
                .filter(|id| {
                    self.find_by_id(**id).is_some()
                        && !frame.entry_pending.iter().any(|(entry, _)| entry == *id)
                })
                .map(|id| (*id, GramRole::Obj))
                .collect::<Vec<_>>(),
        );
        self.selection_evidence
            .retain(|e| e.scope != self.selection_scope);
        self.selection_scope = frame.caller_selection_scope;
        self.current_clause = frame.caller_clause;
        self.source_types = frame.caller_source_types;
    }
}
fn dedup<T: PartialEq>(values: &mut Vec<T>) {
    let mut index = 0;
    while index < values.len() {
        if values[..index].contains(&values[index]) {
            values.remove(index);
        } else {
            index += 1;
        }
    }
}

impl Kind {
    #[must_use]
    pub fn lte(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Join { left, right }, _) => left.lte(other) && right.lte(other),
            (_, Self::Join { left, right }) => self.lte(left) || self.lte(right),
            _ => self == other,
        }
    }
}
impl Payload {
    #[must_use]
    pub fn kind(&self) -> Kind {
        match self {
            Self::Object { .. } | Self::Ability { .. } => Kind::Object,
            Self::Player { .. } => Kind::Player,
            Self::Quality { q } => Kind::Quality { sort: q.clone() },
            Self::Outcome { .. } | Self::Difference => Kind::Outcome,
            Self::Letter { l } => Kind::Letter { letter: l.clone() },
            Self::TurnRef => Kind::TurnRef,
            Self::Pile { .. } => Kind::Pile,
            Self::Join { l, r } => Kind::Join {
                left: Box::new(l.kind()),
                right: Box::new(r.kind()),
            },
        }
    }
    #[must_use]
    pub fn zone(&self) -> Option<&Zone> {
        match self {
            Self::Object { zone, .. } | Self::Ability { zone, .. } | Self::Pile { zone, .. } => {
                zone.as_ref()
            }
            Self::Join { l, r } => l.zone().or_else(|| r.zone()),
            _ => None,
        }
    }
    fn has_type(&self, card_type: &CardType) -> bool {
        matches!(self, Self::Object {ty,..} if ty.contains(card_type))
    }
    fn is_copy(&self) -> bool {
        matches!(
            self,
            Self::Object {
                orig: Some(Origin::Copy),
                ..
            } | Self::Ability {
                orig: Some(Origin::Copy),
                ..
            }
        )
    }
    fn half_reaches(&self, word: &NounWord) -> bool {
        if let Self::Join { l, r } = self {
            return l.half_reaches(word) || r.half_reaches(word);
        }
        match word {
            NounWord::OfType { word, type_ } => self.half_reaches(word) && self.has_type(type_),
            NounWord::Copied { word } => self.half_reaches(word) && self.is_copy(),
            _ => match self {
                Self::Object { ty, zone, .. } => match word {
                    NounWord::Type { type_: t }
                    | NounWord::Subtype {
                        subtype: Subtype::Of { host: t, .. },
                    } => *zone == Some(Zone::Battlefield) && ty.contains(t),
                    NounWord::Permanent => *zone == Some(Zone::Battlefield),
                    _ => false,
                },
                Self::Player { .. } => *word == NounWord::Player,
                _ => false,
            },
        }
    }
}
fn card_zone(zone: Option<&Zone>) -> bool {
    matches!(
        zone,
        Some(Zone::Graveyard | Zone::Exile | Zone::Hand | Zone::Library | Zone::Command)
    )
}
impl Referent {
    fn word_reaches(&self, word: &NounWord) -> bool {
        match word {
            NounWord::OfType { word, type_ } => {
                self.word_reaches(word) && self.payload.has_type(type_)
            }
            NounWord::Copied { word } => self.word_reaches(word) && self.payload.is_copy(),
            NounWord::Source => self.introduced_as_source && self.payload.kind() == Kind::Object,
            _ => match self.payload.as_ref() {
                Payload::Object { ty, zone, orig, .. } => match word {
                    NounWord::Type { type_: t }
                    | NounWord::Subtype {
                        subtype: Subtype::Of { host: t, .. },
                    } => *zone == Some(Zone::Battlefield) && ty.contains(t),
                    NounWord::Card => card_zone(zone.as_ref()),
                    NounWord::Spell
                    | NounWord::Subtype {
                        subtype: Subtype::Spell { .. },
                    }
                    | NounWord::Stack => *zone == Some(Zone::Stack),
                    NounWord::Permanent => *zone == Some(Zone::Battlefield),
                    NounWord::Token => {
                        *zone == Some(Zone::Battlefield) && *orig == Some(Origin::Token)
                    }
                    NounWord::Emblem => *zone == Some(Zone::Command),
                    NounWord::Copy => *orig == Some(Origin::Copy),
                    _ => false,
                },
                Payload::Player { .. } => *word == NounWord::Player,
                Payload::Join { .. } => match word {
                    NounWord::Type { .. }
                    | NounWord::Subtype {
                        subtype: Subtype::Of { .. },
                    }
                    | NounWord::Player
                    | NounWord::Permanent => self.payload.half_reaches(word),
                    NounWord::Join => Kind::Player.lte(&self.payload.kind()),
                    NounWord::Stack => self.payload.zone() == Some(&Zone::Stack),
                    _ => false,
                },
                Payload::Ability { zone, .. } => {
                    *word == NounWord::Ability
                        || (*word == NounWord::Stack && *zone == Some(Zone::Stack))
                }
                Payload::Pile { .. } => *word == NounWord::Pile,
                _ => false,
            },
        }
    }
    fn reaches(&self, reach: &Reach, plurality: &Plurality) -> bool {
        if self.plur != *plurality {
            return false;
        }
        match reach {
            Reach::Bare => Kind::Object.lte(&self.payload.kind()),
            Reach::AtSlot { slot } => {
                Kind::Object.lte(&self.payload.kind())
                    && self.payload.zone().is_none_or(|zone| match slot {
                        SlotCarrier::Permanent => *zone == Zone::Battlefield,
                        SlotCarrier::Card => card_zone(Some(zone)),
                        SlotCarrier::Spell => *zone == Zone::Stack,
                        SlotCarrier::OffStack => *zone != Zone::Stack,
                    })
            }
            Reach::Word { word } => self.det != Determiner::Self_ && self.word_reaches(word),
            Reach::UnionHalf { word } => {
                matches!(*self.payload, Payload::Join { .. }) && self.payload.half_reaches(word)
            }
        }
    }
    fn reaches_characteristic(&self, reach: &Reach, plurality: &Plurality) -> bool {
        fn stamped(
            word: &NounWord,
            stamp: &Stamp,
            ty: &[CardType],
            zone: Option<&Zone>,
            origin: Option<&Origin>,
        ) -> bool {
            match word {
                NounWord::OfType { word, type_ } => {
                    stamped(word, stamp, ty, zone, origin) && ty.contains(type_)
                }
                NounWord::Copied { word } => {
                    stamped(word, stamp, ty, zone, origin) && origin == Some(&Origin::Copy)
                }
                NounWord::Type { type_: t }
                | NounWord::Subtype {
                    subtype: Subtype::Of { host: t, .. },
                } => {
                    (stamp.was_field || stamp.left_field || zone == Some(&Zone::Battlefield))
                        && ty.contains(t)
                }
                NounWord::Card => card_zone(zone),
                NounWord::Spell
                | NounWord::Subtype {
                    subtype: Subtype::Spell { .. },
                } => {
                    zone == Some(&Zone::Stack)
                        || stamp
                            .zone_change
                            .as_ref()
                            .is_some_and(|change| change.origin == Some(Zone::Stack))
                }
                NounWord::Permanent => {
                    stamp.was_field || stamp.left_field || zone == Some(&Zone::Battlefield)
                }
                _ => false,
            }
        }
        if self.reaches(reach, plurality) {
            return true;
        }
        match (reach, self.payload.as_ref()) {
            (
                Reach::Word { word },
                Payload::Object {
                    ty,
                    zone,
                    prov: Some(stamp),
                    orig,
                    ..
                },
            ) => {
                self.det != Determiner::Self_
                    && self.plur == *plurality
                    && stamp.zone_change.is_some()
                    && stamped(word, stamp, ty, zone.as_ref(), orig.as_ref())
            }
            _ => false,
        }
    }
}
impl DRS {
    #[must_use]
    pub fn resolve_candidates(
        &self,
        reach: &Reach,
        plurality: &Plurality,
        in_arg_slot: bool,
    ) -> Vec<&Referent> {
        let mut candidates: Vec<_> = self
            .visible_referents()
            .filter(|r| {
                !r.linked
                    && (if self.characteristic_read {
                        r.reaches_characteristic(reach, plurality)
                    } else {
                        r.reaches(reach, plurality)
                    } || (self.reads_last_known
                        && self.historical_referent_ids.contains(&r.id)
                        && r.reaches_characteristic(reach, plurality)))
            })
            .filter(|r| !in_arg_slot || !self.is_clause_mate(r.id))
            .collect();
        let bare = matches!(reach, Reach::Bare | Reach::AtSlot { .. });
        if !bare && candidates.iter().any(|r| !r.pronoun_resolved) {
            candidates.retain(|r| !r.pronoun_resolved);
        }
        if *plurality == Plurality::Many && candidates.iter().any(|r| !r.replacement_domain) {
            candidates.retain(|r| !r.replacement_domain);
        }
        if bare {
            if let Some(clause) = candidates.iter().map(|r| self.ranking_clause(r)).max() {
                candidates.retain(|r| self.ranking_clause(r) == clause);
            }
            if let Some(rank) = candidates.iter().map(|r| self.ranking_role(r)).min() {
                candidates.retain(|r| self.ranking_role(r) == rank);
            }
            let keys: Vec<_> = candidates.iter().filter_map(|r| r.group_key).collect();
            candidates.retain(|r| r.part_of.is_none_or(|key| !keys.contains(&key)));
            candidates
        } else {
            candidates
                .into_iter()
                .max_by_key(|r| r.id)
                .into_iter()
                .collect()
        }
    }
    #[must_use]
    pub fn resolve_pronoun(
        &self,
        reach: &Reach,
        plurality: &Plurality,
        in_arg_slot: bool,
    ) -> Vec<&Referent> {
        let candidates = self.resolve_candidates(reach, plurality, in_arg_slot);
        if candidates.is_empty() && self.characteristic_read {
            self.shuffled_last_known
                .iter()
                .filter(|r| r.reaches_characteristic(reach, plurality))
                .collect()
        } else {
            candidates
        }
    }
    #[must_use]
    pub fn resolve_reflexive(&self, plurality: &Plurality) -> Vec<&Referent> {
        self.visible_referents()
            .filter(|r| {
                r.plur == *plurality
                    && (self
                        .pending_roles
                        .iter()
                        .any(|(id, role)| *id == r.id && *role == GramRole::Subj)
                        || (r.clause == self.current_clause && r.role == GramRole::Subj))
            })
            .collect()
    }
    pub fn first_mention(&mut self, identity: BindingId, ids: &[ReferentId]) {
        for referent in &mut self.referents {
            if ids.contains(&referent.id) && !referent.idents.contains(&identity) {
                referent.idents.push(identity);
            }
        }
        self.unbound_identities.retain(|id| *id != identity);
        if ids.is_empty() {
            self.unbound_identities.insert(0, identity);
        }
    }
    /// Read all members bound to the parameter, preserving their identities.
    ///
    /// # Errors
    /// Refuses an identity that has no visible binding or earlier refused first
    /// mention.
    pub fn later_mention(
        &mut self,
        identity: BindingId,
        role: GramRole,
        in_arg_slot: bool,
    ) -> Result<Vec<ReferentId>, crate::refusal::Refusal> {
        let ids: Vec<_> = self
            .visible_referents()
            .filter(|r| r.idents.contains(&identity))
            .map(|r| r.id)
            .collect();
        if ids.is_empty() {
            if let Some(delayed) = self
                .delayed_triggers
                .iter()
                .find(|r| r.idents.contains(&identity))
            {
                return Ok(vec![delayed.id]);
            }
            if self.unbound_identities.contains(&identity) {
                return Ok(Vec::new());
            }
            return Err(crate::refusal::Refusal::BindingIdentity { id: identity });
        }
        if in_arg_slot {
            for id in &ids {
                self.record_mention(*id, role, true, false);
            }
        }
        Ok(ids)
    }
    #[must_use]
    pub fn leave_relative(before: &Self, after: &Self) -> Self {
        let mut closed = before.clone();
        let mut hosts: Vec<_> = after
            .referents
            .iter()
            .filter(|r| r.attachment_host && r.id >= before.next_id)
            .cloned()
            .map(|mut r| {
                r.clause = before.current_clause;
                r.role = GramRole::Other;
                r.pronoun_resolved = false;
                r
            })
            .collect();
        hosts.append(&mut closed.referents);
        closed.referents = hosts;
        closed.next_id = before.next_id.max(after.next_id);
        closed.contrast_anchors.clone_from(&after.contrast_anchors);
        closed
    }
    #[must_use]
    pub fn leave_measure(before: &Self, after: &Self) -> Self {
        let mut closed = Self::leave_relative(before, after);
        let mut targets: Vec<_> = after
            .referents
            .iter()
            .filter(|r| r.det == Determiner::Target && r.id >= before.next_id)
            .cloned()
            .collect();
        targets.append(&mut closed.referents);
        closed.referents = targets;
        closed
    }
    pub fn close_modal_box(&mut self, before: &Self) {
        let entity = Kind::Join {
            left: Box::new(Kind::Object),
            right: Box::new(Kind::Player),
        };
        self.referents.retain(|r| {
            r.id < before.next_id
                || (r.payload.kind().lte(&entity)
                    && !matches!(r.det, Determiner::A | Determiner::Count | Determiner::Part))
        });
        let ids: Vec<_> = self.referents.iter().map(|r| r.id).collect();
        self.pending_roles.retain(|(id, _)| ids.contains(id));
        self.action_pattern = before.action_pattern;
        self.group_size_scope.clone_from(&before.group_size_scope);
    }
}

impl DRS {
    /// Apply the post-event zone transition, remapping every scope's
    /// references.
    pub fn apply_zone_move(
        &mut self,
        ids: &[ReferentId],
        deed: Option<&Deed>,
        destination: Option<&Zone>,
    ) {
        let mut ids = ids.to_vec();
        dedup(&mut ids);
        for old_id in ids {
            if self.historical_referent_ids.contains(&old_id) {
                continue;
            }
            let Some(old) = self.find_by_id(old_id) else {
                continue;
            };
            let payload = match old.payload.as_ref() {
                Payload::Object {
                    ty,
                    zone,
                    prov,
                    orig,
                    size,
                } => Payload::Object {
                    ty: ty.clone(),
                    zone: destination.cloned(),
                    orig: orig.clone(),
                    size: *size,
                    prov: Some(Stamp {
                        deeds: deed.cloned().into_iter().collect(),
                        was_field: *zone == Some(Zone::Battlefield),
                        zone_change: Some(ZoneChange {
                            origin: zone.clone(),
                            destination: destination.cloned(),
                        }),
                        left_field: prov.as_ref().is_some_and(|stamp| {
                            stamp.left_field || (stamp.zone_change.is_some() && stamp.was_field)
                        }),
                    }),
                },
                Payload::Ability { orig, .. } => Payload::Ability {
                    orig: orig.clone(),
                    zone: destination.cloned(),
                },
                _ => continue,
            };
            let fresh = self.next_id;
            let remap = |id: &mut ReferentId| {
                if *id == old_id {
                    *id = fresh;
                }
            };
            for referent in &mut self.referents {
                if referent.id == old_id {
                    referent.id = fresh;
                    referent.idents.clear();
                    *referent.payload = payload.clone();
                    referent.moved_from.insert(0, old_id);
                }
            }
            for frame in &mut self.scope_stack {
                let owned = frame.pre_entry_ids.contains(&old_id)
                    || frame.caller_referent_ids.contains(&old_id);
                frame.masked_ids.iter_mut().for_each(remap);
                frame
                    .caller_pending
                    .iter_mut()
                    .for_each(|(id, _)| remap(id));
                frame.entry_pending.iter_mut().for_each(|(id, _)| remap(id));
                frame.caller_referent_ids.iter_mut().for_each(remap);
                if owned {
                    frame.caller_referent_ids.push(fresh);
                }
                dedup(&mut frame.caller_referent_ids);
            }
            for evidence in &mut self.selection_evidence {
                if let Some(id) = &mut evidence.source_group {
                    remap(id);
                }
            }
            self.outer_referent_ids.iter_mut().flatten().for_each(remap);
            if let Some(ids) = &mut self.group_size_scope {
                ids.iter_mut().for_each(remap);
            }
            self.pending_roles.iter_mut().for_each(|(id, _)| remap(id));
            self.resolved_ids.iter_mut().for_each(remap);
            self.performers
                .iter_mut()
                .flat_map(|p| &mut p.ref_ids)
                .for_each(remap);
            if let Some(id) = &mut self.relative_binder {
                remap(id);
            }
            self.next_id = fresh + 1;
        }
    }
}
