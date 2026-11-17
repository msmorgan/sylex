---
needs: []
---

# Nominal helpers at the wrong sort, and Agreement checks that skip

## Why

Several helper declarations under the nominal sorts spell a phrase at a sort the English grammar
does not give it, or spell words no caller prints:

- `thatIsBoth(left: PostModifier, right: PostModifier)` and `postModifier/hasColor` exist for
  *Battlegate Mimic*'s "a spell that's both red and white". The parser reads "red" and "white" as
  predicative adjectives coordinated under a copular subject relative
  (`cargo xtask english forestwalk --card 'Battlegate Mimic' --line 1`); a color is never a
  postmodifier. *Fry*'s "target creature or planeswalker that's white or blue" is the same
  construction with "or" and prints its colors before the head; *Crippling Fear*'s "that aren't of
  the chosen type" and *Benevolent Blessing*'s "that are already attached to it" drop the relative.
- `predicative/youControl` and `predicative/controlledBy` spell "you control" and "$who control",
  so `is(thisArtifact, youControl)` (*Helm of Possession*, `echo`, `suspend`, `soulbond`) spells
  "this artifact be you control".
- `nominal/abilityOf` spells "ability of $source", where *Gaea's Revenge* prints "abilities from
  nongreen sources", protection's rule "abilities from a source with the stated quality"
  [CR#702.16b] and hexproof from's "abilities your opponents control from [quality] sources"
  [CR#702.11d]. *Arrest*, *Demotion*, *Viper's Kiss*, *Power Artifact*, *Stupefying Touch*,
  *Ghostfire Blade*, `detain`, `haste` and `creature` print a possessive ("its activated
  abilities", "This Equipment's equip ability") through `ofSource`.
- *Tithe Taker* and *Suppression Field* write "unless they're mana abilities" as `onlyUnless(…,
  is(them, noun(a(manaAbility))))`: `them` has no binder in a cost shift, and the term prints "a
  mana ability"; *Pithing Needle* and *Damping Matrix* print that too. *Guardian Beast* writes
  "can't be enchanted" as `cant(any, attachTo(a(aura), them))`, whose `aura` is an Aura on the
  battlefield, and prints neither "be" nor "enchanted".

`crates/xtask/src/dsl_check/nominal.rs` decides whether an Agreement `PostModifier` stands in a
plural group by matching constructor names and falling through catch-all arms, so
`targets(upToOf(1), nominal(creature, post = [withDifferentNames]))` is accepted.
`check_nominal_call` returns without an error when a resolution is missing, `elaborate` skips a
declaration's whole Agreement check through its `?`s, `forEach` and `stacked` are recognized by
Template name, and a Definition's body is never checked with its callers' arguments. The one-tree
refusal sees only a literal nested `nominal`, so *Saga*'s `nominal(chapterAbilityOf(thisSaga),
post = [triggeredNotLeftStack])` has two trees.

`docs/evidence/syl-nominal-descriptions/number-snapshot-diff.txt` lists 47 flagged lines that
nothing explains.

## Design

- A copular relative after the head is `thatIs(predicative: Predicative): PostModifier`
  ("that be $predicative"). A color stands at `Predicative` through a `Color` coercion
  (`hasColor`), and a `Predicative` coordinates with `and`, `or`, `andOr` and the two-armed
  correlative `both`; `ofChosen` and `attachedTo` stand at `Predicative` too.
- `costShift` takes `unless: Option(Condition)`, read inside the shift once per member of its
  subject, as a deontic rule's Unless Condition is (`semantics.md` §3.3).
- A bare plural as the copula's complement ("they're mana abilities") is `bare(p)`, over the
  `bare` determiner, which states no quantificational force; under the copula it tests each member
  of its subject as `a` tests one (`semantics.md` §2.4).
- "Can't be enchanted / equipped / fortified [by …]" is `cantBeAttached(host, word, by)`, a
  standing limit on what the host can be enchanted by [CR#303.4], or equipped or fortified by
  [CR#702.16d]. Host legality is one relation: a `mayBeAttachedTo` permission less any
  `cantBeAttached` limit, read by `legalHostOf`, `legalAttachment`, `legallyAttached`, entry,
  attaching and the attachment state-based actions (`semantics.md` §3.3). Protection's enchant and
  equip/fortify clauses use it [CR#702.16c,702.16d].
- "Its <abilities>" is `possessive(owner, p)` over a `possessive(owner)` determiner, which takes
  every member that belongs to its owner, an ability belonging to its source [CR#113.7].
- A chapter ability is the `AbilityClass` `chapter` [CR#715.2], so *Saga* and read ahead write
  "chapter ability of …" as one nominal.
- `forEach` and `countOf` expand to one tree, so the Agreement check treats them alike. An
  aggregate, a superlative or a comparison ranges over its whole domain, as "all" does, so its
  domain forms a group for Agreement.
- `dsl check` runs the nominal rules only on bundles every earlier rule accepted, on terms with
  every Template expanded and every Definition's body checked with each call's arguments; a
  determiner, plurality, slice count, quantity or coordinator it does not classify is refused by
  name.

## Done when

- The answers above are written where they govern: `syl.md` §2, `spelling.md` §2,
  `semantics.md` §2.4 and §3.3, and the declarations' doc comments.
- *Battlegate Mimic*, *Fry*, *Crippling Fear*, *Benevolent Blessing* and the Vintage-legal
  *Glistening Deluge* write their copular relatives through `thatIs`; `thatIsBoth` and
  `postModifier/hasColor` are gone, and every new declaration has a canon caller.
- No `Predicative` spells a clause other than `did` ("that $subj $action"), which
  `syl-lookback-operator-and-holds` owns: `predicative/youControl` is gone,
  `predicative/controlledBy` spells "controlled by $who", and its callers write `controls` or the
  participle. `predicative/was` spells a participle ("cast from your hand"), not a clause.
- *Gaea's Revenge*, `protection` and `hexproofFrom` print "from" through `fromSource`; `abilityOf`
  is gone; *Arrest*, *Demotion*, *Viper's Kiss*, *Power Artifact*, *Stupefying Touch*, *Ghostfire
  Blade*, `detain`, `haste` and `creature` write `possessive`; every remaining `ofSource` caller
  prints "of"; *Saga* and read ahead write `abilityHead(chapter)`, and `chapterAbilityOf` is
  gone.
- *Tithe Taker* and *Suppression Field* write `costShift(…, unless = is(them, noun(bare(…))))`;
  *Pithing Needle* and *Damping Matrix* write `noun(bare(manaAbility))`; *Guardian Beast* and `protection` write
  `cantBeAttached`; the Aura, Equipment and Fortification state-based actions test
  `legallyAttached`.
- `nominal.rs` classifies every determiner, plurality, slice-count, quantity and coordinator form
  by an exhaustive match or refuses it by name, with no `?`, early return or Template name test; a
  test covers each form, `upToOf(1)` among them; a nominal reached through a Template's body or
  parameter in `nominal.head` is refused; a Definition's body is checked at each call.
- Every flagged line of `number-snapshot-diff.txt` is explained in
  `docs/evidence/syl-nominal-descriptions/summary.md`; none is a lost referent.
- The `preModifier/` status and designation Templates, `non`, `or`, `andOr` and each
  `predicative/` Template carry a doc comment; `predicative/noun` and `semantics.md` §2.4 state the
  copula's type-alone reading as the design's choice; `nominal/card.syl` and `preModifier/was.syl`
  cite rules that support their comments.
- `docs/specs/confers-and-rules.md`, `dsl-fmt-bundle-normalization.md`,
  `syl-opponent-noun-and-predicate.md` and `syl-this-head-template.md` describe scopes, coercions
  and head words as they now are.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check`, `cargo test -p xtask`,
  `cargo clippy -p xtask --tests -- -D warnings`, `cargo test -p sylex_syl` and the citation checks
  pass, and the `syl-number-snapshot` diff flags nothing.

## Not in scope

- Printed "and" spelled `or` in premodifier and nominal coordinations and reversed coordination
  order: `syl-canon-wording-defects`. Unprinted location words: `syl-zone-words`.
- Copular relatives inside created tokens, becomes and damage ("a token that's a copy of …",
  "that's still a planeswalker", "damage that would be dealt"): `syl-token-records-to-templates`,
  `syl-become-replaces-types`, `syl-damage-and-life-noun-phrases`. "Already":
  `syl-already-attached`.
- Retyping the `Predicate` consumers (`enchant`, spend purposes, keyword qualities) to `Nominal`.
- The Predicative `did`, which spells a relative clause after the copula until lookbacks are one
  operator: `syl-lookback-operator-and-holds`.

## Pointers

- `plugins/builtin/primitives/postModifier/thatIs.syl`, `plugins/builtin/templates/predicative/`,
  `plugins/builtin/primitives/static/{costShift,cantBeAttached}.syl`,
  `plugins/builtin/templates/determiner/{bare,possessive}.syl`
- `crates/xtask/src/dsl_check/nominal.rs`; `crates/xtask/tests/suite/dsl_check.rs`
- `docs/design/spelling.md` §2, `docs/design/semantics.md` §2.4, §3.3
- `docs/evidence/syl-nominal-descriptions/`, `docs/evidence/syl-nominal-modifier-sorts-and-checks/`
