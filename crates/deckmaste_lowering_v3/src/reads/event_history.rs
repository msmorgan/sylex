//! Event identities and historical participants for grounded lookbacks.
use deckmaste_semantics_v3::ast::Action;
use deckmaste_semantics_v3::ast::Deed;
use deckmaste_semantics_v3::ast::DeedPermanent;
use deckmaste_semantics_v3::ast::DividedVerb;
use deckmaste_semantics_v3::ast::Exchanged;
use deckmaste_semantics_v3::ast::GameEvent;
use deckmaste_semantics_v3::ast::Kind;
use deckmaste_semantics_v3::ast::Location;
use deckmaste_semantics_v3::ast::LookbackSpan;
use deckmaste_semantics_v3::ast::NounPhrase;
use deckmaste_semantics_v3::ast::ObservationPoint;
use deckmaste_semantics_v3::ast::OutcomeSort;
use deckmaste_semantics_v3::ast::Subject;
use deckmaste_semantics_v3::ast::Zone;

use super::CausedEvent;
use super::DRS;
use super::EventVerb;
use super::StepPhase;
use super::ZoneChange;
use crate::reads::ActionAnchorsExt;
use crate::reads::ActionEventsExt;
use crate::reads::DeltaEventsExt;
use crate::reads::LocationPredicateFactsExt;
use crate::reads::NounPhraseFactsExt;
/// Discourse inspection of `Deed`.
pub trait DeedEventHistoryExt: Sized {
    #[must_use]
    fn permanent_actor(&self) -> bool;
    #[must_use]
    fn about_a_permanent(&self) -> bool;
}

impl DeedEventHistoryExt for Deed {
    fn permanent_actor(&self) -> bool {
        matches!(
            self,
            Self::Action {
                permanent: DeedPermanent::Performs,
                ..
            }
        )
    }
    fn about_a_permanent(&self) -> bool {
        matches!(
            self,
            Self::Action {
                permanent: DeedPermanent::IsAbout,
                ..
            }
        )
    }
}
/// Discourse inspection of `Action`.
pub trait ActionEventHistoryExt: Sized {
    #[must_use]
    fn canonical_patient(&self) -> Option<NounPhrase>;
    #[doc(hidden)]
    fn canonical_patient_at(&self, fuel: u64) -> Option<NounPhrase>;
    #[must_use]
    fn subject_is_patient(&self) -> bool;
    #[must_use]
    fn event_verbs(&self) -> Vec<EventVerb>;
    #[doc(hidden)]
    fn event_verbs_at(&self, fuel: u64) -> Vec<EventVerb>;
    #[must_use]
    fn caused_event(&self, performer: Option<Kind>, patient: Option<Kind>) -> CausedEvent;
    #[must_use]
    fn event_layers(&self) -> Vec<&Self>;
    #[doc(hidden)]
    fn event_layers_at(&self, fuel: u64) -> Vec<&Self>;
    #[must_use]
    fn described_events(&self, subject: Option<&Kind>) -> Vec<CausedEvent>;
    #[must_use]
    fn subject_reads_departed_permanent(&self, recorded: Option<&Zone>) -> bool;
}

impl ActionEventHistoryExt for Action {
    fn canonical_patient(&self) -> Option<NounPhrase> {
        self.canonical_patient_at(64)
    }
    fn canonical_patient_at(&self, fuel: u64) -> Option<NounPhrase> {
        match (fuel, self) {
            (1.., Self::InCaller { body }) => {
                body.canonical_patient_at(fuel - 1)
                    .map(|body| NounPhrase::InCaller {
                        body: Box::new(body),
                    })
            }
            (
                1..,
                Self::Move { subject, .. }
                | Self::SetStatus { subject, .. }
                | Self::TurnOver { subject, .. }
                | Self::ClearDamage { subject }
                | Self::Target { subject }
                | Self::Block { attacker: subject },
            ) => Some(*subject.clone()),
            _ => None,
        }
    }
    fn subject_is_patient(&self) -> bool {
        self.canonical_patient() == Some(NounPhrase::Actor)
            || self.own_deed().is_some_and(Deed::about_a_permanent)
    }
    fn event_verbs(&self) -> Vec<EventVerb> {
        self.event_verbs_at(64)
    }
    fn event_verbs_at(&self, fuel: u64) -> Vec<EventVerb> {
        if fuel == 0 {
            return Vec::new();
        }
        let verb = match self {
            Self::InCaller { body } | Self::OwnScope { body, .. } => {
                return body.event_verbs_at(fuel - 1);
            }
            Self::DealDamage { .. } => EventVerb::DamageDealt,
            Self::MarkDamage { .. } => EventVerb::DamageMarked,
            Self::PreventDamage { .. } => EventVerb::DamagePrevented,
            Self::RedirectDamage { .. } => EventVerb::DamageRedirected,
            Self::ClearDamage { .. } => EventVerb::DamageCleared,
            Self::SetStatus { status, .. } => EventVerb::Status {
                status: status.clone(),
            },
            Self::TurnOver { .. } => EventVerb::TurnedOver,
            Self::TurnFaceDown { .. } => EventVerb::TurnedFaceDown,
            Self::Combat { .. } => EventVerb::Combat,
            Self::Attack { .. } | Self::AttackWith { .. } => EventVerb::Attacked,
            Self::AttackUnblocked { .. } => EventVerb::AttackedUnblocked,
            Self::Block { .. } => EventVerb::Blocked,
            Self::TapForMana { .. } => EventVerb::TappedForMana,
            Self::CommitCrime => EventVerb::CrimeCommitted,
            Self::Attachment { move_, .. } => EventVerb::Attachment {
                move_: move_.clone(),
            },
            Self::GainDesignation { .. } | Self::GameGains { .. } => EventVerb::DesignationGained,
            Self::Unlock { .. } => EventVerb::Unlocked,
            Self::Conclude { .. } => EventVerb::Concluded,
            Self::Choose { .. } => EventVerb::Chose,
            Self::Vote { .. } => EventVerb::Voted,
            Self::Copy { .. } => EventVerb::Copied,
            Self::ChangeLife { delta } => {
                return delta
                    .life_outcomes()
                    .into_iter()
                    .map(|sort| EventVerb::Life { sort })
                    .collect();
            }
            Self::Exchange { exchanged } => {
                return if matches!(exchanged.as_ref(), Exchanged::LifeTotals { .. }) {
                    vec![
                        EventVerb::Exchanged,
                        EventVerb::Life {
                            sort: OutcomeSort::LifeGained,
                        },
                        EventVerb::Life {
                            sort: OutcomeSort::LifeLost,
                        },
                    ]
                } else {
                    vec![EventVerb::Exchanged]
                };
            }
            Self::AddMana { .. } => EventVerb::ManaAdded,
            Self::Expose { verb, .. } => EventVerb::Exposed { verb: verb.clone() },
            Self::Search { .. } => EventVerb::Searched,
            Self::Shuffle { .. } => EventVerb::Shuffled,
            Self::FlipCoins { .. } => EventVerb::CoinsFlipped,
            Self::RollDice { .. } => EventVerb::DiceRolled,
            Self::CreateObject { .. } => EventVerb::Created,
            Self::PutCounters { .. } => EventVerb::CountersPut,
            Self::Distribute { verb, .. } => match verb.as_ref() {
                DividedVerb::Counters { .. } => EventVerb::CountersPut,
                DividedVerb::Damage { .. } => EventVerb::DamageDealt,
            },
            Self::RemoveCounters { .. } => EventVerb::CountersRemoved,
            Self::MoveCounters { .. } => {
                return vec![EventVerb::CountersRemoved, EventVerb::CountersPut];
            }
            Self::Move { .. } => EventVerb::Moved,
            Self::Pay { .. } => EventVerb::Paid,
            Self::Procedure { kind, .. } => EventVerb::Procedure { kind: kind.clone() },
            Self::Relate { .. } => EventVerb::Related,
            Self::Merge { .. } => EventVerb::Merged,
            Self::SkipUntap { .. } => EventVerb::UntapSkipped,
            Self::SkipPart { .. } => EventVerb::PartSkipped,
            Self::AddTurnPart { .. } => EventVerb::TurnPartAdded,
            _ => return Vec::new(),
        };
        vec![verb]
    }
    fn caused_event(&self, performer: Option<Kind>, patient: Option<Kind>) -> CausedEvent {
        CausedEvent {
            deed: self.named_deed(),
            zone_change: self
                .written_zone_change()
                .map(|(origin, destination)| ZoneChange {
                    origin,
                    destination: Some(destination),
                }),
            verbs: self.event_verbs(),
            performer: performer.map(Box::new),
            patient: patient.map(Box::new),
            permitted: false,
        }
    }
    fn event_layers(&self) -> Vec<&Self> {
        self.event_layers_at(64)
    }
    fn event_layers_at(&self, fuel: u64) -> Vec<&Self> {
        match (fuel, self) {
            (0, _) => Vec::new(),
            (_, Self::InCaller { body }) => body.event_layers_at(fuel - 1),
            (_, Self::OwnScope { deed, body }) => {
                let mut layers = if deed.is_some() { vec![self] } else { Vec::new() };
                layers.extend(body.event_layers_at(fuel - 1));
                layers
            }
            _ => {
                if self.event_head_ok() {
                    vec![self]
                } else {
                    Vec::new()
                }
            }
        }
    }
    fn described_events(&self, subject: Option<&Kind>) -> Vec<CausedEvent> {
        self.event_layers()
            .into_iter()
            .map(|layer| {
                if layer.subject_is_patient() {
                    layer.caused_event(None, subject.cloned())
                } else {
                    layer.caused_event(
                        subject.cloned(),
                        layer
                            .scope_body()
                            .canonical_patient()
                            .and_then(|p| p.kind()),
                    )
                }
            })
            .collect()
    }
    fn subject_reads_departed_permanent(&self, recorded: Option<&Zone>) -> bool {
        matches!(self.scope_body(), Self::Move { subject, from_, at_, .. } if **subject == NounPhrase::Actor && from_.reads_departed_permanent(at_, recorded))
    }
}
/// Discourse inspection of `Location`.
pub trait LocationEventHistoryExt: Sized {
    #[must_use]
    fn writes_no_origin(&self) -> bool;
    #[must_use]
    fn positive_zones(&self) -> Option<Vec<&Self>>;
    #[doc(hidden)]
    fn positive_zones_at(&self, fuel: u64) -> Option<Vec<&Self>>;
    #[must_use]
    fn reads_departed_permanent(&self, at: &ObservationPoint, recorded: Option<&Zone>) -> bool;
}

impl LocationEventHistoryExt for Location {
    fn writes_no_origin(&self) -> bool {
        let mut loc = self;
        for _ in 0..64 {
            match loc {
                Self::InCaller { body } => loc = body,
                Self::AnyLocation => return true,
                _ => return false,
            }
        }
        false
    }
    fn positive_zones(&self) -> Option<Vec<&Self>> {
        self.positive_zones_at(64)
    }
    fn positive_zones_at(&self, fuel: u64) -> Option<Vec<&Self>> {
        match (fuel, self) {
            (0, _) | (_, Self::AnyLocation | Self::Not { .. }) => None,
            (_, Self::InCaller { body }) => body.positive_zones_at(fuel - 1),
            (
                _,
                Self::Zone { .. }
                | Self::FromEnd { .. }
                | Self::ExiledWith { .. }
                | Self::Pile { .. },
            ) => Some(vec![self]),
            (_, Self::Coord { locs, .. }) => locs.iter().try_fold(Vec::new(), |mut all, loc| {
                all.extend(loc.positive_zones_at(fuel - 1)?);
                Some(all)
            }),
        }
    }
    fn reads_departed_permanent(&self, at: &ObservationPoint, recorded: Option<&Zone>) -> bool {
        if self.writes_no_origin() {
            recorded == Some(&Zone::Battlefield)
        } else {
            *at == ObservationPoint::Departure
                && self.positive_zones().is_some_and(|zones| {
                    zones
                        .iter()
                        .any(|zone| zone.sort() == Some(Zone::Battlefield))
                })
        }
    }
}
impl DRS {
    #[must_use]
    pub fn caused_performer(&self, action: &Action) -> Kind {
        let actor = self.performers.first();
        if action.own_deed().is_some_and(Deed::about_a_permanent)
            || self
                .scope_stack
                .iter()
                .any(|frame| frame.deed.as_ref().is_some_and(Deed::about_a_permanent))
        {
            Kind::Player
        } else if action.own_deed().is_some_and(Deed::permanent_actor)
            && actor.is_none_or(|actor| actor.is_you)
        {
            Kind::Object
        } else {
            actor.map_or(Kind::Player, |actor| *actor.kind.clone())
        }
    }
    #[must_use]
    pub fn caused_patient(&self, action: &Action) -> Option<Kind> {
        action
            .scope_body()
            .canonical_patient()
            .map(|patient| patient.kind_in(self, Kind::Object))
            .or_else(|| {
                action
                    .own_deed()
                    .filter(|deed| deed.about_a_permanent())
                    .map(|_| {
                        self.performers
                            .first()
                            .filter(|actor| !actor.is_you)
                            .map_or(Kind::Object, |actor| *actor.kind.clone())
                    })
            })
    }
    pub fn record_caused(&mut self, before: &Self, phase: StepPhase, action: &Action) {
        if action.event_head_ok() && phase == StepPhase::PostEvent && !before.action_pattern {
            self.caused_events.insert(
                0,
                action.caused_event(
                    Some(before.caused_performer(action)),
                    before.caused_patient(action),
                ),
            );
        }
    }
    #[must_use]
    pub fn this_way_events(&self, action: &Action, subject: Option<&Kind>) -> Vec<&CausedEvent> {
        let anchor = action.participial_anchor();
        let verbs = action.event_verbs();
        self.caused_events
            .iter()
            .filter(|event| {
                event.read_by(anchor.as_ref(), &verbs)
                    && event.subject_fits(subject, action.subject_is_patient())
            })
            .collect()
    }
    #[must_use]
    pub fn this_way_recorded_origin(
        &self,
        subject: &Subject,
        action: &Action,
        within: Option<&LookbackSpan>,
    ) -> Option<Zone> {
        if !matches!(within, Some(LookbackSpan::ThisWay)) {
            return None;
        }
        if let Action::Move { from_, .. } = action.scope_body()
            && from_.writes_no_origin()
        {
            let subject = match subject {
                Subject::Actor { np } => np.kind(),
                Subject::By { np } => Some(np.kind_in(self, Kind::Object)),
                Subject::Any => None,
            };
            let origins: Vec<_> = self
                .this_way_events(action, subject.as_ref())
                .into_iter()
                .filter_map(|event| {
                    event
                        .zone_change
                        .as_ref()
                        .and_then(|change| change.origin.clone())
                })
                .collect();
            if origins.contains(&Zone::Battlefield) {
                Some(Zone::Battlefield)
            } else {
                origins.first().cloned()
            }
        } else {
            None
        }
    }
}
/// Discourse inspection of `GameEvent`.
pub trait GameEventHistoryExt: Sized {
    #[must_use]
    fn trigger_events(&self) -> Vec<CausedEvent>;
}

impl GameEventHistoryExt for GameEvent {
    fn trigger_events(&self) -> Vec<CausedEvent> {
        let mut event = self;
        for _ in 0..64 {
            match event {
                Self::InCaller { body }
                | Self::NthOccurrence { event: body, .. }
                | Self::Causes { event: body, .. } => event = body,
                Self::Does { subj, action } => {
                    let kind = match subj.as_ref() {
                        Subject::Actor { np } => np.kind(),
                        Subject::Any | Subject::By { .. } => None,
                    };
                    return action.described_events(kind.as_ref());
                }
                _ => return Vec::new(),
            }
        }
        Vec::new()
    }
}
