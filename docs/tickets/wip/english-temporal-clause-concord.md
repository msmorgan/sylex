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

- Time is two features of a Clause or Verb Phrase, past and non-past, each carried or not.
  Neither is the verb's `Inflection`. A present-tense verb carries non-past and a preterite
  carries past. A modal's preterite (*would*, *could*, *might*) carries neither: it is
  ambiguous between past time and modal remoteness, remoteness being much the more common
  (CGEL, Ch. 3, §2.4, pp. 106–107), so the form fixes no time, and the lexicon says so by
  giving it no tense. A plain form, a participle and irrealis *were* carry neither
  (CGEL, Ch. 3, §7, pp. 160–162). The feature-bundle mapping table carries the lexicon's tense
  into the grammar as these two features (`docs/decisions/english.md` §1).
- An imperative Clause is non-past: its verb is tenseless and its situation is later than the
  utterance (CGEL, Ch. 3, §7, p. 159).
- A phrase takes time from its dependents as well as from its head. A temporal Preposition
  Phrase carries the time of its clause, and gives it to the phrase it is an Adjunct of. A
  modal's bare-infinitival Complement, whose situation is simultaneous with the modal's time
  or later (CGEL, Ch. 3, §7, p. 160), gives its time to the modal's phrase the same way, and
  so does the Complement of passive or progressive *be*, which is not temporally distinct
  from it (CGEL, Ch. 14, §4.2.2, p. 1217). Perfect *have* does
  not: the perfect is itself a past tense, its participle's situation before the time of
  *have* (CGEL, Ch. 3, §5, pp. 139–140), and a present perfect takes a past-time Adjunct in
  its experiential use (pp. 143–144). A plain-form or participial Verb Phrase,
  which carries no time of its own, so takes the time of the temporal clause inside it (in
  *draw a card whenever you cast a spell*, the Verb Phrase is as *cast* is).
- A temporal clause locates its host's situation at the time of its own (CGEL, Ch. 8, §6,
  p. 700), so no head carries one time where a dependent that gives it time carries the other.
  This holds in both directions: no past clause under a non-past host (*Whenever you
  cast[preterite] a spell, draw a card*; *… you may draw a card*) and no non-past clause under
  a past host (*Whenever you cast a spell, they put[preterite] a counter on it*). A side that
  carries neither agrees with either (*If you would draw a card while your library has no
  cards in it*).
- The rule is stated once in the grammar and holds wherever a time-giving dependent meets a
  head: every Adjunct position of a Predicate host (after a Verb Phrase, before or after a
  Clause, beside another front dependent, over a coordination of Clauses, interleaved before a
  later Complement, in a Complement Cluster or an Adjunct Cluster), and the Complement of a
  modal or of passive or progressive *be*. No attachment site states it for itself.
- A coordination carries each time that any conjunct carries, so the rule is distributive: a
  dependent of a coordination agrees with every conjunct, and every conjunct of a coordinated
  dependent agrees with the head, as a modifier of a coordination applies to each coordinate
  (CGEL, Ch. 15, §1.3.4, p. 1286). A past clause is no Adjunct of *[creatures you control get
  +1/+1] and [gained vigilance]*, and *Whenever you cast or copy a spell* under a non-past host
  has no preterite *cast*.
- The temporal prepositions are those whose clause places the host's situation in time:
  *whenever* in its "on any occasion when" use (CGEL, Ch. 8, §14.6.2, p. 764; Ch. 12, §6.4,
  p. 1076), *when*, *until*, *before*, *after*, *while* and *as* (*as you cast this spell*,
  *as it entered*). That a preposition's clause is temporal is declared once, with the frame
  that takes the clause, as what the frame selects; no flag or feature names the words
  (`docs/decisions/english.md` §3).
- A conditional preposition's clause is independent of its host in time (CGEL, Ch. 8, §14.1.2,
  p. 743) and gives no time: *if*, *unless*, *as long as*, *as though*. A Nominal carries no
  time, so a temporal clause that modifies one is unrestricted (*the player's life total
  before the damage was dealt*).
- The header comment of `prepositions.ron`, which leaves a preterite Complement of *whenever*
  for semantics to reject, is replaced by this rule.

## Done when

- No unit of the pool has a Reading in which a temporal clause and its host differ in time, a
  coordinated host or clause included, at any attachment site.
- These have no preterite *cast*: *Kykar, Zephyr Awakener*'s second line; *Ral, Storm
  Conduit*'s "Whenever you cast or copy" line; *Pyromancer
  Ascension*'s second line; and a unit whose *whenever* clause modifies a present-tense modal
  host (*Whenever you cast a spell, you may …*).
- *Jeska's Will*'s first line has no preterite *cast* where *as you cast this spell* modifies
  *control*. Temporal *as* also modifies a Nominal (*that player's life total as the turn
  began*, *Sengir, the Dark Baron*), which carries no time, so the Reading in which the clause
  modifies *a commander* keeps both tenses.
- *Soulgorger Orgg*'s *the life you lost when it entered* keeps the Reading in which *when it
  entered* modifies *lost*, and has none in which it modifies *gain*. *Minion of the Wastes*'s
  *the life paid as it entered* keeps the Reading in which *as it entered* modifies *paid*,
  and has none in which it modifies *are*.
- *Laboratory Maniac*'s *would draw a card while your library has no cards in it* keeps its
  Reading, and *Bold Plagiarist* has no Reading with a preterite *put* over its present-tense
  *whenever* clause.
- Judgments pin every clause of the rule, each on a real or cited construction, so that
  removing any one statement of it from the grammar fails a judgment: a past temporal clause
  under a past host (grammatical); under a present-tense host, a present-tense modal host, an
  imperative host, a host inside a modal's Complement, a passive auxiliary's host and a
  coordinated host (each ungrammatical); a non-past temporal clause under a past host, fronted
  and after the verb (ungrammatical); a temporal clause under a modal preterite, and a modal
  preterite inside a temporal clause (grammatical); a coordinated temporal clause one of whose
  conjuncts differs from the host (ungrammatical); the post-clausal comma position, the
  existential host and a Complement Cluster's Adjunct (each with an ungrammatical case); a
  preterite conditional clause under an imperative host (grammatical); and a preterite
  temporal clause modifying a Nominal (grammatical).
- No unit loses its last Reading and no census verdict changes for the worse; the landing
  lists every unit whose Readings changed, by preposition and by which clause of the rule
  removed them, and any unit where a removed Reading was real English.
- `cargo xtask english strata --check` passes, and `cargo test` and
  `cargo clippy --tests -- -D warnings` pass for each changed crate.

## Not in scope

- Tense agreement between a main clause and a content clause or relative clause (backshift).
- The "no matter when" use of *whenever*, an exhaustive conditional, which no unit is known to
  use.

## Pointers

- `crates/vocabulary/lexicon/english/prepositions.ron` (header comment; the `FiniteClause`
  frames).
- `crates/english/grammar/english.fug`: the `slot` declarations (a frame's Complement kind names
  one, and a phrase's `ComplementKind` is that name); `valence PrepositionPhrase` (`agree`
  gives the phrase its Complement's features); the `adjunct PrepositionPhrase` schemas with a
  Predicate host and the Productions with a `Hosted(PrepositionPhrase, Predicate)` child;
  `Imperative`; the auxiliary Productions (`AuxiliaryComplement`) and the `BareInf` slot; the
  `coordination VerbPhrase` and `coordination Clause` schemas (a feature of one value that the
  conjuncts share is carried by the coordination when any conjunct carries it).
- `crates/syntax/src/compile.rs`: `slot_equations` (equations the grammar states once per Frame
  Slot kind), `interleaved_dependent`, `complement_clusters` and `adjunct_clusters` (Productions
  the engine derives, which no authored equation reaches).
- The lexicon's verb forms carry `tense` (`crates/lexicon/src/dimensions.rs`) and the
  feature-bundle mapping table carries lexicon features into the grammar.
- `docs/decisions/english.md` §1, §3, §6.
