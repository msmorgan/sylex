---
needs: []
---

# Engine: a batch whose members each need a replacement choice loses no events, and the replacement choice offers only the first non-empty [CR#616.1] tier

## Why

Lost events (reachable). When a member of a batch suspends for a replacement choice,
`surface_choice` (`crates/engine/src/replace_registry.rs`) stores no remainder. If a later
member of that remainder suspends again in `resume_replacements`, the function returns: the
rest of the batch is dropped, and so are the facts of the member just resolved, which are
emitted only after the loop. Members that pass are scheduled one by one as
`Occurrence::Single`, in reverse order, and the earlier members commit as a separate partial
batch. One event can hold several occurrences [CR#603.2c], so splitting the batch changes what
a "whenever one or more" trigger sees. Floating shields are keyed per instance
(`ReplacementKey::Floating`), so two creatures that each hold two regeneration shields,
destroyed in one state-based batch, hit this.

No tiers (latent). `replace_event` offers every applicable replacement. [CR#616.1a..616.1d]
require a self-replacement effect, then a control-on-entry one, then an enter-as-copy one,
then a back-face-up one to be chosen first, and only then may any applicable effect be chosen
[CR#616.1e]. None of those kinds exists yet.

## Done when

- The suspended replacement state owns the unprocessed tail, the facts already resolved and the
  members that passed. A suspend in the middle of the tail carries all of it forward, and when
  the last choice resolves every member commits as one `Occurrence::Batch`, in member order.
- Before a replacement choice is surfaced, the applicable set is narrowed to the first non-empty
  tier of [CR#616.1a..616.1e]. Each replacement kind carries its tier as that kind lands.
- Tests: two creatures, each with two regeneration shields, are destroyed by one destroy-all;
  both choices are surfaced, both creatures regenerate, and a "whenever one or more creatures …"
  trigger sees one occurrence. A self-replacement and an ordinary replacement both apply to one
  event, and only the self-replacement is offered.

## Not in scope

- The APNAP order of simultaneous choosers [CR#616.1]. Note it where the choice is surfaced;
  ticket it if it needs work.

## Pointers

- `crates/engine/src/replace_registry.rs` (`replace_event`, `surface_choice`,
  `resume_replacements`)
- `crates/engine/src/step/mod.rs` (`apply_occurrence`'s suspend handling)
- `crates/engine/src/state.rs` (`ReplaceState`)
- `crates/engine/src/sba.rs` (the state-based batch)
