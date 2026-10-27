//! Referents shared by alternative event and condition headers.
use super::DRS;
use super::Referent;

impl Referent {
    #[must_use]
    pub fn header_corresponds(&self, other: &Self) -> bool {
        fn normalized(value: &Referent) -> Referent {
            let mut value = value.clone();
            value.id = 0;
            value.clause = 0;
            value.pronoun_resolved = false;
            value.idents.clear();
            value
        }
        normalized(self) == normalized(other)
    }
    #[must_use]
    pub fn header_description_corresponds(&self, other: &Self) -> bool {
        let mut other = other.clone();
        other.role = self.role;
        self.header_corresponds(&other)
    }
}

impl DRS {
    fn header_fresh<'a>(&self, after: &'a Self) -> Vec<&'a Referent> {
        after
            .visible_referents()
            .filter(|r| self.find_by_id(r.id).is_none())
            .collect()
    }

    #[must_use]
    pub fn common_header_context(&self, branches: &[Self]) -> Self {
        let Some((primary, alternatives)) = branches.split_first() else {
            return self.clone();
        };
        if alternatives.is_empty() {
            return primary.clone();
        }
        let fresh = self.header_fresh(primary);
        let mut retained: Vec<_> = fresh
            .iter()
            .filter_map(|reference| {
                let unique_here = fresh
                    .iter()
                    .filter(|other| reference.header_corresponds(other))
                    .count()
                    == 1;
                let correspondences: Vec<Vec<_>> = alternatives
                    .iter()
                    .map(|branch| {
                        let branch_fresh = self.header_fresh(branch);
                        let same_role: Vec<_> = branch_fresh
                            .iter()
                            .copied()
                            .filter(|other| reference.header_corresponds(other))
                            .collect();
                        if !same_role.is_empty() {
                            same_role
                        } else if fresh
                            .iter()
                            .filter(|other| reference.header_description_corresponds(other))
                            .count()
                            == 1
                        {
                            branch_fresh
                                .into_iter()
                                .filter(|other| reference.header_description_corresponds(other))
                                .collect()
                        } else {
                            Vec::new()
                        }
                    })
                    .collect();
                if unique_here && correspondences.iter().all(|rs| rs.len() == 1) {
                    let mut reference = (*reference).clone();
                    reference.idents.retain(|ident| {
                        correspondences
                            .iter()
                            .all(|rs| rs.iter().all(|r| r.idents.contains(ident)))
                    });
                    Some(reference)
                } else {
                    None
                }
            })
            .collect();
        retained.extend(self.referents.iter().filter_map(|original| {
            let refined = primary.find_by_id(original.id)?;
            if alternatives
                .iter()
                .all(|branch| branch.find_by_id(original.id).is_some())
            {
                Some(
                    if alternatives.iter().all(|branch| {
                        branch
                            .find_by_id(original.id)
                            .is_some_and(|r| refined.header_corresponds(r))
                    }) {
                        refined
                    } else {
                        original
                    }
                    .clone(),
                )
            } else {
                None
            }
        }));
        let mut out = primary.clone();
        out.pending_roles
            .retain(|(id, _)| retained.iter().any(|r| r.id == *id));
        out.resolved_ids
            .retain(|id| retained.iter().any(|r| r.id == *id));
        out.referents = retained;
        out.selection_evidence.retain(|evidence| {
            alternatives
                .iter()
                .all(|branch| branch.selection_evidence.contains(evidence))
        });
        out.next_selection_scope = branches
            .iter()
            .fold(self.next_selection_scope, |n, branch| {
                n.max(branch.next_selection_scope)
            });
        out.next_id = branches
            .iter()
            .fold(self.next_id, |n, branch| n.max(branch.next_id));
        out.performers.clone_from(&self.performers);
        out
    }
}
