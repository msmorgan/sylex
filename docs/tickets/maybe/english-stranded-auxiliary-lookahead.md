---
needs: []
---
# A stranded auxiliary is licensed by how the next word is spelled

## Why

`StrandedModal`, `StrandedPerfect` and `StrandedBe` let a finite auxiliary head its Verb Phrase
alone before the ellipsis site of its Complement (*If you can't, ...*, *each opponent who
can't*), through the `elliptical` property. The property refuses the Production when the word
that follows has a lexical head spelled like a verb in the omitted Complement's form, since the
following material would then be the Complement (*cards that weren't [cast this way]*). The
intent is same-head complement priority (ADR §3). The test is on spelling, not on whether a
Complement analysis of the following material exists, and it lives in the engine as a property
of its own.

Two readings of this exist. In one, it is complement priority approximated by a lookahead, in an
engine that ADR §1 says names no grammar term and so cannot grow a second grammar. In the
other, checking only that the next word cannot head the omitted Complement is the right
treatment of ellipsis after an auxiliary: it stops ellipsis before a real Complement and keeps
it before a finite predicate (*each opponent who can't discards a card*).

## Open questions

- Can the chart decide ellipsis by the absence of a Complement analysis of the following
  material, and if so should `elliptical` become that?

## Done when

- A string where the spelling test and a Complement-analysis test would differ is found on the
  supported corpus, or its absence is recorded. If there is none, the ticket closes with no
  change.
- If there is one, the stranded auxiliary's Reading there is the right one, and the census
  shows no Reading lost on the supported corpus.

## Pointers

- `crates/english/grammar/english.fug`: `StrandedModal`, `StrandedPerfect`, `StrandedBe`,
  `class StrandableModal`, `class StrandableHave`, `class StrandableBe`.
- `crates/syntax/src/grammar.rs` (`elliptical`).
- `docs/decisions/english.md` §1, §3.
- Local evidence (gitignored): `docs/evidence/english-fug-hax-check/fable.md` (F13),
  `docs/evidence/english-fug-hax-check/gemini.md` (§2, Cleared).
