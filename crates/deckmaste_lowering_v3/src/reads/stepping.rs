//! Mutually recursive checker operations, matching the Lean `StepOps`
//! interface.
use deckmaste_semantics_v3::ast::Ability;
use deckmaste_semantics_v3::ast::Action;
use deckmaste_semantics_v3::ast::Amount;
use deckmaste_semantics_v3::ast::CardType;
use deckmaste_semantics_v3::ast::Condition;
use deckmaste_semantics_v3::ast::Cost;
use deckmaste_semantics_v3::ast::DeckTrait;
use deckmaste_semantics_v3::ast::GameEvent;
use deckmaste_semantics_v3::ast::Location;
use deckmaste_semantics_v3::ast::MoveRider;
use deckmaste_semantics_v3::ast::NounPhrase;
use deckmaste_semantics_v3::ast::Predicate;
use deckmaste_semantics_v3::ast::Quantity;
use deckmaste_semantics_v3::ast::Replacement;
use deckmaste_semantics_v3::ast::StaticSpec;
use deckmaste_semantics_v3::ast::Timing;
use deckmaste_semantics_v3::ast::TokenSpec;
use deckmaste_semantics_v3::ast::WouldClause;

use super::DRS;
use super::GramRole;
use super::ReferentId;
use super::StepPhase;
use crate::refusal::Refusal;

pub type StepResult = (DRS, Vec<Refusal>);
pub type NounResult = (DRS, Vec<ReferentId>, Vec<Refusal>);

pub trait StepOps {
    fn fuel(&self) -> u64;
    fn quantity(&self, drs: DRS, value: &Quantity) -> StepResult;
    fn timing(&self, drs: DRS, value: &Timing) -> StepResult;
    fn location(&self, drs: DRS, value: &Location) -> StepResult;
    fn would(&self, drs: DRS, value: &WouldClause) -> StepResult;
    fn replacement(&self, drs: DRS, value: &Replacement, entering: bool) -> StepResult;
    fn rider(&self, drs: DRS, value: &MoveRider) -> StepResult;
    fn token_metadata(&self, drs: DRS, value: &TokenSpec) -> (DRS, Vec<CardType>, Vec<Refusal>);
    fn deck_trait(&self, value: &DeckTrait) -> Vec<Refusal>;
    fn predicate(&self, drs: DRS, value: &Predicate) -> StepResult;
    fn noun(&self, drs: DRS, role: GramRole, in_arg_slot: bool, value: &NounPhrase) -> NounResult;
    fn amount(&self, drs: DRS, value: &Amount) -> StepResult;
    fn condition(&self, drs: DRS, value: &Condition) -> StepResult;
    fn event(&self, drs: DRS, value: &GameEvent) -> StepResult;
    fn action(&self, phase: StepPhase, role: GramRole, drs: DRS, value: &Action) -> StepResult;
    fn specification(&self, drs: DRS, value: &StaticSpec) -> StepResult;
    fn cost(&self, drs: DRS, value: &Cost) -> StepResult;
    fn ability(&self, drs: DRS, value: &Ability) -> Vec<Refusal>;
}
