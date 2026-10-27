//! Mutually recursive checker operations, matching the Lean `StepOps`
//! interface.
use super::DRS;
use super::GramRole;
use super::ReferentId;
use super::StepPhase;
use crate::ast::Ability;
use crate::ast::Action;
use crate::ast::Amount;
use crate::ast::CardType;
use crate::ast::Condition;
use crate::ast::Cost;
use crate::ast::DeckTrait;
use crate::ast::GameEvent;
use crate::ast::Location;
use crate::ast::MoveRider;
use crate::ast::NounPhrase;
use crate::ast::Predicate;
use crate::ast::Quantity;
use crate::ast::Replacement;
use crate::ast::StaticSpec;
use crate::ast::Timing;
use crate::ast::TokenSpec;
use crate::ast::WouldClause;
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
