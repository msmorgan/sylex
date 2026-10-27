//! Event head and magnitude rules from `Semantics/Check/Events.lean`.
use crate::ast::Action;
use crate::ast::Deed;
use crate::ast::Delta;
use crate::ast::DidClause;
use crate::ast::GameEvent;
use crate::ast::NounPhrase;
use crate::ast::OutcomeSort;
use crate::ast::TallyMode;

impl<T> Delta<T> {
    pub fn life_outcomes(&self) -> Vec<OutcomeSort> {
        match self {
            Self::Up { .. } => vec![OutcomeSort::LifeGained],
            Self::Down { .. } => vec![OutcomeSort::LifeLost],
            Self::Set { .. } => vec![OutcomeSort::LifeGained, OutcomeSort::LifeLost],
        }
    }
}

impl Action {
    #[must_use]
    pub fn scope_body(&self) -> &Self {
        let mut body = self;
        for _ in 0..64 {
            match body {
                Self::InCaller { body: inner } | Self::OwnScope { body: inner, .. } => body = inner,
                _ => break,
            }
        }
        body
    }

    #[must_use]
    pub fn own_deed(&self) -> Option<&Deed> {
        let mut body = self;
        for _ in 0..64 {
            match body {
                Self::InCaller { body: inner }
                | Self::OwnScope {
                    deed: None,
                    body: inner,
                } => body = inner,
                Self::OwnScope {
                    deed: Some(deed), ..
                } => return Some(deed),
                _ => return None,
            }
        }
        None
    }

    pub fn head_gate(&self, admits: impl FnOnce(&Self) -> bool) -> bool {
        self.own_deed().is_some() || admits(self.scope_body())
    }

    #[must_use]
    pub fn event_head(&self) -> bool {
        match self {
            Self::DealDamage { .. }
            | Self::MarkDamage { .. }
            | Self::ClearDamage { .. }
            | Self::PreventDamage { .. }
            | Self::RedirectDamage { .. }
            | Self::SetStatus { .. }
            | Self::TurnOver { .. }
            | Self::TurnFaceDown { .. }
            | Self::Combat { .. }
            | Self::Attack { .. }
            | Self::Block { .. }
            | Self::AttackUnblocked { .. }
            | Self::AttackWith { .. }
            | Self::Target { .. }
            | Self::TapForMana { .. }
            | Self::CommitCrime
            | Self::Attachment { .. }
            | Self::ChangeControl { .. }
            | Self::Trigger
            | Self::Crew { .. }
            | Self::Saddle { .. }
            | Self::FullyUnlock { .. }
            | Self::GainDesignation { .. }
            | Self::GameGains { .. }
            | Self::Unlock { .. }
            | Self::Conclude { .. }
            | Self::Choose { .. }
            | Self::Vote { .. }
            | Self::Copy { .. }
            | Self::ChangeLife { .. }
            | Self::AddMana { .. }
            | Self::Draw { .. }
            | Self::Expose { .. }
            | Self::Search { .. }
            | Self::Shuffle { .. }
            | Self::FlipCoins { .. }
            | Self::RollDice { .. }
            | Self::CreateObject { .. }
            | Self::PutCounters { .. }
            | Self::Distribute { .. }
            | Self::RemoveCounters { .. }
            | Self::MoveCounters { .. }
            | Self::Move { .. }
            | Self::Pay { .. }
            | Self::Procedure { .. }
            | Self::Relate { .. }
            | Self::Merge { .. }
            | Self::SkipUntap { .. }
            | Self::SkipPart { .. }
            | Self::AddTurnPart { .. }
            | Self::Exchange { .. } => true,
            Self::InCaller { .. }
            | Self::OwnScope { .. }
            | Self::RestartGame
            | Self::SeparateIntoPiles { .. }
            | Self::RevealChoices { .. }
            | Self::ChooseNewTargets { .. }
            | Self::CopyTargets { .. }
            | Self::ApplyResultsTable { .. }
            | Self::IgnoreOutcomes { .. }
            | Self::ShiftResult { .. }
            | Self::StoreResults { .. }
            | Self::RerollStored { .. }
            | Self::Establish { .. }
            | Self::Instruct { .. }
            | Self::WithContinuation { .. }
            | Self::Limited { .. }
            | Self::DoIf { .. }
            | Self::DoForEach { .. }
            | Self::DoForEachKind { .. }
            | Self::Repeat { .. }
            | Self::Sequentially { .. }
            | Self::Simultaneously { .. }
            | Self::ChooseModes { .. }
            | Self::Delay { .. }
            | Self::Replace { .. }
            | Self::HoldUntil { .. }
            | Self::TriggerReflexively { .. } => false,
        }
    }

    pub fn event_head_ok(&self) -> bool {
        self.head_gate(Self::event_head)
    }

    #[must_use]
    pub fn event_has_magnitude(&self) -> bool {
        let mut body = self;
        for _ in 0..64 {
            match body {
                Self::InCaller { body: inner } | Self::OwnScope { body: inner, .. } => body = inner,
                Self::DealDamage { .. }
                | Self::MarkDamage { .. }
                | Self::ChangeLife {
                    delta: Delta::Up { .. } | Delta::Down { .. },
                }
                | Self::Pay { .. }
                | Self::Draw { .. }
                | Self::PutCounters { .. }
                | Self::RemoveCounters { .. }
                | Self::MoveCounters { .. }
                | Self::CreateObject { .. }
                | Self::AddMana { .. } => return true,
                _ => return false,
            }
        }
        false
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EventFacts {
    pub has_magnitude: bool,
}

impl GameEvent {
    #[must_use]
    pub fn facts(&self) -> EventFacts {
        let mut event = self;
        for _ in 0..64 {
            match event {
                Self::InCaller { body } => event = body,
                Self::NthOccurrence { event: inner, .. } | Self::Causes { event: inner, .. } => {
                    event = inner;
                }
                Self::Does { action, .. } => {
                    return EventFacts {
                        has_magnitude: action.event_has_magnitude(),
                    };
                }
                Self::BeginningOf { .. }
                | Self::EndOf { .. }
                | Self::Now
                | Self::StateHolds { .. }
                | Self::ChapterMark { .. }
                | Self::StateBasedActionsChecked => return EventFacts::default(),
            }
        }
        EventFacts::default()
    }
    #[must_use]
    pub fn scope_body(&self) -> &Self {
        let mut event = self;
        for _ in 0..64 {
            match event {
                Self::InCaller { body } => event = body,
                _ => break,
            }
        }
        event
    }
}
impl DidClause {
    #[must_use]
    pub fn has_magnitude(&self) -> bool {
        self.action.event_has_magnitude()
    }
}
impl TallyMode {
    #[must_use]
    pub fn permits(&self, clause: &DidClause) -> bool {
        match self {
            Self::Events => true,
            Self::Quantity => clause.has_magnitude(),
        }
    }
}
impl NounPhrase {
    #[must_use]
    pub fn is_you(&self) -> bool {
        let mut phrase = self;
        for _ in 0..64 {
            match phrase {
                Self::InCaller { body }
                | Self::FirstMention { phrase: body, .. }
                | Self::LaterMention { phrase: body, .. } => phrase = body,
                Self::You => return true,
                _ => return false,
            }
        }
        false
    }
}

impl super::CausedEvent {
    #[must_use]
    pub fn read_by(
        &self,
        anchor: Option<&super::ParticipialAnchor>,
        verbs: &[super::EventVerb],
    ) -> bool {
        match anchor {
            Some(super::ParticipialAnchor::Deed { deed }) => {
                self.deed.as_ref() == Some(deed)
                    || (matches!(deed, crate::ast::Deed::Action { label, permanent: crate::ast::DeedPermanent::None } if label.as_str() == "cast")
                        && matches!(&self.deed, Some(crate::ast::Deed::Action { label, permanent: crate::ast::DeedPermanent::None }) if label.as_str() == "play"))
            }
            Some(super::ParticipialAnchor::ZoneChange {
                origin,
                destination,
            }) => self.zone_change.as_ref().is_some_and(|change| {
                change.destination.as_ref() == Some(destination)
                    && origin.as_ref().is_none_or(|origin| {
                        change.origin.as_ref().is_none_or(|zone| zone == origin)
                    })
            }),
            None => self.verbs.iter().any(|verb| verbs.contains(verb)),
        }
    }
    #[must_use]
    pub fn subject_fits(
        &self,
        subject: Option<&crate::ast::Kind>,
        subject_is_patient: bool,
    ) -> bool {
        let participant = if subject_is_patient { &self.patient } else { &self.performer };
        participant
            .as_deref()
            .zip(subject)
            .is_none_or(|(participant, subject)| {
                participant.lte(subject) || subject.lte(participant)
            })
    }
}
