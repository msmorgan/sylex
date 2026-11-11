---
needs: []
---
# A word flag and two flat clauses for one specifying sentence

## Why

`feature Variable = Cited`, held only by *type* (`variable: true` in the lexicon), lets
`SpecifyingClause` and `SpecifyingModalClause` take a subtype name as the value of a copular
clause (*the new creature type can't be Wall*, Artificial Evolution, the only card with the
shape). The comment cites CGEL Ch. 4 §5.5, pp. 266–269. In CGEL, variable and value are roles
of any specifying copular clause, with no lexical condition on the Subject's noun; a per-noun
flag with one holder is what ADR §3 rules out. Both Productions are flat clauses that bypass the
Verb Phrase, so no adjunct, coordination of values or other auxiliary reaches them; the modal
one borrows `class StrandableModal`, and the value is `Class(Subtype)` rather than the `Mention`
Category for a word cited rather than used.

## Open questions

- Route the value through the copula's Predicative as a `Mention`, find what keeps that from
  adding a Reading to every *is red*, or keep the two Productions with a comment that names
  Artificial Evolution?

## Done when

- CGEL pp. 266–269 is re-read and whether it puts any lexical condition on the variable is
  recorded; if it does, the ticket closes with the comments corrected.
- `Variable` and *type*'s `variable: true` are gone, or the comment says they exist for one
  card.
- Artificial Evolution keeps a correct Reading, and the census shows no Reading lost on the
  supported corpus and no unit growing past what the chosen route's ablation reports.

## Not in scope

- Other one-card Productions: `english-single-card-productions`.

## Pointers

- `crates/english/grammar/english.fug`: `feature Variable`, `SpecifyingClause`,
  `SpecifyingModalClause`, `class StrandableModal`, `category Mention`, `category NounPhrase`,
  `category BareNominal`.
- `crates/vocabulary/lexicon/english/nouns.ron` (*type*).
- `docs/decisions/english.md` §3 ("No policing").
- Local evidence (gitignored): `docs/evidence/english-fug-hax-check/fable.md` (F4),
  `docs/evidence/english-fug-hax-check/gemini.md` (C1).
