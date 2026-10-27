//! Noun-phrase stepping with discourse identity and grammatical roles.
use super::DRS;
use super::Determiner;
use super::GramRole;
use super::NounResult;
use super::Payload;
use super::Referent;
use super::StepOps;
use super::circumstances::kind_check;
use super::circumstances::refuse;
use crate::ast::AttachWord;
use crate::ast::Coordinator;
use crate::ast::DesignationLabel;
use crate::ast::DesignationScope;
use crate::ast::Kind;
use crate::ast::Location;
use crate::ast::NounPhrase;
use crate::ast::NounWord;
use crate::ast::Plurality;
use crate::ast::Reach;
use crate::ast::Zone;
use crate::refusal::Refusal;

fn introduce(
    mut drs: DRS,
    reference: Referent,
    role: GramRole,
    in_arg_slot: bool,
    errors: Vec<Refusal>,
) -> NounResult {
    let ids = drs.prepend(vec![reference], role, in_arg_slot);
    (drs, ids, errors)
}
pub fn step_noun_with(
    ops: &impl StepOps,
    mut drs: DRS,
    role: GramRole,
    in_arg_slot: bool,
    noun: &NounPhrase,
) -> NounResult {
    let context = |drs| NounContext {
        ops,
        drs,
        role,
        in_arg_slot,
    };
    match noun {
        NounPhrase::InCaller { body } => {
            let mut caller = drs.clone();
            caller.enter_caller();
            let (mut out, ids, errors) = ops.noun(caller, role, in_arg_slot, body);
            out.leave_caller(&drs, &ids);
            (out, ids, errors)
        }
        NounPhrase::FirstMention { id, phrase } => {
            let (mut out, ids, errors) = ops.noun(drs, role, in_arg_slot, phrase);
            out.first_mention(*id, &ids);
            (out, ids, errors)
        }
        NounPhrase::LaterMention { id, .. } => match drs.later_mention(*id, role, in_arg_slot) {
            Ok(ids) => (drs, ids, Vec::new()),
            Err(error) => (drs, Vec::new(), vec![error]),
        },
        NounPhrase::Pro { reach, plurality } => context(drs).pronoun(reach, plurality),
        NounPhrase::Refl { plurality } => context(drs).reflexive(plurality),
        NounPhrase::Which => {
            if let Some(id) = drs
                .relative_binder
                .filter(|id| drs.is_visible(*id) && drs.find_by_id(*id).is_some())
            {
                drs.record_mention(id, role, in_arg_slot, false);
                drs.relative_uses += 1;
                (drs, vec![id], Vec::new())
            } else {
                (drs, Vec::new(), vec![Refusal::WhichUnbound])
            }
        }
        NounPhrase::Described {
            determiner,
            predicate,
        } => super::step_description_with(ops, &drs, role, in_arg_slot, determiner, predicate),
        NounPhrase::Coord {
            coordinator: Coordinator::Or,
            phrases,
        } => super::step_noun_alternatives_with(ops, &drs, role, in_arg_slot, phrases),
        NounPhrase::Coord { phrases, .. } => context(drs).coordinated(phrases),
        NounPhrase::EachOf { group } => context(drs).each_of(group),
        NounPhrase::SomeOf {
            count,
            description,
            group,
        } => context(drs).some_of(count, description.as_deref(), group),
        NounPhrase::Slice { amount, pile, .. } => {
            context(drs).slice(amount, pile, noun.plurality())
        }
        NounPhrase::Run { stop, pile, .. } => context(drs).run(stop, pile, noun.plurality()),
        NounPhrase::PileOf { count, by_ } => context(drs).pile_of(count, by_.as_deref()),
        NounPhrase::OneEachOf { roles, pool } => context(drs).one_each_of(roles, pool),
        NounPhrase::TheRest { plurality } => context(drs).rest(plurality),
        NounPhrase::PossessorOf { axis, subject } => context(drs).possessor(axis, subject),
        NounPhrase::CombatPlayer { .. } => {
            introduce(drs, Kind::Player.binding(), role, in_arg_slot, Vec::new())
        }
        NounPhrase::Headed { head, subject } => context(drs).headed(head, subject),
        NounPhrase::Agree { group, .. } => context(drs).agreement(group),
        NounPhrase::Becomes { subject } => context(drs).becomes(subject),
        NounPhrase::Counterpart { subject, .. } => {
            let (out, _, errors) = ops.noun(drs, GramRole::Other, false, subject);
            let mut reference = Kind::Object.binding();
            reference.payload = Box::new(NounWord::Permanent.read_payload(&reference.payload));
            introduce(out, reference, role, in_arg_slot, errors)
        }
        NounPhrase::AttachHost { word, head } => context(drs).attachment_host(word, head),
        NounPhrase::TheGrantor { head } => {
            let mut reference = Kind::Object.binding();
            reference.payload = Box::new(head.read_payload(&reference.payload));
            introduce(
                drs,
                reference,
                role,
                in_arg_slot,
                refuse(head.is_head(), Refusal::HeadNoun),
            )
        }
        NounPhrase::Designated { designation, whose } => {
            context(drs).designated(designation, whose)
        }
        NounPhrase::You => super::mention_you(drs, &noun.plurality(), role, in_arg_slot),
        NounPhrase::This => super::mention_this(drs, role, in_arg_slot),
        NounPhrase::Actor => {
            if let Some(shape) = drs.performers.first().cloned() {
                if shape.ref_ids.is_empty() {
                    introduce(drs, shape.kind.binding(), role, in_arg_slot, Vec::new())
                } else {
                    if in_arg_slot {
                        for id in &shape.ref_ids {
                            drs.record_mention(*id, role, true, false);
                        }
                    }
                    (drs, shape.ref_ids, Vec::new())
                }
            } else {
                super::mention_you(drs, &Plurality::One, role, in_arg_slot)
            }
        }
        NounPhrase::Any => (drs, Vec::new(), Vec::new()),
    }
}

struct NounContext<'a, O> {
    ops: &'a O,
    drs: DRS,
    role: GramRole,
    in_arg_slot: bool,
}
impl<O: StepOps> NounContext<'_, O> {
    fn pronoun(self, reach: &Reach, plurality: &Plurality) -> NounResult {
        let Self {
            mut drs,
            role,
            in_arg_slot,
            ..
        } = self;
        let winners: Vec<_> = drs
            .resolve_pronoun(reach, plurality, in_arg_slot)
            .iter()
            .map(|r| r.id)
            .collect();
        if let [id] = winners.as_slice() {
            drs.record_mention(*id, role, in_arg_slot, reach.is_bare_pronoun());
            (drs, winners, Vec::new())
        } else {
            (
                drs,
                Vec::new(),
                vec![Refusal::Anaphor {
                    r: reach.clone(),
                    pl: plurality.clone(),
                    found: winners.len() as u64,
                }],
            )
        }
    }
    fn reflexive(self, plurality: &Plurality) -> NounResult {
        let Self {
            mut drs,
            role,
            in_arg_slot,
            ..
        } = self;
        let winners: Vec<_> = drs
            .resolve_reflexive(plurality)
            .iter()
            .map(|r| r.id)
            .collect();
        if let [id] = winners.as_slice() {
            drs.record_mention(*id, role, in_arg_slot, false);
            (drs, winners, Vec::new())
        } else {
            (
                drs,
                Vec::new(),
                vec![Refusal::ReflexiveAntecedent {
                    pl: plurality.clone(),
                    found: winners.len() as u64,
                }],
            )
        }
    }
    fn coordinated(self, phrases: &[NounPhrase]) -> NounResult {
        let Self {
            ops,
            mut drs,
            role,
            in_arg_slot,
            ..
        } = self;
        let start = drs.next_id;
        let coordinated = drs.coordinated_self;
        let mut ids = Vec::new();
        let mut errors = refuse(phrases.len() >= 2, Refusal::AtLeastTwo);
        for phrase in phrases {
            drs.coordinated_self = drs.coordinated_self_in(&ids);
            let (out, more_ids, more) = ops.noun(drs, role, in_arg_slot, phrase);
            drs = out;
            ids.extend(more_ids);
            errors.extend(more);
        }
        drs.coordinated_self = coordinated;
        let group_ids = drs.push_coordinated_group(start, &ids, role, in_arg_slot);
        ids.extend(group_ids);
        (drs, ids, errors)
    }
    fn each_of(self, group: &NounPhrase) -> NounResult {
        let Self {
            ops,
            drs,
            role,
            in_arg_slot,
            ..
        } = self;
        let (out, ids, mut errors) = ops.noun(drs, GramRole::Other, false, group);
        let mut reference = Referent::new(
            Determiner::Each,
            Plurality::One,
            ids.first()
                .and_then(|id| out.find_by_id(*id))
                .map_or_else(|| *Kind::Object.binding().payload, |r| *r.payload.clone()),
        );
        reference.member_of = ids.first().copied();
        errors.extend(refuse(
            group.plurality() == Plurality::Many || group.determiner() == Some(Determiner::Each),
            Refusal::Plural,
        ));
        errors.extend(refuse(group.group_mention(), Refusal::GroupMention));
        introduce(out, reference, role, in_arg_slot, errors)
    }
    fn some_of(
        self,
        count: &crate::ast::SliceCount,
        description: Option<&crate::ast::Predicate>,
        group: &NounPhrase,
    ) -> NounResult {
        let Self {
            ops,
            drs,
            role,
            in_arg_slot,
            ..
        } = self;
        let (context, mut errors) = super::step_slice_count_with(ops, drs, count);
        let (context, more) = if let Some(p) = description {
            ops.predicate(context, p)
        } else {
            (context, Vec::new())
        };
        errors.extend(more);
        let (mut out, ids, more) = ops.noun(context, GramRole::Other, false, group);
        errors.extend(more);
        let types = description
            .as_ref()
            .map_or_else(Vec::new, |p| p.seed_types());
        let payload = super::selection_payload(
            &types,
            ids.first().and_then(|id| out.find_by_id(*id)),
            count,
        );
        for r in &mut out.referents {
            if ids.contains(&r.id) && r.group_key.is_none() {
                r.group_key = Some(r.id);
            }
        }
        let mut reference = Referent::new(Determiner::Part, count.plurality(), payload);
        reference.part_of = ids
            .first()
            .and_then(|id| out.find_by_id(*id))
            .and_then(|r| r.group_key);
        let kind = reference.payload.kind();
        let result_ids = out.prepend(vec![reference], role, in_arg_slot);
        for id in ids {
            out.record_selection(kind.clone(), Some(id), false);
        }
        errors.extend(refuse(group.partitive_base(), Refusal::PartitiveBase));
        (out, result_ids, errors)
    }
    fn slice(
        self,
        amount: &crate::ast::Amount,
        pile: &Location,
        plurality: Plurality,
    ) -> NounResult {
        let Self {
            ops,
            drs,
            role,
            in_arg_slot,
            ..
        } = self;
        let (out, mut errors) = ops.amount(drs, amount);
        let (out, more) = super::step_location_with(ops, out, pile);
        errors.extend(more);
        if let Location::Zone {
            zone: Zone::Library | Zone::Graveyard,
            whose: Some(owner),
        } = pile
        {
            errors.extend(refuse(owner.slice_possessor_ok(), Refusal::SlicePossessor));
        }
        introduce(
            out,
            Referent::new(
                Determiner::The,
                plurality,
                super::slice_payload(pile.sort(), amount),
            ),
            role,
            in_arg_slot,
            errors,
        )
    }
    fn run(
        self,
        stop: &crate::ast::Predicate,
        pile: &Location,
        plurality: Plurality,
    ) -> NounResult {
        let Self {
            ops,
            drs,
            role,
            in_arg_slot,
            ..
        } = self;
        let (out, mut errors) = ops.predicate(drs, stop);
        let (mut out, more) = super::step_location_with(ops, out, pile);
        errors.extend(more);
        let (run, stopped) = super::run_payloads(pile.sort(), stop.seed_types());
        let mut reference = Referent::new(Determiner::Part, Plurality::One, stopped);
        reference.part_of = Some(out.next_id + 1);
        let stopped_id = out.prepend(vec![reference], GramRole::Other, false)[0];
        let mut reference = Referent::new(Determiner::The, plurality, run);
        reference.group_key = Some(out.next_id);
        let run_id = out.prepend(vec![reference], role, in_arg_slot)[0];
        (out, vec![run_id, stopped_id], errors)
    }
    fn pile_of(self, count: &crate::ast::SliceCount, by_: Option<&NounPhrase>) -> NounResult {
        let Self {
            ops,
            drs,
            role,
            in_arg_slot,
            ..
        } = self;
        let (out, mut errors) = super::step_slice_count_with(ops, drs, count);
        let (out, more) = if let Some(by_) = by_ {
            let (out, _, errors) = ops.noun(out, GramRole::Other, false, by_);
            (out, errors)
        } else {
            (out, Vec::new())
        };
        errors.extend(more);
        let reach = Reach::Word {
            word: Box::new(NounWord::Pile),
        };
        let found = out.resolve_candidates(&reach, &Plurality::Many, false);
        let payload = Payload::Pile {
            zone: found.first().and_then(|r| r.payload.zone()).cloned(),
            size: None,
            face: found.first().and_then(|r| r.payload.face()).cloned(),
        };
        errors.extend(refuse(
            found.len() == 1,
            Refusal::Anaphor {
                r: reach,
                pl: Plurality::Many,
                found: found.len() as u64,
            },
        ));
        introduce(
            out,
            Referent::new(Determiner::Part, count.plurality(), payload),
            role,
            in_arg_slot,
            errors,
        )
    }
    fn one_each_of(self, roles: &[crate::ast::Predicate], pool: &NounPhrase) -> NounResult {
        let Self {
            ops,
            mut drs,
            role,
            in_arg_slot,
            ..
        } = self;
        let mut errors = Vec::new();
        for predicate in roles {
            let (out, more) = ops.predicate(drs, predicate);
            drs = out;
            errors.extend(more);
        }
        let (out, ids, more) = ops.noun(drs, GramRole::Other, false, pool);
        errors.extend(more);
        let source = ids.first().and_then(|id| out.find_by_id(*id));
        let payload = Payload::Object {
            ty: source.map_or_else(Vec::new, |r| r.payload.types()),
            zone: source.and_then(|r| r.payload.zone()).cloned(),
            prov: None,
            orig: None,
            size: None,
        };
        errors.extend(refuse(!roles.is_empty(), Refusal::NonEmpty));
        introduce(
            out,
            Referent::new(Determiner::A, Plurality::Many, payload),
            role,
            in_arg_slot,
            errors,
        )
    }
    fn rest(self, plurality: &Plurality) -> NounResult {
        let Self {
            drs,
            role,
            in_arg_slot,
            ..
        } = self;
        let kind = drs.rest_kind(plurality).unwrap_or(Kind::Object);
        let ok = drs.rest_fits(&kind, plurality);
        let source = drs.rest_group(&kind, plurality);
        let zone = source.and_then(|r| r.payload.zone()).cloned();
        let payload = match kind {
            Kind::Pile => Payload::Pile {
                zone,
                size: None,
                face: None,
            },
            Kind::Player => Payload::Player { chosen: false },
            _ => Payload::Object {
                ty: source.map_or_else(Vec::new, |r| r.payload.types()),
                zone,
                prov: None,
                orig: None,
                size: None,
            },
        };
        introduce(
            drs,
            Referent::new(Determiner::The, plurality.clone(), payload),
            role,
            in_arg_slot,
            refuse(
                ok,
                Refusal::TheRestFits {
                    plurality: plurality.clone(),
                },
            ),
        )
    }
    fn possessor(self, axis: &crate::ast::PossessorAxis, subject: &NounPhrase) -> NounResult {
        let Self {
            ops,
            mut drs,
            role,
            in_arg_slot,
            ..
        } = self;
        let saved = drs.characteristic_read;
        drs.characteristic_read = true;
        let (mut out, ids, mut errors) = ops.noun(drs, GramRole::Other, false, subject);
        out.characteristic_read = saved;
        let kind = subject.selected_kind(&out, &ids);
        errors.extend(refuse(
            kind.lte(&Kind::Object),
            Refusal::PossessorKind {
                axis: axis.clone(),
                k: Box::new(kind),
            },
        ));
        introduce(out, Kind::Player.binding(), role, in_arg_slot, errors)
    }
    fn headed(self, head: &NounWord, subject: &NounPhrase) -> NounResult {
        let Self {
            ops,
            drs,
            role,
            in_arg_slot,
            ..
        } = self;
        let ascribable = subject.ascribable_in(&drs);
        let (mut out, ids, mut errors) = ops.noun(drs, role, in_arg_slot, subject);
        errors.extend(kind_check(
            Kind::Object,
            ids.first()
                .and_then(|id| out.find_by_id(*id))
                .map(|r| r.payload.kind()),
        ));
        for r in &mut out.referents {
            if ids.contains(&r.id) {
                *r.payload = head.read_payload(&r.payload);
            }
        }
        errors.extend(refuse(ascribable, Refusal::Ascribable));
        errors.extend(refuse(head.is_head(), Refusal::HeadNoun));
        (out, ids, errors)
    }
    fn agreement(self, group: &NounPhrase) -> NounResult {
        let Self {
            ops,
            drs,
            role,
            in_arg_slot,
            ..
        } = self;
        let (out, ids, mut errors) = ops.noun(drs, role, in_arg_slot, group);
        errors.extend(refuse(
            group.plurality() == Plurality::Many,
            Refusal::Plural,
        ));
        errors.extend(refuse(
            matches!(
                group.determiner(),
                Some(Determiner::Count | Determiner::Target)
            ),
            Refusal::CountedMention,
        ));
        (out, ids, errors)
    }
    fn becomes(self, subject: &NounPhrase) -> NounResult {
        let Self {
            ops,
            drs,
            role,
            in_arg_slot,
            ..
        } = self;
        let (mut out, ids, mut errors) = ops.noun(drs, role, in_arg_slot, subject);
        errors.extend(kind_check(
            Kind::Object,
            ids.first()
                .and_then(|id| out.find_by_id(*id))
                .map(|r| r.payload.kind()),
        ));
        for r in &mut out.referents {
            if ids.contains(&r.id)
                && let Payload::Object { zone, .. } = r.payload.as_mut()
            {
                *zone = Some(Zone::Battlefield);
            }
        }
        (out, ids, errors)
    }
    fn attachment_host(self, word: &AttachWord, head: &NounWord) -> NounResult {
        let Self {
            drs,
            role,
            in_arg_slot,
            ..
        } = self;
        let kind = head.kind();
        let payload = if kind == Kind::Player {
            Payload::Player { chosen: false }
        } else {
            Payload::Object {
                ty: head.attach_host_types(),
                zone: head.attach_host_zone(),
                prov: None,
                orig: None,
                size: None,
            }
        };
        let mut reference = Referent::new(Determiner::The, Plurality::One, payload);
        reference.attachment_host = true;
        let ok = (*word == AttachWord::Enchanted
            && !matches!(head, NounWord::OfType { .. } | NounWord::Copied { .. }))
            || kind.lte(&Kind::Object);
        introduce(
            drs,
            reference,
            role,
            in_arg_slot,
            refuse(
                ok,
                Refusal::KindLte {
                    k: Box::new(kind),
                    bound: Box::new(Kind::Object),
                },
            ),
        )
    }
    fn designated(self, designation: &DesignationLabel, whose: &NounPhrase) -> NounResult {
        let Self {
            ops,
            drs,
            role,
            in_arg_slot,
            ..
        } = self;
        let (out, ids, mut errors) = ops.noun(drs, GramRole::Other, false, whose);
        errors.extend(kind_check(
            Kind::Player,
            ids.first()
                .and_then(|id| out.find_by_id(*id))
                .map(|r| r.payload.kind()),
        ));
        errors.extend(refuse(
            matches!(
                designation,
                DesignationLabel::Named {
                    scope: DesignationScope::HeldByCard,
                    ..
                }
            ),
            Refusal::DesignationScope {
                designation: designation.clone(),
            },
        ));
        introduce(out, Kind::Object.binding(), role, in_arg_slot, errors)
    }
}
