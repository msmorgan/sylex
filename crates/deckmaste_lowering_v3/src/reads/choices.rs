//! Context retained by embedded text, from `Semantics/Check/Choices.lean`.
use super::DRS;
use super::Payload;
impl DRS {
    #[must_use]
    pub fn choice_context(&self) -> Self {
        Self {
            referents: self
                .visible_referents()
                .filter(|r| r.payload.is_choice())
                .cloned()
                .collect(),
            next_id: self.next_id,
            current_clause: self.current_clause,
            ..Self::default()
        }
    }
    #[must_use]
    pub fn linked_text_context(&self) -> Self {
        let current =
            self.visible_referents()
                .filter_map(|referent| match referent.payload.as_ref() {
                    Payload::Quality { .. } | Payload::Player { chosen: true } => {
                        Some(referent.clone())
                    }
                    Payload::Object {
                        zone: Some(_),
                        prov: Some(_),
                        ..
                    } => {
                        let acted = self.grantor_linked.as_ref().is_none_or(|grantor| {
                            referent.id < grantor.since
                                && (referent.linked
                                    || grantor
                                        .before
                                        .iter()
                                        .find(|r| r.id == referent.id)
                                        .is_none_or(|r| {
                                            r.payload.provenance() != referent.payload.provenance()
                                        }))
                        });
                        if !acted {
                            return None;
                        }
                        let mut result = referent.clone();
                        if !result.linked {
                            result.linked = true;
                            result.link_site = self.next_id;
                            result.link_arms.clear();
                        }
                        Some(result)
                    }
                    _ => None,
                });
        let published = self
            .grantor_linked
            .as_ref()
            .map_or(&[][..], |g| g.published.as_slice());
        let mut grantor: Vec<_> = published
            .iter()
            .enumerate()
            .map(|(index, referent)| {
                let mut result = referent.clone();
                result.id = self.next_id + 1 + index as u64;
                result.linked = true;
                result.link_site = self.next_id + 1;
                result
            })
            .collect();
        grantor.reverse();
        grantor.extend(current);
        Self {
            referents: grantor,
            next_id: if published.is_empty() {
                self.next_id
            } else {
                self.next_id + 1 + published.len() as u64
            },
            ..self.choice_context()
        }
    }
}
