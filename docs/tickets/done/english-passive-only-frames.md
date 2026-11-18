---
needs: []
---
# Passive-only verb frames, non-extractable indirect objects, and recipient-passive *deal*

## Why

`crates/vocabulary/lexicon/english/verbs.ron` gives `deal` `Ditransitive` in active voice solely so `valence` will derive the recipient passive (*Whenever this creature is dealt damage*, *Each creature dealt damage this way*). Active ditransitive *deal* occurs on 0 Corpus Units in the Oracle register, and deriving active Productions from `deal`'s `Ditransitive` frame adds spurious Readings across 86 Corpus Units: divided-damage spells (*Arc Lightning*: `deals [3 damage … among [one, two, or three]]_{Oi} [targets]_{Od}`), temporal and manner Adjuncts (*Case of the Burning Masks*, *Aggravate*: `dealt [damage]_{Oi} [this turn / this way]_{Od}`), and relative clauses (*Azorius Ploy*: `gap0` and `gap1` alongside monotransitive `deal`). CGEL recognizes verbs restricted to the passive with a particular complementation alongside verbs restricted to the active (CGEL, Ch. 16, §10.1.2, p. 1432 [17] and p. 1435 [31]). Independently, `valence_family` in `crates/syntax/src/compile.rs` derives an `IndirectObject` (`O^i`) Gap Production (`gap0`) for every active `Ditransitive` frame (*give*, *offer*), even though resistance to prenuclear/relativized position is the primary syntactic property distinguishing `O^i` from `O^d` (CGEL, Ch. 4, §4.3(d), pp. 248–250 [9]).

## Design

- Replace `Frame::active_only: bool` in `sylex_syntax` with a three-way voice restriction (`ActiveOrPassive`, `ActiveOnly`, `PassiveOnly`, defaulting to `ActiveOrPassive`). Reject a `PassiveOnly` frame in `Frame::is_well_formed` when its family and slots cannot passivize.
- In `sylex_syntax` compilation, emit active `valence` Productions only when the frame's voice is `ActiveOrPassive` or `ActiveOnly`, emit passive Productions (verbal, adjectival, and prepositional) only when it is `ActiveOrPassive` or `PassiveOnly`, format `frame_label` suffixes `[ActiveOnly]` and `[PassiveOnly]`, and combine `unlike_frames` voices by intersection (emitting no unlike frame when `ActiveOnly` meets `PassiveOnly`).
- Exclude `SlotRelation::IndirectObject` from `extractable` in `valence_family` (`crates/syntax/src/compile.rs`, alongside the existing objective-predicative exclusion), so active `Ditransitive` frames derive a Gap for `Object` (`O^d`) only, while recipient-passive `Ditransitive` frames keep their `Object` Gap.
- Extend `sylex_vocabulary`'s verb-frame schema so an individual frame can be marked `PassiveOnly` (and `ActiveOnly`), and change `deal` in `crates/vocabulary/lexicon/english/verbs.ron` to declare its `Ditransitive` frame `PassiveOnly`.
- Update `docs/design/english.md` §3 and `docs/contexts/oracle-english/CONTEXT.md` (`Passive-only Frame` beside `Active-only Frame`) to state both voice restrictions.
- Add unit tests in `crates/syntax/tests/` for `PassiveOnly` frames and for the absence of `IndirectObject` extraction on active `Ditransitive` frames, plus Judgment assertions in `crates/english/judgments/` pinning the removal of active ditransitive `deal` on *Arc Lightning* and *Azorius Ploy* while keeping recipient-passive *is dealt damage*.

## Done when

- `Frame` in `crates/syntax/src/frames.rs` carries the `ActiveOrPassive | ActiveOnly | PassiveOnly` voice restriction in place of `active_only: bool`, active `Ditransitive` frames derive no `IndirectObject` Gap Production, and `deal`'s `Ditransitive` frame in `crates/vocabulary/lexicon/english/verbs.ron` is `PassiveOnly`.
- `docs/design/english.md` §3 and `docs/contexts/oracle-english/CONTEXT.md` document `Passive-only Frame` alongside `Active-only Frame`.
- `cargo xtask english strata --check` against a pre-change baseline shows 0 lost faces, 0 verdict regressions, and 0 Corpus Units with increased Readings, with *Arc Lightning*, *Case of the Burning Masks*, and *Azorius Ploy* losing their active `Ditransitive` `deal` Readings and recipient-passive *deal* Corpus Units preserved.
- `cargo test` and `cargo clippy --tests -- -D warnings` pass for `sylex_syntax`, `sylex_vocabulary`, `sylex_english`, and `xtask`.

## Pointers

- `crates/syntax/src/frames.rs` (`Frame`)
- `crates/syntax/src/compile.rs` (`valence`, `valence_family`, `unlike_frames`, `frame_label`, `facts.active_only`)
- `crates/syntax/tests/active_only_frames.rs`
- `crates/vocabulary/src/schema.rs`, `crates/vocabulary/src/build.rs`, `crates/vocabulary/lexicon/english/verbs.ron` (`deal`, `cost`), `crates/vocabulary/lexicon/english/closed_class.ron` (`have`)
- `crates/english/src/profile.rs`
- `docs/design/english.md` (§3)
- `docs/contexts/oracle-english/CONTEXT.md` (`Active-only Frame`)
