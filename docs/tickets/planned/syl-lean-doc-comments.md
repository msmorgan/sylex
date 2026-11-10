---
needs: [cleanup-step6-spine, syl-misnamed-declarations]
---

# Doc comments in `.syl` words, not Lean's

## Why

Many doc comments under `plugins/` still speak Lean:

- Lean terms: `keyword_action/amass.syl` (`that (.type .creature)`, "resolved inside `ownScope`")
  and `keyword_action/resolve.syl` (`GameEvent.causes (.event (does .any (resolve thisSpell)))
  <move>`, `WouldClause.causedBy (.event …) <clause>`).
- Lean constructor paths: 54 primitives whose whole comment is "Authoring primitive for
  `Predicate.…`" or "Full-arity binding for `….…`", and about 25 other comment lines that name a
  constructor as `Sort.ctor` (`Action.instruct`, `DeonticMods.rider`, `Definitions.wither`,
  `DRS.distributivePair?`, …), some of them in canon cards. A primitive's constructor clause is
  the one place the AST's names appear (`docs/decisions/syl.md` §1).
- Generator-inserted mentions: `templates/action/controllerSacrifices.syl`,
  `templates/action/lookAndSortSlice.syl` and `templates/duration/duringNextTurnOf.syl` say they
  use `firstMention` / `laterMention`, which `.syl` cannot write; expansion decides first mentions
  (`syl.md` §4).

- Repeated citations: 206 comment lines are a bare citation that repeats only rules the same
  comment already cites, as `templates/turn_point/endOfCombat.syl`'s `# [CR#511.2]` under
  "abilities that trigger "at end of combat" trigger as the end of combat step begins
  [CR#511.2]".

A reader is pointed at names and syntax that exist in no current source.

`syl-misnamed-declarations` renames `modifiesPrecedingAction`, which
`templates/action/doUnderProhibition.syl`'s comment names as `Duration.modifiesPrecedingAction`,
and lands first.

## Done when

- No doc comment under `plugins/`, apart from the two below, writes a Lean term, names a
  constructor as `Sort.ctor`, or names `firstMention`, `laterMention` or another construct `.syl`
  cannot write. Each such comment says
  the same thing in `.syl` declaration names and printed words, or, where it said nothing the
  declaration does not already state (a bare "Authoring primitive for …"), is deleted.
- No comment line is a bare citation of rules its comment already cites; a bare citation line
  that is a comment's only citation of a rule is folded into the sentence it supports.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass;
  `cargo xtask cite check --list-noncompliant` is empty and `cargo xtask cite check` reports
  0 stale.

## Not in scope

- `abilityHead`'s comment, which `syl-doc-comment-rationales` rewrites to state its rationale.
- `theRingTemptsYou.syl`'s `Action.ownScope` primitive and its comment, which
  `syl-ring-tempts-you-raw-scope` deletes.

## Pointers

`docs/decisions/syl.md` §1 and §4.
