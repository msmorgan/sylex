---
needs: []
---
# Restricting a Category to some of another's Productions

## Why

The `.fug` language cannot say "this Category is that one minus some of its Productions", so a
restricted Category restates the Productions it keeps, and each copy must be changed with its
twin by hand; nothing checks that they agree. `SubjectInitialClause` (a Clause that opens with
its Subject, for bare relatives) restates five Productions, each flagged `// Twin:` or listed
above `RelativeSubjectPredicate`:

- `SubjectPredicate` / `RelativeSubjectPredicate`
- `CliticSubjectPredicate` / `RelativeCliticSubjectPredicate`
- `FloatedEach` / `RelativeFloatedEach`
- `FloatedAll` / `RelativeFloatedAll`
- `AdjunctGap` / `RelativeAdjunctGap`

Other Categories say in their comments that they restate Productions without naming twins:
`category Possessor` (the Noun Phrase Productions over `BareNominal`) and
`category RelativeTemporalNominal` (`RelativeTime` and `RelativeTimePostmodifier` restating
`NominalPostmodifier` and `TemporalPostmodifier`). `TemporalNounPhrase` and
`TargetedNounPhrase` may be further cases.

## Open questions

- Should the DSL gain a way to state a restriction once (for example, a Category that excludes
  named Productions, or equations over engine summaries such as whether a Clause is
  Subject-initial), or should the twins stay with a check that they agree?

## Done when

- Every restating Category in `english.fug` is listed with its twins, including whether
  `TemporalNounPhrase` and `TargetedNounPhrase` are such cases.
- The owner's ruling is carried out: either each restriction is stated once and the twins are
  deleted, or a check fails the build when a pair of twins disagrees beyond their stated
  difference.
- The census shows identical Readings before and after.

## Pointers

- `crates/english/grammar/english.fug`: the Productions above, `category SubjectInitialClause`,
  `category Possessor`, `category RelativeTemporalNominal`, `category TemporalNounPhrase`,
  `category TargetedNounPhrase`.
- `crates/syntax/src/fug.rs`, `crates/syntax/src/grammar.rs`.
- Local evidence (gitignored): `docs/evidence/english-fug-hax-check/fable.md` (§2 Cleared, §3),
  `docs/evidence/english-fug-hax-check/gemini.md` (§2 Cleared, §3).
