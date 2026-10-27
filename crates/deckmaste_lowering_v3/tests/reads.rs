use deckmaste_lowering_v3::reads::ActorShape;
use deckmaste_lowering_v3::reads::DRS;
use deckmaste_lowering_v3::reads::Determiner;
use deckmaste_lowering_v3::reads::GramRole;
use deckmaste_lowering_v3::reads::Payload;
use deckmaste_lowering_v3::reads::Referent;
use deckmaste_semantics_v3::ast::Kind;
use deckmaste_semantics_v3::ast::NounWord;
use deckmaste_semantics_v3::ast::Plurality;
use deckmaste_semantics_v3::ast::Reach;
use deckmaste_semantics_v3::ast::SlotCarrier;
use deckmaste_semantics_v3::ast::Zone;
use dpsi::Ident;

fn object(det: Determiner) -> Referent {
    Referent::new(
        det,
        Plurality::One,
        Payload::Object {
            ty: vec![],
            zone: Some(Zone::Battlefield),
            prov: None,
            orig: None,
            size: None,
        },
    )
}
fn add(drs: &mut DRS, role: GramRole) -> u64 {
    drs.prepend(vec![object(Determiner::A)], role, false)[0]
}
#[test]
fn bare_pronouns_rank_clause_then_role_and_refuse_ties() {
    let mut drs = DRS::default();
    let subject = add(&mut drs, GramRole::Subj);
    add(&mut drs, GramRole::Obj);
    let winner = drs.resolve_candidates(&Reach::Bare, &Plurality::One, false);
    assert_eq!(winner[0].id, subject);
    add(&mut drs, GramRole::Subj);
    assert_eq!(
        drs.resolve_candidates(&Reach::Bare, &Plurality::One, false)
            .len(),
        2
    );
    drs.next_clause();
    let recent = add(&mut drs, GramRole::Other);
    assert_eq!(
        drs.resolve_candidates(&Reach::Bare, &Plurality::One, false)[0].id,
        recent
    );
}
#[test]
fn buffered_subject_is_a_clause_mate_but_ranks_for_possessors() {
    let mut drs = DRS::default();
    let subject = add(&mut drs, GramRole::Other);
    drs.next_clause();
    drs.record_mention(subject, GramRole::Subj, true, false);
    assert_eq!(
        drs.resolve_candidates(&Reach::Bare, &Plurality::One, true),
        Vec::<&Referent>::new()
    );
    assert_eq!(
        drs.resolve_candidates(&Reach::Bare, &Plurality::One, false)[0].id,
        subject
    );
    assert_eq!(drs.resolve_reflexive(&Plurality::One)[0].id, subject);
}
#[test]
fn demonstrative_anticoreference_is_a_preference() {
    let mut drs = DRS::default();
    let old = add(&mut drs, GramRole::Other);
    let new = add(&mut drs, GramRole::Other);
    drs.record_mention(new, GramRole::Other, false, true);
    let reach = Reach::Word {
        word: Box::new(NounWord::Permanent),
    };
    assert_eq!(
        drs.resolve_candidates(&reach, &Plurality::One, false)[0].id,
        old
    );
    drs.referents.retain(|r| r.id != old);
    assert_eq!(
        drs.resolve_candidates(&reach, &Plurality::One, false)[0].id,
        new
    );
}
#[test]
fn own_scope_masks_caller_but_preserves_performer_and_bound_arguments() {
    let mut drs = DRS::default();
    let actor = add(&mut drs, GramRole::Subj);
    let argument = add(&mut drs, GramRole::Obj);
    drs.performers.push(ActorShape {
        kind: Box::new(Kind::Object),
        is_you: false,
        ascribable: true,
        ref_ids: vec![actor],
        plur: Plurality::One,
        selves: vec![],
    });
    drs.enter_own_scope(None);
    assert!(drs.is_visible(actor));
    assert!(!drs.is_visible(argument));
    drs.in_caller(|caller| {
        assert!(caller.is_visible(argument));
        caller.record_mention(argument, GramRole::Obj, true, false);
    });
    assert!(drs.is_visible(argument));
    drs.leave_own_scope();
    assert_eq!(
        drs.scope_stack,
        Vec::<deckmaste_lowering_v3::reads::OwnScopeFrame>::new()
    );
    assert!(drs.pending_roles.contains(&(argument, GramRole::Obj)));
}
#[test]
fn coordinated_identity_returns_every_member_without_reintroducing_them() {
    let mut drs = DRS::default();
    let first = add(&mut drs, GramRole::Obj);
    let second = add(&mut drs, GramRole::Obj);
    let identity = Ident::new("pair");
    drs.first_mention(identity, &[first, second]);
    let before = drs.next_id;
    let ids = drs.later_mention(identity, GramRole::Obj, true).unwrap();
    assert_eq!(ids, vec![second, first]);
    assert_eq!(drs.next_id, before);
    assert_eq!(drs.pending_roles.len(), 2);
}
#[test]
fn failed_first_mention_is_not_reported_again() {
    let mut drs = DRS::default();
    let identity = Ident::new("missing");
    assert!(drs.later_mention(identity, GramRole::Other, false).is_err());
    drs.first_mention(identity, &[]);
    assert_eq!(
        drs.later_mention(identity, GramRole::Other, false).unwrap(),
        Vec::<u64>::new()
    );
}
#[test]
fn consuming_slot_excludes_departed_object_before_ranking() {
    let mut drs = DRS::default();
    let live = add(&mut drs, GramRole::Other);
    let mut departed = object(Determiner::A);
    if let Payload::Object { zone, .. } = departed.payload.as_mut() {
        *zone = Some(Zone::Graveyard);
    }
    drs.prepend(vec![departed], GramRole::Subj, false);
    let reach = Reach::AtSlot {
        slot: SlotCarrier::Permanent,
    };
    assert_eq!(
        drs.resolve_candidates(&reach, &Plurality::One, false)[0].id,
        live
    );
}
#[test]
fn modal_box_closes_hypotheticals_and_keeps_universal_domains() {
    let before = DRS::default();
    let mut after = before.clone();
    add(&mut after, GramRole::Subj);
    let universal = after.prepend(vec![object(Determiner::All)], GramRole::Obj, true)[0];
    after.close_modal_box(&before);
    assert_eq!(after.referents.len(), 1);
    assert_eq!(after.referents[0].id, universal);
}
#[test]
fn measured_box_keeps_targets_and_attachment_hosts_only() {
    let before = DRS::default();
    let mut after = before.clone();
    add(&mut after, GramRole::Other);
    let target = after.prepend(vec![object(Determiner::Target)], GramRole::Obj, false)[0];
    let mut host = object(Determiner::The);
    host.attachment_host = true;
    let host = after.prepend(vec![host], GramRole::Other, false)[0];
    let closed = DRS::leave_measure(&before, &after);
    assert_eq!(
        closed.referents.iter().map(|r| r.id).collect::<Vec<_>>(),
        vec![target, host]
    );
    assert_eq!(closed.next_id, after.next_id);
}

#[test]
fn zone_move_clears_binding_identity_and_remaps_scope_bookkeeping() {
    let mut drs = DRS::default();
    let old = add(&mut drs, GramRole::Obj);
    let identity = Ident::new("patient");
    drs.first_mention(identity, &[old]);
    drs.record_mention(old, GramRole::Obj, true, false);
    drs.enter_own_scope(None);
    drs.apply_zone_move(&[old, old], None, Some(&Zone::Graveyard));
    let fresh = drs.referents[0].id;
    assert_ne!(fresh, old);
    assert_eq!(drs.referents[0].moved_from, vec![old]);
    assert!(!drs.is_visible(fresh));
    assert_eq!(
        drs.scope_stack[0].entry_pending,
        vec![(fresh, GramRole::Obj)]
    );
    drs.leave_own_scope();
    assert!(drs.is_visible(fresh));
    assert_eq!(drs.referents[0].payload.zone(), Some(&Zone::Graveyard));
    assert!(drs.later_mention(identity, GramRole::Obj, false).is_err());
}
