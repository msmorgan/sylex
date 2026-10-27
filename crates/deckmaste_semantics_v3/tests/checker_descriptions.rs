use deckmaste_semantics_v3::ast::CardType;
use deckmaste_semantics_v3::ast::Coordinator;
use deckmaste_semantics_v3::ast::Deed;
use deckmaste_semantics_v3::ast::DeedPermanent;
use deckmaste_semantics_v3::ast::Kind;
use deckmaste_semantics_v3::ast::NounPhrase;
use deckmaste_semantics_v3::ast::NounWord;
use deckmaste_semantics_v3::ast::Plurality;
use deckmaste_semantics_v3::ast::PossessorAxis;
use deckmaste_semantics_v3::ast::Predicate;
use deckmaste_semantics_v3::ast::Zone;
use deckmaste_semantics_v3::reads::DRS;
use deckmaste_semantics_v3::reads::Determiner;
use deckmaste_semantics_v3::reads::GramRole;
use deckmaste_semantics_v3::reads::ParticipialAnchor;
use deckmaste_semantics_v3::reads::Payload;
use deckmaste_semantics_v3::reads::Referent;
use deckmaste_semantics_v3::reads::Stamp;
use deckmaste_semantics_v3::reads::ZoneChange;
use deckmaste_semantics_v3::reads::participial_read;

fn creature() -> Referent {
    Referent::new(
        Determiner::Target,
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
fn deed(label: &str) -> Deed {
    Deed::Action {
        label: label.into(),
        permanent: DeedPermanent::None,
    }
}
fn stamped() -> Referent {
    let mut r = creature();
    if let Payload::Object { zone, prov, .. } = r.payload.as_mut() {
        *zone = Some(Zone::Exile);
        *prov = Some(Stamp {
            deeds: vec![deed("exile")],
            was_field: true,
            zone_change: Some(ZoneChange {
                origin: Some(Zone::Battlefield),
                destination: Some(Zone::Exile),
            }),
            left_field: true,
        });
    }
    r
}
#[test]
fn alternative_headers_intersect_bindings_and_reserve_every_allocated_identity() {
    let before = DRS::default();
    let mut left = before.clone();
    let left_id = left.prepend(vec![creature()], GramRole::Subj, true)[0];
    left.first_mention("common".into(), &[left_id]);
    left.first_mention("left_only".into(), &[left_id]);
    let mut right = before.clone();
    right.next_id = 10;
    let right_id = right.prepend(vec![creature()], GramRole::Obj, true)[0];
    right.first_mention("common".into(), &[right_id]);
    let merged = before.common_header_context(&[left, right]);
    assert_eq!(merged.next_id, 11);
    assert_eq!(merged.referents.len(), 1);
    assert_eq!(merged.referents[0].id, left_id);
    assert_eq!(
        merged.referents[0].idents,
        vec![dpsi::Ident::from("common")]
    );
    assert_eq!(merged.pending_roles, vec![(left_id, GramRole::Subj)]);
}
#[test]
fn alternative_headers_do_not_publish_branch_local_or_ambiguous_descriptions() {
    let before = DRS::default();
    let mut left = before.clone();
    left.prepend(vec![creature()], GramRole::Obj, true);
    let merged = before.common_header_context(&[left.clone(), before.clone()]);
    assert_eq!(merged.referents, Vec::<Referent>::new());
    assert_eq!(merged.next_id, 1);
    let mut right = before.clone();
    right.prepend(vec![creature()], GramRole::Obj, true);
    right.prepend(vec![creature()], GramRole::Obj, true);
    let merged = before.common_header_context(&[left, right]);
    assert_eq!(merged.referents, Vec::<Referent>::new());
    assert_eq!(merged.pending_roles, Vec::<(u64, GramRole)>::new());
}
#[test]
fn alternative_headers_keep_only_unanimous_refinements_of_inherited_referents() {
    let mut before = DRS::default();
    before.prepend(vec![creature()], GramRole::Other, false);
    let mut left = before.clone();
    left.referents[0].controlled_by_you = Some(true);
    let merged = before.common_header_context(&[left.clone(), before.clone()]);
    assert_eq!(merged.referents, before.referents);
    let merged = before.common_header_context(&[left.clone(), left.clone()]);
    assert_eq!(merged.referents, left.referents);
}
#[test]
fn definite_descriptions_require_matching_control_and_reject_moved_mentions() {
    let mut drs = DRS::default();
    let mut yours = creature();
    yours.controlled_by_you = Some(true);
    let id = drs.prepend(vec![yours], GramRole::Obj, false)[0];
    let mut theirs = creature();
    theirs.controlled_by_you = Some(false);
    drs.prepend(vec![theirs], GramRole::Obj, false);
    let p = Predicate::Coord {
        coordinator: Coordinator::And,
        items: vec![
            Predicate::HasType {
                type_: CardType::creature(),
            },
            Predicate::HasPossessor {
                axis: PossessorAxis::Controller,
                possessor: Box::new(NounPhrase::You),
            },
        ],
    };
    assert_eq!(
        drs.definite_antecedents(&p)
            .iter()
            .map(|r| r.id)
            .collect::<Vec<_>>(),
        vec![id]
    );
    drs.referents
        .iter_mut()
        .find(|r| r.id == id)
        .unwrap()
        .moved_from
        .push(99);
    assert_eq!(drs.definite_antecedents(&p), Vec::<&Referent>::new());
}
#[test]
fn participial_descriptions_read_departed_permanents_and_distinguish_this_way() {
    let mut drs = DRS::default();
    let id = drs.prepend(vec![stamped()], GramRole::Obj, false)[0];
    let anchor = ParticipialAnchor::Deed {
        deed: deed("exile"),
    };
    assert_eq!(
        drs.participial_candidates(
            &anchor,
            Some(&Plurality::One),
            Some(&NounWord::Permanent),
            true,
            &[]
        )[0]
        .id,
        id
    );
    assert_eq!(
        drs.participial_candidates(
            &anchor,
            Some(&Plurality::One),
            Some(&NounWord::Spell),
            true,
            &[]
        ),
        Vec::<&Referent>::new()
    );
    drs.referents[0].linked = true;
    assert_eq!(
        drs.participial_candidates(&anchor, Some(&Plurality::One), None, true, &[]),
        Vec::<&Referent>::new()
    );
    assert_eq!(
        drs.participial_candidates(&anchor, Some(&Plurality::One), None, false, &[])
            .len(),
        1
    );
}
#[test]
fn participial_readback_counts_exclusive_link_arms_as_one_publication() {
    let mut left = stamped();
    left.id = 10;
    left.linked = true;
    left.link_site = 2;
    left.link_arms = vec![(0, 0)];
    let mut right = left.clone();
    right.id = 11;
    right.link_arms = vec![(0, 1)];
    assert_eq!(
        participial_read(Some(&Plurality::One), &[&left, &right])
            .unwrap()
            .len(),
        2
    );
    right.link_site = 3;
    assert_eq!(
        participial_read(Some(&Plurality::One), &[&left, &right]),
        Err(2)
    );
}
#[test]
fn coordination_preserves_members_and_publishes_a_plural_group() {
    let mut drs = DRS::default();
    let ids = drs.prepend(vec![creature(), creature()], GramRole::Obj, false);
    let groups = drs.push_coordinated_group(0, &ids, GramRole::Obj, false);
    assert_eq!(groups.len(), 1);
    let group = drs.find_by_id(groups[0]).unwrap();
    assert_eq!(group.plur, Plurality::Many);
    for id in ids {
        assert_eq!(drs.find_by_id(id).unwrap().part_of, Some(groups[0]));
    }
}
#[test]
fn contrast_anchors_respect_exclusion_control_and_source_compatibility() {
    let mut drs = DRS::default();
    let mut other = creature();
    other.controlled_by_you = Some(false);
    let id = drs.prepend(vec![other], GramRole::Obj, false)[0];
    assert_eq!(
        drs.contrast_anchor(&Kind::Object, &[], None, Some(false)),
        Some(deckmaste_semantics_v3::reads::ContrastAnchor::Mention { id })
    );
    drs.contrast_excluded.push(id);
    assert_eq!(
        drs.contrast_anchor(&Kind::Object, &[], None, Some(false)),
        None
    );
    assert_eq!(
        drs.contrast_anchor(&Kind::Player, &[], None, None),
        Some(deckmaste_semantics_v3::reads::ContrastAnchor::You)
    );
}
