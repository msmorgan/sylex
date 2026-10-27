//! Selection evidence and remainder groups, from `Check/Words`.
use super::DRS;
use super::Determiner;
use super::Referent;
use super::ReferentId;
use super::SelectionEvidence;
use super::whole_groups;
use crate::ast::Kind;
use crate::ast::Plurality;
fn group(kind: &Kind, r: &Referent) -> bool {
    kind.lte(&r.payload.kind()) && r.plur == Plurality::Many
}
fn prefer_ordinary(refs: Vec<&Referent>) -> Vec<&Referent> {
    if refs.iter().any(|r| !r.replacement_domain) {
        refs.into_iter().filter(|r| !r.replacement_domain).collect()
    } else {
        refs
    }
}
impl DRS {
    #[must_use]
    pub fn count_many(&self, kind: &Kind) -> usize {
        self.visible_referents().filter(|r| group(kind, r)).count()
    }
    #[must_use]
    pub fn count_many_any(&self) -> usize {
        whole_groups(
            &self
                .visible_referents()
                .filter(|r| r.plur == Plurality::Many)
                .collect::<Vec<_>>(),
        )
        .len()
    }
    #[must_use]
    pub fn count_groups(&self, kind: &Kind) -> usize {
        prefer_ordinary(whole_groups(
            &self
                .visible_referents()
                .filter(|r| !r.linked && r.det != Determiner::Part && group(kind, r))
                .collect::<Vec<_>>(),
        ))
        .len()
    }
    #[must_use]
    pub fn count_chosen_objects(&self, kind: &Kind) -> usize {
        self.visible_referents()
            .filter(|r| r.chosen_object && !r.linked && kind.lte(&r.payload.kind()))
            .count()
    }
    #[must_use]
    pub fn count_parts(&self, kind: &Kind) -> usize {
        self.visible_referents()
            .filter(|r| !r.linked && r.det == Determiner::Part && kind.lte(&r.payload.kind()))
            .count()
    }
    pub fn record_selection(&mut self, kind: Kind, source_group: Option<ReferentId>, chosen: bool) {
        self.selection_evidence.insert(
            0,
            SelectionEvidence {
                kind: Box::new(kind),
                source_group,
                chosen,
                scope: self.selection_scope,
            },
        );
    }
    #[must_use]
    pub fn choice_was_possible(&self, kind: &Kind) -> bool {
        self.selection_evidence
            .iter()
            .any(|e| e.scope == self.selection_scope && e.chosen && kind.lte(&e.kind))
    }
    #[must_use]
    pub fn subset_was_possible(&self, kind: &Kind) -> bool {
        self.selection_evidence.iter().any(|e| {
            e.scope == self.selection_scope
                && kind.lte(&e.kind)
                && e.source_group.is_some_and(|id| {
                    self.is_visible(id) && self.find_by_id(id).is_some_and(|r| group(kind, r))
                })
        })
    }
    pub fn enter_selection_scope(&mut self) {
        self.selection_scope = self.next_selection_scope;
        self.next_selection_scope += 1;
    }
    pub fn leave_selection_scope(&mut self, before: &Self) {
        let scope = self.selection_scope;
        self.selection_scope = before.selection_scope;
        self.next_selection_scope = self.next_selection_scope.max(before.next_selection_scope);
        self.selection_evidence.retain(|e| e.scope != scope);
    }
    #[must_use]
    pub fn parts_taken(&self, kind: &Kind) -> u64 {
        self.visible_referents()
            .filter(|r| r.det == Determiner::Part && kind.lte(&r.payload.kind()))
            .map(|r| r.payload.size().unwrap_or(1))
            .sum()
    }
    #[must_use]
    pub fn counted_group_size(&self, kind: &Kind) -> Option<u64> {
        prefer_ordinary(
            whole_groups(&self.visible_referents().collect::<Vec<_>>())
                .into_iter()
                .filter(|r| r.det != Determiner::Part && group(kind, r))
                .collect(),
        )
        .first()
        .and_then(|r| r.payload.size())
    }
    #[must_use]
    pub fn distributive_pair(&self, kind: &Kind) -> Option<&Referent> {
        let member = self
            .distributive_subject
            .filter(|id| self.is_visible(*id))
            .and_then(|id| self.find_by_id(id))?;
        let whole = self.find_by_id(member.member_of?)?;
        let key = whole.group_key?;
        (self.is_visible(whole.id)
            && !whole.linked
            && group(kind, whole)
            && self
                .referents
                .iter()
                .filter(|r| r.part_of == Some(key) && r.plur == Plurality::One)
                .count()
                == 2)
            .then_some(whole)
    }
    #[must_use]
    pub fn rest_fits(&self, kind: &Kind, plurality: &Plurality) -> bool {
        match plurality {
            Plurality::One => {
                self.distributive_pair(kind).is_some()
                    || (self.count_groups(kind) <= 1
                        && self.count_parts(kind) != 0
                        && self
                            .counted_group_size(kind)
                            .is_some_and(|n| n == self.parts_taken(kind) + 1))
            }
            Plurality::Many => {
                self.count_groups(kind) <= 1
                    && (self.count_parts(kind) != 0 || self.subset_was_possible(kind))
            }
        }
    }
    #[must_use]
    pub fn rest_source(&self, kind: &Kind) -> Option<&Referent> {
        let refs: Vec<_> = self.visible_referents().collect();
        refs.into_iter().rev().fold(None, |rest, r| {
            if r.linked {
                rest
            } else if r.det == Determiner::Part {
                rest.or_else(|| kind.lte(&r.payload.kind()).then_some(r))
            } else if group(kind, r) {
                Some(r)
            } else {
                rest
            }
        })
    }
    #[must_use]
    pub fn rest_group(&self, kind: &Kind, plurality: &Plurality) -> Option<&Referent> {
        (if *plurality == Plurality::One { self.distributive_pair(kind) } else { None })
            .or_else(|| self.rest_source(kind))
    }
    #[must_use]
    pub fn rest_kind(&self, plurality: &Plurality) -> Option<Kind> {
        let distributive = if *plurality == Plurality::One {
            self.distributive_subject
                .filter(|id| self.is_visible(*id))
                .and_then(|id| self.find_by_id(id))
                .and_then(|r| r.member_of)
                .and_then(|id| self.find_by_id(id))
                .map(|r| r.payload.kind())
        } else {
            None
        };
        distributive
            .or_else(|| {
                self.visible_referents()
                    .find(|r| !r.linked && r.det == Determiner::Part)
                    .map(|r| r.payload.kind())
            })
            .or_else(|| {
                self.selection_evidence
                    .iter()
                    .find(|e| e.scope == self.selection_scope)
                    .map(|e| *e.kind.clone())
            })
    }
    pub fn consume_rest(&mut self, before: &Self, kind: &Kind) {
        let ids: Vec<_> = before
            .visible_referents()
            .filter(|r| r.det == Determiner::Part && kind.lte(&r.payload.kind()))
            .map(|r| r.id)
            .collect();
        for r in &mut self.referents {
            if ids.contains(&r.id) {
                r.det = Determiner::The;
            }
        }
        let source = before.rest_source(kind).map(|r| r.id);
        self.selection_evidence.retain(|e| {
            !(e.scope == before.selection_scope
                && kind.lte(&e.kind)
                && e.source_group.is_some()
                && e.source_group == source)
        });
    }
}

impl DRS {
    #[must_use]
    pub fn coordinated_self_in(&self, ids: &[ReferentId]) -> Option<ReferentId> {
        ids.iter()
            .copied()
            .find(|id| self.find_by_id(*id).is_some_and(|r| r.default_anchor))
            .or(self.coordinated_self)
    }
    pub fn push_coordinated_group(
        &mut self,
        start: ReferentId,
        members: &[ReferentId],
        role: super::GramRole,
        in_arg_slot: bool,
    ) -> Vec<ReferentId> {
        let payloads: Vec<_> = members
            .iter()
            .filter_map(|id| self.find_by_id(*id))
            .map(|r| *r.payload.clone())
            .collect();
        let Some(payload) = super::Payload::group_of(&payloads) else {
            return Vec::new();
        };
        let group_id = self.next_id;
        let fresh: Vec<_> = members.iter().copied().filter(|id| *id >= start).collect();
        let mut sources = Vec::new();
        for r in fresh.iter().filter_map(|id| self.find_by_id(*id)) {
            if !sources.contains(&r.part_of) {
                sources.push(r.part_of);
            }
        }
        let part_of = if sources.len() == 1 { sources[0] } else { None };
        for r in &mut self.referents {
            if fresh.contains(&r.id) && r.part_of.is_none() {
                r.part_of = Some(group_id);
            }
        }
        let mut group = Referent::new(Determiner::The, Plurality::Many, payload);
        group.group_key = Some(group_id);
        group.part_of = part_of;
        self.prepend(vec![group], role, in_arg_slot)
    }
}
