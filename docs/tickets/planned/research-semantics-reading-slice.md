---
needs: [cleanup-step6-surface, syl-shared-phrase-written-once, syl-replacement-stated-as-clause]
---

# Try the four-step reading on cards chosen to break it

## Why

`docs/design/semantics.md` §6 says how a surface term is read into its meaning: classify each
clause, open its boxes, resolve its references, build the meaning. Nothing has run it. Its §8
already lists what the steps as written leave unsettled (items 11 to 13), and taking the
sorts that say what a clause means out of the term language makes several names meet at one sort
(item 7). Building `sylex_semantics` on that design untested commits to it, and removing the
scaffolding sorts from the corpus first commits harder. A slice that is thrown away tests both
on the real declarations once the surface gate, the shared phrase and the clause-form
replacements have landed.

## Design

- The slice answers two questions and no others:
  1. Do the four steps, in the order given, read each card on the list into the meaning its
     rules text has? Where a step fails, what does it need that the steps before it do not
     give?
  2. For each set of names that meet at one sort, are its members one word or do they need
     names that tell them apart?
- It rewrites no corpus. A card on the list gets its surface term written by hand in the slice's
  folder, in the form `docs/design/syl.md` §6 gives with the scaffolding sorts gone, with
  the term the card means and each known wrong one: the wide and the narrow attachment, the
  passive and the copular "is tapped".
- Its code is thrown away. It lives under the gitignored
  `docs/evidence/research-semantics-reading-slice/` and may call or copy `cargo xtask dsl check`
  and `dsl expand`. Its findings note is `findings.md` in that folder, and what lands is the
  design-doc edits the findings force.
- What the slice does not test it does not decide. Crate boundaries, the sorts beyond the names
  that meet, the pragmatics data and recovery get the crudest stub that lets the cards be read,
  and the findings list each stub as not decided. How a replacement's two clauses are held
  (`semantics.md` §8 item 2) is one of them: a replacement is written both ways.

## Done when

- The findings note's first section states, for each question, the results that would change
  `semantics.md` §6 or §8.
- These cards are read, each checked against its Oracle text: *Tide of War*, *Carom*,
  *Worship*, *Illusionary Mask*, *Cursed Windbreaker*, *Soulflayer*, *Urza's Armor*,
  *Deflecting Palm*, *Dandân*, *Mana Leak*, *Contamination*, *Bounding Krasis*, *Centaur
  Omenreader*, *Broken Wings*, *Elven Riders* and *Detection Tower*, and the cards under
  "Events and their participants" in `docs/unusual-cards.md`.
- For each card the findings give its hand-written terms, what each step did with each term,
  and the outcome of §6: read, refused with the rule that refused it, or not understood. For
  each wrong term they say whether a step refused it or it would reach pragmatics.
- For each set of names that meets at one sort (`doIf`, `replace`, `nthOccurrence`, the ten
  `or` declarations and any other the audit of `plugins/builtin/` finds), the findings give
  each member's wording and say whether the set is one word or needs distinct names.
- `semantics.md` §6 and §8 and `syl.md` §6 are edited from the findings, each edit naming the
  card that forced it, and an item of §8 the slice answers leaves §8.
- The findings list every stub and what it leaves undecided.
- No file under `crates/` or `plugins/` is changed by the landing.

## Not in scope

- Recovery and how many candidates a card has: `english-syl-term-recovery`.
- Building `sylex_surface` or `sylex_semantics`, and taking the scaffolding sorts out of the
  corpus: they follow this ticket's findings, with the Rust structures of step 7.
- Which default a trailing modifier over a coordination gets: `docs/design/pragmatics.md` §4.

## Pointers

- `docs/design/semantics.md` §2, §6, §7, §8; `docs/design/syl.md` §6;
  `docs/design/architecture.md` ("The layers"); `docs/design/pragmatics.md`
- `docs/specs/voice-and-object-phrases.md` §3 to §5, §8
- `docs/unusual-cards.md` ("Events and their participants")
- `crates/xtask/src/dsl_check.rs`, `crates/xtask/src/dsl_expand.rs`
