---
needs: []
---

# A temporal clause agrees in time sphere with the clause it modifies

## Why

*Whenever you cast a noncreature spell, choose one —* (*Kykar, Zephyr Awakener*) has two
Readings that differ only in *cast* as present or preterite, and no judgment can tell them
apart. About a thousand units that begin with *Whenever* carry the same fork on a syncretic
*cast* or *put*, and units with *when* and *until* carry it too. The preterite Reading is not
English there. A temporal Adjunct locates its host's situation at the time of its own clause,
so the two share a time sphere: *Whenever he visited, we cooked*, *Whenever he visits, offer
him tea*, never *Whenever he visited, we cook* or *Don't go until I was feeling better*. The
grammar lets a preposition's finite Clause Complement be of either tense whatever it modifies.

## Design

- A Preposition Phrase headed by a temporal preposition with a finite Clause Complement, as an
  Adjunct of a Predicate or Clause, agrees with its host in time sphere: a preterite Complement
  under a preterite host (*the life you lost when it entered*), and no preterite Complement
  under a present-tense or imperative host. An imperative's verb is a plain form, tenseless,
  whose situation is later than the utterance (CGEL, Ch. 3, §7, p. 159), so it is non-past with
  the present tense.
- The temporal prepositions are those whose clause places the host's situation in time:
  *whenever* in its "on any occasion when" use (CGEL, Ch. 8, §14.6.2, p. 764; Ch. 12, §6.4,
  p. 1076), *when*, *until*, *before*, *after*, *while* and *once* (CGEL, Ch. 3, §4.2.5,
  p. 135, lists temporal constructions apart from conditional ones).
- A conditional preposition's clause is independent of its host in time (CGEL, Ch. 8, §14.1.2,
  p. 743) and stays unrestricted: *if*, *unless*, and *as long as*, which Oracle uses as a
  condition on past events (*as long as a creature died this turn*).
- The same phrase as the modifier of a Nominal has no host tense to agree with and stays
  unrestricted (*the player's life total before the damage was dealt*).
- The header comment of `prepositions.ron`, which says a preterite Complement of *whenever* is
  left for semantics to reject, is replaced by this rule.

## Open questions

- What the rule says under a host that is neither present nor preterite: a past participle
  (*cards exiled when it entered*, if the corpus has such a case), a gerund-participle, an
  infinitive, an irrealis *were*. Options: such a host takes its time sphere from the finite
  clause it sits in; it is unrestricted; or it admits only a non-preterite Complement.
- How the agreement is stated in `.fug` and what it costs. A Preposition Phrase carries none of
  its Complement's `Inflection` today, so the check needs that fact on the phrase (a feature on
  a Category every unit builds, widening chart summaries) or a separate Category for temporal
  clausal phrases. The answer needs the measured change in chart size and parse time on the
  pool for each.
- Whether any preposition on the temporal list has, in the corpus, a preterite Complement under
  a non-past host that is real English (a use the list above misjudges). Each such case names
  the preposition's other use and moves it, or that use, off the list.

## Done when

- *Kykar, Zephyr Awakener*'s second line has one Reading, and no unit of the pool has a Reading
  with a preterite Complement of a temporal preposition under a present-tense or imperative
  host.
- *Soulgorger Orgg*'s *the life you lost when it entered* keeps the Reading in which *when it
  entered* modifies *lost*, and has none in which it modifies *gain*.
- Judgments pin each of: a preterite temporal clause under a preterite host (grammatical), under
  a present-tense host and under an imperative host (both ungrammatical), a preterite
  conditional clause under an imperative host (grammatical), and a preterite temporal clause
  modifying a Nominal (grammatical).
- The landing reports every unit whose Reading count or census verdict changed, by preposition,
  and the chart size and parse time on the pool before and after.
- `cargo xtask english strata --check` passes, and `cargo test` and
  `cargo clippy --tests -- -D warnings` pass for each changed crate.

## Not in scope

- Tense agreement between a main clause and a content clause or relative clause (backshift).
- The "no matter when" use of *whenever*, an exhaustive conditional, which no unit is known to
  use.

## Pointers

- `crates/vocabulary/lexicon/english/prepositions.ron` (header comment; the `FiniteClause`
  frames).
- `crates/english/grammar/english.fug` (`Inflection`, `PrepositionPhrase`, `Clause`,
  `VerbPhrase`, the `adjunct PrepositionPhrase` schemas, `FiniteComplementClause`).
- `docs/decisions/english.md` §3, §4, §6.
