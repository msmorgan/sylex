---
needs: []
---
# Plural *each of* and the Case it sets to stay a Subject

## Why

`SynesisDistributivePartitive` gives *each of* a plural oblique plural agreement (*Each of X
target creatures you control connive*, Change of Plans), citing CGEL Ch. 5 §18.2, p. 504 for
synesis. Whether that page covers *each* is disputed: one reading finds *each* absent from
§18.2, the fused heads that vary in CGEL being *any*, *none*, *either* and *neither* (§18.3(f)),
and every *each of them* example singular; the other reads p. 504 as covering plural *each of*.
The Production also sets `$.Case = Nominative` on a Noun Phrase with no case, which keeps it out
of Complement positions (`slot NP`, `NonNumericComplement`) and appears to let it host a clitic
in `CliticSubjectPredicate` (*each of those creatures're*); and `complement.Case := Plain`
appears to refuse *each of them connive*. `AnyPartitive` handles *any of* with a plural oblique
by leaving Number unvalued.

## Design

- *Each of X target creatures you control connive* (Change of Plans) is an agreement error in the
  text: it reads through an Editorial Correction to *connives*, and `SynesisDistributivePartitive`
  is deleted, so fused *each* with a plural oblique agrees in the singular only.

## Done when

- CGEL pp. 501–507 is re-read and whether it covers plural agreement with *each of* is recorded.
- `SynesisDistributivePartitive` is deleted, and no *each of* Subject has a Reading with a
  plural verb or as a clitic host through a `Case` value.
- Change of Plans reads through an Editorial Correction, and the census shows no other verdict
  change on the supported corpus.

## Not in scope

- The ban on fused *each* in Complements: `english-fused-each-partitive`.

## Pointers

- `crates/english/grammar/english.fug`: `SynesisDistributivePartitive`,
  `DistributivePartitive`, `AnyPartitive`, `CliticSubjectPredicate`, `slot NP`,
  `NonNumericComplement`.
- `crates/english/corrections.txt`, `docs/contexts/oracle-english/CONTEXT.md` (Editorial
  Correction).
- Local evidence (gitignored): `docs/evidence/english-fug-hax-check/fable.md` (F6),
  `docs/evidence/english-fug-hax-check/gemini.md` (C4).
