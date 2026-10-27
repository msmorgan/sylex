use deckmaste_lowering_v3::reads::*;
use deckmaste_lowering_v3::refusal::Refusal;
use deckmaste_semantics_v3::ast::*;

fn draw() -> Action {
    Action::Draw {
        quantity: Box::new(Quantity::ExactlyOf {
            amount: Box::new(Amount::Lit { value: 2 }),
        }),
    }
}

#[test]
fn event_gates_read_definition_headers_but_reject_bare_control_flow() {
    let compound = Action::Sequentially {
        steps: vec![draw(), draw()],
    };
    assert!(!compound.event_head_ok());
    let scoped = Action::OwnScope {
        deed: None,
        body: Box::new(compound.clone()),
    };
    assert!(!scoped.event_head_ok());
    let named = Action::InCaller {
        body: Box::new(Action::OwnScope {
            deed: Some(Deed::Action {
                label: "customDraw".into(),
                permanent: DeedPermanent::None,
            }),
            body: Box::new(compound),
        }),
    };
    assert!(named.event_head_ok());
    assert!(!named.event_has_magnitude());
    assert!(
        Action::OwnScope {
            deed: None,
            body: Box::new(draw())
        }
        .event_has_magnitude()
    );
}

#[test]
fn setting_life_publishes_both_outcomes_without_a_tally_magnitude() {
    let delta = Delta::Set {
        amount: Box::new(Amount::Lit { value: 7 }),
    };
    assert_eq!(
        delta.life_outcomes(),
        [OutcomeSort::LifeGained, OutcomeSort::LifeLost]
    );
    let action = Action::ChangeLife { delta };
    assert!(action.event_head_ok());
    assert!(!action.event_has_magnitude());
    assert!(
        Action::ChangeLife {
            delta: Delta::Down {
                amount: Box::new(Amount::Lit { value: 7 })
            }
        }
        .event_has_magnitude()
    );
}

#[test]
fn event_magnitude_survives_cause_and_occurrence_wrappers() {
    let event = GameEvent::Causes {
        cause: Box::new(Causing::AnEffect),
        event: Box::new(GameEvent::NthOccurrence {
            ordinal: Ordinal::Nth { n: 2 },
            per: None,
            event: Box::new(GameEvent::Does {
                subj: Box::new(Subject::Any),
                action: Box::new(draw()),
            }),
        }),
    };
    assert!(event.facts().has_magnitude);
    assert!(!GameEvent::Now.facts().has_magnitude);
}

#[test]
fn selected_group_preserves_carrier_and_exact_count() {
    let source = Referent::new(
        Determiner::The,
        Plurality::Many,
        Payload::Object {
            ty: vec![],
            zone: Some(Zone::Library),
            prov: None,
            orig: None,
            size: Some(2),
        },
    );
    let selection = selection_payload(
        &[CardType::creature()],
        Some(&source),
        &SliceCount::Counted {
            quantity: Box::new(Quantity::ExactlyOf {
                amount: Box::new(Amount::InCaller {
                    body: Box::new(Amount::Lit { value: 1 }),
                }),
            }),
        },
    );
    assert_eq!(selection.zone(), Some(&Zone::Library));
    assert_eq!(selection.types(), [CardType::creature()]);
    assert_eq!(selection.size(), Some(1));
    assert_eq!(Amount::Lit { value: -1 }.exact(), None);
    assert_eq!(
        Quantity::Range {
            low: Some(1),
            high: Some(2)
        }
        .exact(),
        None
    );
}

#[test]
fn local_checks_distinguish_missing_ambiguous_and_private_outcomes() {
    let mut drs = DRS::default();
    assert_eq!(
        DieSides::ThoseDice.local_checks(&drs),
        [Refusal::OutcomeInScope {
            s: OutcomeSort::DiceRolled,
            found: 0
        }]
    );
    let outcome = Referent::new(
        Determiner::The,
        Plurality::One,
        Payload::Outcome {
            sort: OutcomeSort::DiceRolled,
        },
    );
    drs.prepend(vec![outcome.clone()], GramRole::Other, false);
    assert_eq!(
        DieSides::ThoseDice.local_checks(&drs),
        Vec::<Refusal>::new()
    );
    drs.prepend(vec![outcome], GramRole::Other, false);
    assert_eq!(
        DieSides::ThoseDice.local_checks(&drs),
        [Refusal::OutcomeInScope {
            s: OutcomeSort::DiceRolled,
            found: 2
        }]
    );
    drs.enter_own_scope(None);
    assert_eq!(
        DieSides::ThoseDice.local_checks(&drs),
        [Refusal::OutcomeInScope {
            s: OutcomeSort::DiceRolled,
            found: 0
        }]
    );
}

#[test]
fn embedded_text_keeps_choices_and_discards_ordinary_mentions() {
    let mut drs = DRS::default();
    let ordinary = Referent::new(
        Determiner::A,
        Plurality::One,
        Payload::Player { chosen: false },
    );
    let ids = drs.prepend(
        vec![
            ChoiceSort::Quality {
                sort: QualitySort::Color,
            }
            .binding(),
            ChoiceSort::Player.binding(),
            ordinary,
        ],
        GramRole::Other,
        false,
    );
    let chosen = drs.choice_context();
    assert_eq!(
        chosen.referents.iter().map(|r| r.id).collect::<Vec<_>>(),
        ids[..2]
    );
    assert_eq!(chosen.next_id, drs.next_id);
    drs.enter_own_scope(None);
    assert_eq!(drs.choice_context().referents, Vec::<Referent>::new());
}

#[test]
fn count_shapes_reject_zero_dice_and_nonliteral_roll_ranges() {
    assert_eq!(
        DieSides::Sides { sides: 0 }.local_checks(&DRS::default()),
        [Refusal::NonZeroQ]
    );
    assert_eq!(
        RollWatch::ResultIn {
            quantity: Box::new(Quantity::ExactlyOf {
                amount: Box::new(Amount::Lit { value: 0 })
            })
        }
        .local_checks(),
        [Refusal::NonZeroQ, Refusal::QuantLiteral]
    );
    assert_eq!(
        RollWatch::ResultIn {
            quantity: Box::new(Quantity::Range {
                low: Some(1),
                high: Some(6)
            })
        }
        .local_checks(),
        Vec::<Refusal>::new()
    );
    assert!(
        !Quantity::Range {
            low: Some(1),
            high: Some(4)
        }
        .modes_fit(3)
    );
}

#[test]
fn copied_token_metadata_uses_the_selected_source_and_performer_number() {
    let creation = CreationSpec::Token {
        spec: Box::new(TokenSpec::CopyOf {
            source: Box::new(NounPhrase::You),
            exceptions: vec![],
        }),
        riders: vec![],
    };
    let referent = creation.created_referent(
        &Plurality::One,
        &Plurality::Many,
        &[
            CardType::artifact(),
            CardType::creature(),
            CardType::artifact(),
        ],
    );
    assert_eq!(
        referent.payload.types(),
        [CardType::creature(), CardType::artifact()]
    );
    assert_eq!(referent.payload.zone(), Some(&Zone::Battlefield));
    assert_eq!(referent.payload.origin(), Some(&Origin::Token));
    assert_eq!(referent.plur, Plurality::Many);
    assert!(referent.created_result);
}

#[test]
fn linked_text_publishes_grantor_objects_at_fresh_identities() {
    let moved = Referent::new(
        Determiner::The,
        Plurality::One,
        Payload::Object {
            ty: vec![CardType::creature()],
            zone: Some(Zone::Exile),
            prov: Some(Stamp {
                deeds: vec![],
                was_field: true,
                zone_change: Some(ZoneChange {
                    origin: Some(Zone::Battlefield),
                    destination: Some(Zone::Exile),
                }),
                left_field: true,
            }),
            orig: None,
            size: None,
        },
    );
    let mut drs = DRS::default();
    drs.prepend(vec![moved.clone()], GramRole::Obj, false);
    let linked = drs.linked_text_context();
    assert_eq!(linked.referents.len(), 1);
    assert!(linked.referents[0].linked);
    assert_eq!(linked.referents[0].link_site, drs.next_id);
    drs.grantor_linked = Some(GrantorLinked {
        since: 0,
        published: vec![moved],
        before: vec![],
    });
    let linked = drs.linked_text_context();
    assert_eq!(linked.referents.len(), 1);
    assert_eq!(linked.referents[0].id, drs.next_id + 1);
    assert_eq!(linked.referents[0].link_site, drs.next_id + 1);
    assert_eq!(linked.next_id, drs.next_id + 2);
}
