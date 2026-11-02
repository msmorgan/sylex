---
needs: [plugins-v3-builtin-primitives-and-templates, lean-generated-canon-modules, semantics-rust-rewrite]
---
# `toCast` over a noun phrase, and Ice Cauldron's noted mana

Standard constraints apply. Design ticket, design proposed below. Found by the 2026-10-25
triage of an external audit ("Atraxa"), verified against `wmyykqnv`. Two parts, each separately
committable; neither depends on the other. The checker half of the original Part 1 (a cast
permission publishing "that spell", and respelling *Rogue Class* and *Summon: Esper
Valigarmanda* with it) is `discourse-cast-permission-publishes-spell`; nothing here needs it.

AST types are in Rust `semantics` (today `lean/Semantics/Abilities.lean`); primitives and
Templates are `.syl` under `plugins_v3/builtin/`. "Fragment" below means a Lean proof-suite
fragment carried to a Rust test by `semantics-rust-rewrite`.

## Part 1: `toCast` takes a NounPhrase

`SpendPurpose.toCast (predicate : Predicate)` (primitive
`plugins_v3/builtin/primitives/mana/toCast.syl`) cannot take an anaphor, so "to cast those
spells" / "to cast that spell" cannot be written.

Change it to `SpendPurpose.toCast (spells : NounPhrase)`, and the primitive's parameter to
`spells: NounPhrase`. Every current site writes a description; rewrite each to `bare <its
predicate>` (the generic plural "to cast creature spells"), keeping the predicate. Sites today:
`plugins_v3/canon/cards/Ancient Ziggurat.syl`, `Mishra's Workshop.syl`, `Unblinking
Observer.syl`, and the fragments in `lean/Semantics/Proofs/` `CardsMana.lean`, `CardsChoice.lean`,
`CardFidelity.lean`, `Lookback.lean` and `Rules.lean` (grep `toCast`; re-grep the `.syl` corpus at
claim time). `discourse` steps `toCast`'s argument as a NounPhrase in the caller's context
instead of a Predicate at kind `.object`.

## Part 2: Ice Cauldron

### The card

*Ice Cauldron* (fragment `iceCauldronNotedMana`, `lean/Semantics/Proofs/CardsMana.lean`): "{X},
{T}: You may exile a nonland card from your hand. You may cast that card for as long as it remains
exiled. Put a charge counter on this artifact and note the type and amount of mana spent to pay
this activation cost. Activate only if there are no charge counters on this artifact. / {T},
Remove a charge counter from this artifact: Add this artifact's last noted type and amount of
mana. Spend this mana only to cast the last card exiled with this artifact."

The fragment's spelling, `addMana 1 (ofTheLastNotedType thisArtifact) [spendOnly [toCast
exiledWithThisArtifact]]`, has three defects: the amount is a literal 1, "the last" is dropped,
and the first ability (which writes the note) is not authored. The two abilities are linked
[CR#607.2e,607.2a].

### Design

1. **Noting.** No action records information today (checked 2026-10-25: no "note" or
   "remember" action). Add `Action.noteManaSpent`, no arguments, with a primitive
   `noteManaSpent: Action` under `plugins_v3/builtin/primitives/action/`: "note the type and
   amount of mana spent to pay this activation cost" [CR#607.2e]. It is a linked-ability writer;
   `discourse` treats it as an action that introduces no referent, and its readers are the
   existing `ProducedMana.lastNoted subject` (primitive
   `plugins_v3/builtin/primitives/mana/ofTheLastNotedType.syl`) and item 2's amount. Add it to
   the event-head list only if a card printing "whenever … note" is found (none was).
2. **The noted amount.** Add `Amount.lastNoted (subject : NounPhrase)` beside
   `ProducedMana.lastNoted`, with primitive `theLastNotedAmountOf(subject: NounPhrase): Amount`.
   It reads the amount the linked note recorded, as `ProducedMana.lastNoted` reads its type.
3. **"The last card exiled with this artifact."** No recency selector exists: `Quantity.theLast`
   is "the removal that leaves none of its kind" for counters, and `Predicate.superlative`
   (`plugins_v3/builtin/primitives/predicate/superlative.syl`) ranges over a `ProjAxis` with no
   time axis. An object receives a timestamp when it enters a zone [CR#613.7d]. Add
   `ProjAxis.zoneEntry`, the timestamp at which the object entered its current zone, and write
   "the last <p>" as `the (and [p, superlative .max .zoneEntry p])`; it composes with the
   existing `superlative`.
4. **Spell the whole card** as `plugins_v3/canon/cards/Ice Cauldron.syl`, replacing the fragment:
   the first ability (exile up to one nonland card from your hand, the cast permission "for as
   long as it remains exiled", `putCounters 1 chargeCounter thisArtifact`, the note, and
   "Activate only if there are no charge counters on this artifact"); the second
   `addMana (theLastNotedAmountOf thisArtifact) (ofTheLastNotedType thisArtifact) [spendOnly
   [toCast (the (and [isCard, in_ (exiledWith thisArtifact), superlative .max .zoneEntry (and
   [isCard, in_ (exiledWith thisArtifact)])]))]]` (with `toCast` over a NounPhrase if Part 1 has
   landed, else over the predicate). If a step of this design does not check, STOP and report the
   refusal rather than reaching for a raw constructor.

## Tests

- Part 1: the tests over each `toCast` site, rewritten (same claim, new spelling).
- Part 2: the fragment test `okIceCauldronNotedMana` is retired, replaced by the spelled card
  (its claim, that the mana ability checks, is kept inside the card). Added: the second
  ability's amount is `Amount.lastNoted thisArtifact`; the new axis is accepted under
  `superlative` at kind `.object` and refused at `.player`.

## Acceptance (each part)

`cargo test` and `cargo clippy --tests -- -D warnings` for `semantics` and `discourse`, the CR
citation gates (bless newly cited rules). Glossary: name any new public item against
`docs/contexts/game-model/CONTEXT.md` first and add an entry through `domain-modeling` if it names
a new concept.
