---
needs: []
---

# Nominal helpers at the wrong sort, and Agreement checks that skip

## Why

Several helper declarations under the nominal sorts spell a phrase at a sort the English grammar
does not give it, or spell words no caller prints:

- `thatIsBoth(left: PostModifier, right: PostModifier)` and `postModifier/hasColor` exist for
  *Battlegate Mimic*'s "a spell that's both red and white". The parser reads "red" and "white" as
  predicative adjectives coordinated under a copular relative clause
  (`cargo xtask english forestwalk --card 'Battlegate Mimic' --line 1`); a color is never a
  postmodifier. *Fry*'s "target creature or planeswalker that's white or blue" is the same
  construction with "or", and for want of it is written
  `nominal(pre = [or([white, blue])], or([creature, planeswalker]))`, which prints the colors
  before the head.
- `predicative/youControl` and `predicative/controlledBy` spell "you control" and "$who control",
  so `is(thisArtifact, youControl)` (*Helm of Possession*, `echo`, `suspend`) spells "this artifact
  be you control". `templates/condition/controls.syl` spells the same body as a clause.
- `nominal/abilityOf` spells "ability of $source". Its one canon caller, *Gaea's Revenge*, prints
  "abilities from nongreen sources", as do `protection` and `hexproofFrom`
  [CR#702.16b,702.11d]. *Arrest* is cited for `ofSource` but prints "its activated abilities".

`crates/xtask/src/dsl_check/nominal.rs` decides whether an Agreement `PostModifier` stands in a
plural group by matching constructor names and falling through catch-all arms, so a case it does
not know is silently accepted or refused: `targets(upToOf(1), nominal(creature, post =
[withDifferentNames]))` is accepted, because `quantity_is_at_most_one` knows only `Range` and
`ExactlyOf`. `check_nominal_call` returns without an error when the signature, an argument or
`head` is missing, `elaborate` returns `None` through its `?`s and so skips a declaration's whole
Agreement check, and `forEach` is recognized by Template name. A Template whose body is a `nominal`
may stand in `nominal.head` (`nominal(pre = [copied], abilityOf(it))`), so "a nominal has one tree"
(`spelling.md` §2) holds only for a literal nested call.

Two hand-written terms have meanings the design does not settle. *Tithe Taker* and *Suppression
Field* write the mana-ability exemption as `onlyUnless(costShift(…), is(them, noun(a(manaAbility))))`:
`them` has no binder in a cost shift, `semantics.md` §3.3 reads an Unless Condition per participant
only inside a deontic rule's box, and the term prints "a mana ability" for "mana abilities".
*Guardian Beast* writes "can't be enchanted" as `cant(any, attachTo(a(aura), them))`, where `aura`
at `Nominal` is an Aura on the battlefield and an Aura entering attached [CR#303.4f] is not yet
there.

`docs/evidence/syl-nominal-descriptions/number-snapshot-diff.txt` lists 47 flagged lines, some
losing an anaphor (*Dread Slaver*, *Infernal Vessel*), and nothing explains them.

## Open questions

- What is the term for a post-head copular relative, "that's both red and white" and "that's
  white or blue": a `PostModifier` taking a `Predicative`, with colors and `or` at `Predicative`,
  or something else?
- Does a cost shift's exemption ("unless they're mana abilities") read per ability, and through
  which construct?
- Does a head type word in `attachTo`'s object cover an Aura entering attached, as entering
  permanents are read at the event's point in time (`semantics.md` §3.1)?

## Done when

- The open questions are answered, each answer written where it governs.
- *Battlegate Mimic* and *Fry* print their Oracle words in printed order, at the sorts the parse
  gives them; `thatIsBoth` and `postModifier/hasColor` are gone or at the right sort.
- No `Predicative` spells a clause: `is(…, youControl)` and its `controlledBy` form are respelled
  so each caller prints grammatical text, or the declarations move to the sort they are.
- *Gaea's Revenge*, `protection` and `hexproofFrom` print "from"; *Arrest* prints "its activated
  abilities"; every remaining declaration among `abilityOf` and `ofSource` has a caller that
  prints its spelling.
- *Tithe Taker*, *Suppression Field* and *Guardian Beast* mean what their Oracle text means under
  the answers, and print its words.
- In `nominal.rs`, every quantity, determiner and slice-count form is classified by an exhaustive
  match or refused by name: no catch-all arm, no early return and no `?` skips a check, and no
  Template is recognized by its name. A test covers each form, `upToOf(1)` among them.
- A nominal reached through a Template's body in `nominal.head` is refused or the one-tree rule
  in `spelling.md` §2 is restated as what the check enforces.
- Every flagged line of `number-snapshot-diff.txt` is explained in the evidence summary, and any
  that is a lost reference is fixed.
- The declarations the nominal design names carry a doc comment saying what they mean: the
  `preModifier/` status and designation Templates (`tapped`, `untapped`, `faceDown`, `phasedOut`,
  `foretold`, `plotted`), `non`, `or`, `andOr`, and each `predicative/` Template.
- `predicative/noun` and `semantics.md` §2.4 state the copula's type-alone reading as the design's
  choice, not as [CR#109.2]'s; `nominal/card.syl` and `preModifier/was.syl` cite rules that support
  their comments.
- `docs/specs/confers-and-rules.md`, `docs/tickets/planned/dsl-fmt-bundle-normalization.md`,
  `syl-opponent-noun-and-predicate.md` and `syl-this-head-template.md` describe scopes and head
  words as they now are.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check`, `cargo test -p xtask` and
  `cargo clippy -p xtask --tests -- -D warnings` pass.

## Not in scope

- Printed "and" spelled `or` in premodifier and nominal coordinations, reversed coordination
  order, and unprinted location words: `syl-canon-wording-defects`, `syl-zone-words`.
- Retyping the `Predicate` consumers (`enchant`, spend purposes, keyword qualities) to `Nominal`.

## Pointers

- `plugins/builtin/templates/postModifier/thatIsBoth.syl`,
  `plugins/builtin/primitives/postModifier/hasColor.syl`, `plugins/builtin/templates/predicative/`,
  `plugins/builtin/templates/nominal/abilityOf.syl`, `plugins/builtin/templates/condition/controls.syl`
- `crates/xtask/src/dsl_check/nominal.rs`; `crates/xtask/tests/suite/dsl_check.rs`
  (`agreement_postmodifier_requires_a_plural_group_determiner`)
- `crates/english/grammar/english.fug` (the relative-clause and Predicative productions);
  `crates/english/README.md`, "Surveying a unit's forest"
- `docs/design/spelling.md` §2, `docs/design/semantics.md` §2.4, §3.1, §3.3
- `docs/evidence/syl-nominal-descriptions/`
