// These tests isolate the noun handler. Any callback into an unrelated AST
// layer is a test failure, rather than an accepting fallback.
use deckmaste_semantics_v3::ast::*;
use deckmaste_semantics_v3::reads::*;
use deckmaste_semantics_v3::refusal::Refusal;
struct Nouns;
impl StepOps for Nouns {
    fn fuel(&self) -> u64 {
        64
    }
    fn quantity(&self, _drs: DRS, _value: &Quantity) -> StepResult {
        panic!("unexpected callback: quantity")
    }
    fn timing(&self, _drs: DRS, _value: &Timing) -> StepResult {
        panic!("unexpected callback: timing")
    }
    fn location(&self, _drs: DRS, _value: &Location) -> StepResult {
        panic!("unexpected callback: location")
    }
    fn would(&self, _drs: DRS, _value: &WouldClause) -> StepResult {
        panic!("unexpected callback: would")
    }
    fn replacement(&self, _drs: DRS, _value: &Replacement, _entering: bool) -> StepResult {
        panic!("unexpected callback: replacement")
    }
    fn rider(&self, _drs: DRS, _value: &MoveRider) -> StepResult {
        panic!("unexpected callback: rider")
    }
    fn token_metadata(&self, _drs: DRS, _value: &TokenSpec) -> (DRS, Vec<CardType>, Vec<Refusal>) {
        panic!("unexpected callback: token_metadata")
    }
    fn deck_trait(&self, _value: &DeckTrait) -> Vec<Refusal> {
        panic!("unexpected callback: deck_trait")
    }
    fn predicate(&self, _drs: DRS, _value: &Predicate) -> StepResult {
        panic!("unexpected callback: predicate")
    }
    fn noun(&self, drs: DRS, role: GramRole, in_arg_slot: bool, value: &NounPhrase) -> NounResult {
        step_noun_with(self, drs, role, in_arg_slot, value)
    }
    fn amount(&self, _drs: DRS, _value: &Amount) -> StepResult {
        panic!("unexpected callback: amount")
    }
    fn condition(&self, _drs: DRS, _value: &Condition) -> StepResult {
        panic!("unexpected callback: condition")
    }
    fn event(&self, _drs: DRS, _value: &GameEvent) -> StepResult {
        panic!("unexpected callback: event")
    }
    fn action(&self, _phase: StepPhase, _role: GramRole, _drs: DRS, _value: &Action) -> StepResult {
        panic!("unexpected callback: action")
    }
    fn specification(&self, _drs: DRS, _value: &StaticSpec) -> StepResult {
        panic!("unexpected callback: specification")
    }
    fn cost(&self, _drs: DRS, _value: &Cost) -> StepResult {
        panic!("unexpected callback: cost")
    }
    fn ability(&self, _drs: DRS, _value: &Ability) -> Vec<Refusal> {
        panic!("unexpected callback: ability")
    }
}
fn object() -> Referent {
    Referent::new(
        Determiner::A,
        Plurality::One,
        Payload::Object {
            ty: vec![CardType::creature()],
            zone: Some(Zone::Battlefield),
            prov: None,
            orig: None,
            size: None,
        },
    )
}
#[test]
fn noun_stepping_applies_principle_a_and_argument_coreference_gate() {
    let mut drs = DRS::default();
    let id = drs.prepend(vec![object()], GramRole::Subj, true)[0];
    let (out, ids, errors) = Nouns.noun(
        drs.clone(),
        GramRole::Obj,
        true,
        &NounPhrase::Refl {
            plurality: Plurality::One,
        },
    );
    assert_eq!(ids, vec![id]);
    assert_eq!(errors, Vec::<Refusal>::new());
    assert!(out.pending_roles.contains(&(id, GramRole::Obj)));
    let (_, ids, errors) = Nouns.noun(
        drs,
        GramRole::Obj,
        true,
        &NounPhrase::Pro {
            reach: Reach::Bare,
            plurality: Plurality::One,
        },
    );
    assert_eq!(ids, Vec::<ReferentId>::new());
    assert_eq!(
        errors,
        vec![Refusal::Anaphor {
            r: Reach::Bare,
            pl: Plurality::One,
            found: 0
        }]
    );
}
#[test]
fn noun_stepping_refuses_unbound_which_and_counts_visible_relative_uses() {
    let (_, ids, errors) = Nouns.noun(DRS::default(), GramRole::Obj, true, &NounPhrase::Which);
    assert_eq!(ids, Vec::<ReferentId>::new());
    assert_eq!(errors, vec![Refusal::WhichUnbound]);
    let mut drs = DRS::default();
    let id = drs.prepend(vec![object()], GramRole::Other, false)[0];
    drs.relative_binder = Some(id);
    let (out, ids, errors) = Nouns.noun(drs, GramRole::Subj, true, &NounPhrase::Which);
    assert_eq!(ids, vec![id]);
    assert_eq!(out.relative_uses, 1);
    assert_eq!(errors, Vec::<Refusal>::new());
}
#[test]
fn caller_noun_reads_unmask_actual_identities_without_creating_duplicates() {
    let mut drs = DRS::default();
    let id = drs.prepend(vec![object()], GramRole::Other, false)[0];
    drs.enter_own_scope(None);
    let noun = NounPhrase::InCaller {
        body: Box::new(NounPhrase::FirstMention {
            id: "subject".into(),
            phrase: Box::new(NounPhrase::Pro {
                reach: Reach::Bare,
                plurality: Plurality::One,
            }),
        }),
    };
    let (out, ids, errors) = Nouns.noun(drs, GramRole::Obj, true, &noun);
    assert_eq!(ids, vec![id]);
    assert_eq!(errors, Vec::<Refusal>::new());
    assert!(out.is_visible(id));
    assert_eq!(out.referents.len(), 1);
    let (out, later_ids, errors) = Nouns.noun(
        out,
        GramRole::Obj,
        true,
        &NounPhrase::LaterMention {
            id: "subject".into(),
            phrase: Box::new(NounPhrase::Any),
        },
    );
    assert_eq!(later_ids, ids);
    assert_eq!(out.referents.len(), 1);
    assert_eq!(errors, Vec::<Refusal>::new());
}
#[test]
fn source_noun_is_reused_and_sourceless_context_is_refused() {
    let (out, first, errors) = Nouns.noun(DRS::default(), GramRole::Subj, true, &NounPhrase::This);
    assert_eq!(errors, Vec::<Refusal>::new());
    let (mut out, second, errors) = Nouns.noun(out, GramRole::Other, false, &NounPhrase::This);
    assert_eq!(first, second);
    assert_eq!(out.referents.len(), 1);
    assert_eq!(errors, Vec::<Refusal>::new());
    out.sourceless = true;
    let (_, ids, errors) = Nouns.noun(out, GramRole::Subj, true, &NounPhrase::This);
    assert_eq!(ids, first);
    assert_eq!(errors, vec![Refusal::NoSource]);
}
#[test]
fn plural_identity_reads_every_coordinated_member_and_group() {
    let noun = NounPhrase::FirstMention {
        id: "participants".into(),
        phrase: Box::new(NounPhrase::Coord {
            coordinator: Coordinator::And,
            phrases: vec![NounPhrase::You, NounPhrase::This],
        }),
    };
    let (out, ids, errors) = Nouns.noun(DRS::default(), GramRole::Other, false, &noun);
    assert_eq!(errors, Vec::<Refusal>::new());
    assert_eq!(ids.len(), 3);
    let (out, later, errors) = Nouns.noun(
        out,
        GramRole::Obj,
        true,
        &NounPhrase::LaterMention {
            id: "participants".into(),
            phrase: Box::new(NounPhrase::Any),
        },
    );
    assert_eq!(later, ids.iter().rev().copied().collect::<Vec<_>>());
    assert_eq!(errors, Vec::<Refusal>::new());
    assert_eq!(out.referents.len(), 3);
}
