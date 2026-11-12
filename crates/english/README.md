# sylex_english

The Oracle English grammar
([`docs/decisions/english.md`](../../docs/decisions/english.md)): the
grammar data file [`grammar/english.fug`](grammar/english.fug), written in the
`.fug` language that `sylex_syntax::fug` describes, and its binding into
`sylex_syntax`.

The grammar names Categories, features, Schemas and core closed-class lexemes
only. Vocabulary and Subcategorization Frames come from the lexical environment
the caller binds it with: the seed in `tests/seed/` for the seed proof, and
`sylex_vocabulary`'s core lexicon and Catalogs for the strata.
`tests/catalog_leak.rs` checks that the grammar names only core closed-class
lexemes of that lexicon and that none of its own names spells a catalog item.

## Working rules

- **Census before and after.** Every change to the grammar, the engine, the
  lexicon or the census runs the census before and after it:

  ```
  cargo xtask english strata --output target/english/before.json   # before
  cargo xtask english strata --check                               # after
  ```

  No face is lost, and every unit whose verdict or Readings change is
  reported.
- **Canonicalizations.** A change that introduces or tightens a
  canonicalization ([`docs/decisions/english.md`](../../docs/decisions/english.md)
  §8) compares, per unit, the set of Reading Normal Forms before and after,
  under exactly that equivalence and nothing wider: the analyses stay the same
  while the number of trees falls.
- **Judgments.** `cargo test -p sylex_english --test judgments` runs the
  judgment set. How an entry is written and adjudicated, by two independent
  judges, is in [`judgments/README.md`](judgments/README.md).
- **Reported, never gated.** Where the endorsed Reading ranks, the cheapest
  tier, judgment tallies and forest size (`derivations`, `total_derivations`,
  Surplus) are reported. None is a pass condition, and no change is made to
  move one.
- **A gate's ablation.** Before adding or retiring a gate
  ([`docs/decisions/english.md`](../../docs/decisions/english.md) §6), remove
  it, run the census, and compare with the census that has it: the spurious
  Readings and units the forest gains without it, and the largest units'
  growth. The comparison is reported with the change that adds or retires
  the gate and is not kept; the gate's comment in
  [`grammar/english.fug`](grammar/english.fug) names one real card and the
  spurious structure the gate prevents on it, as the Preposition Phrase
  valence's `closed_complement` comment does. This holds for every gate
  labeled "admission gate" in the grammar, and for any other gate when a
  change adds, retires or edits it.

## Seed proof

`tests/seed_proof.rs` proves the frame engine end to end on a seed of about
fifteen open-class words: active complementation, same-head complement priority
(`docs/decisions/english.md` §3), Host-Category-limited attachment with
cross-head ambiguity retained (including Seedborn Muse), passives with and
without `by`, object and stranded-preposition Gaps across finite, progressive,
perfect and passive forms, and Right Node Raising.
Every Reading is admitted and realizes to its input; hand-rebuilt Readings reparse.
`chart_and_forest_sizes` prints chart items, forest nodes, families and Readings
per seed sentence.

## Corpus and units

`corpus` reads the supported corpus (Vintage legal or restricted, outside
`set_type: funny`) from `data/scryfall/oracle-cards.jsonl` through
`sylex_upstream`, which reads the snapshot once per process, one `Face` per
Card Face record with its own name in every form in lexical scope
(`corpus::own_names`, a function of the face's name alone). Each face yields
Corpus Units: its Type Line, and each paragraph line of its rules text after
balanced reminder text is removed
(`sylex_upstream::oracle_text::strip_reminder_text`). The split needs no parse.

## Complexity Strata

`profile` assigns each unit its Complexity Stratum
(`docs/contexts/oracle-english/CONTEXT.md`). A unit is read as its longest
lexical analyses; its Syntactic Skeleton
replaces each open-class catalog item with a placeholder (`<Subtype>`,
`<KeywordAction>`, ...). Its Syntactic Families are recognized from lexical
categories, word forms and declared frames: an outer spine (labels, modal
headers, bullets, `Cost : Effect`, keyword lines) is Stratum 0, and each family
inside belongs to its Stratum (an auxiliary to 2, a bare relative to 3, a
preposition with a clause complement to 4, a coordinator to 6, *instead* or a
quoted ability to 7, ...). The unit takes the highest (Max-Stratum Composition
Rule). The rules are a partition for planning and gating, not a grammar: a
Stratum ticket that finds a unit misplaced fixes the family rule, and every unit
like it moves with it. [`STRATA.md`](STRATA.md) says what each Stratum is and
what it brought into the grammar.

## Census

`census::run` profiles and parses every unit and reports it **read** (a Reading
exists and round-trip law 1 holds over the whole Packed Forest),
**known-wrong-only** (a judgment for that exact text is satisfied by none of
its Readings) or **unread** (with the reason: the grammar
does not bind, a vocabulary gap, no Reading, or a law failure, which is a
defect). Counts are per Stratum, with faces counted at their highest unit, plus
per Syntactic Family and the most frequent skeletons.
The census, judgment evaluator and `english parse` command consume Readings
one at a time through the syntax engine's lazy Huang & Chiang (2005),
Algorithm 3 enumeration over a flyweight arena. The Parse APIs borrow their
Parse and return iterators:

- `best_readings(k) -> impl Iterator<Item = Reading> + '_`
- `best_ranked(k) -> impl Iterator<Item = (RankingKey, Reading)> + '_`
- `readings() -> impl Iterator<Item = Reading> + '_`

Intermediate candidates share tree and child-sequence handles; owned Readings,
including their feature maps and Selected Frames, are materialized only by
`.next()`. Consumers needing indexing or slices use `.collect()`.

The census does not enumerate Readings to check law 2: each Reading already
belongs to the parse of the input, and law 1 establishes that its realization
is that input. Under the same parse context and lexical environment, membership
in the parse of its realization follows. Independent-tree law 2 tests remain
in the test suite. The census materializes Readings only for judgments.

Each unit's count is reported two ways. `derivations` is the size of its Packed
Forest, the number of trees: the census's no-Reading verdict and the judgment
evaluator's 4096-derivation limit use it, and `total_derivations` sums it. It
multiplies, so a unit of several sentences, each a little ambiguous, can count
in the thousands. The unit is therefore also split into its **Segments**
(`OUTER_CATEGORIES`; the walk is `segments::segments`): the walk descends from the root through the line, its
top-level sentence sequence, activation costs, labels, modes and modal headers,
and a Keyword Line's items and parameters, and each first Constituent of another
Category (always a `Sentence`), or leaf of an outer Production, is a Segment,
identified by its extent without the structural punctuation at its edges. A
sentence sequence nested in a Segment, as in a quoted ability, stays in it. Each
unit reports its Segments with their Derivations and its **Surplus**,
Σ(nᵢ − 1) over them, which is 0 exactly when the unit is unambiguous; the
report sums it in `total_surplus` and counts ambiguous units and Segments. A
unit with a Derivation whose Segments differ between Derivations, or do not
multiply to its count, is not segmentable (its Surplus is `derivations − 1`);
the summary lists it and `--check` fails on it.

Each unit also reports its `cheapest_tier`: how many of its Derivations share
the least Ranking Key (`docs/decisions/english.md` §5), before the final
tie-break, counted on the Packed Forest without building trees. The summary
gives how many ambiguous units have a cheapest tier of one, the tiers' sum over
ambiguous units, and the largest tier. Ranking never changes which Readings
exist: verdicts, Readings and Surplus do not depend on it. It is reported, never
gated (see Working rules).

`COMPLETED_STRATUM` is the highest Stratum whose gate is met: every unit at or
below it that is not deferred is read, so both round-trip laws hold and no
known-wrong-only unit counts. It advances only then. A unit at or below
it that is not read is a regression, unless [`deferred.txt`](deferred.txt) names
it by card and text: a deferred unit is left unread on purpose, owned by a
ticket, and counted apart. A law failure is never deferred, and a deferred unit
that reads is stale, as is a deferred line that names no unit of the
whole snapshot (with or without `--card`).

Where a face's Oracle text is in error, the census reads the Editorial
Correction recorded in [`corrections.txt`](corrections.txt) instead, and the
grammar is not relaxed for the error. The file's header says what counts as
one. Each correction names the card, the Oracle text, the corrected text and
its evidence. A correction whose Oracle text
no longer occurs exactly once in its face's text is stale and fails `--check`.

```
cargo xtask english strata [--card NAME]... [--check]
```

prints the summary and writes the full per-unit report to
`target/english/strata.json` (untracked). `--check` fails on a regression, a
stale deferral (read, or naming no unit) or a unit that is not segmentable.

## Inspecting a parse

```sh
cargo xtask english parse --limit 2 'Destroy target creature.' '+NounPhrase target creature => NounPhrase "target creature" head creature' '*PrepositionPhrase target creature'
cargo xtask english parse --as-card 'Seedborn Muse' 'Untap all permanents you control.'
cargo xtask english parse --card 'Delver of Secrets // Insectile Aberration'
```

There are three ways to give it text:

- **Text alone.** Quote each input separately. Inputs use the
  [judgment-entry syntax](judgments/README.md), including category roots and
  assertions. Bare Oracle text is parsed as a positive entry at the grammar
  root. Balanced reminder text is stripped.
- **`--as-card NAME` with text.** Each input is parsed as if it were on the
  card so named: the name is an input like the text, and no card is fetched. A
  name that is no card name is refused. An input in entry syntax may state its
  card itself with `<card=NAME>` instead; giving both is an error.
- **`--card NAME` with no text.** The faces so named (a face's name, or its
  whole card's) are fetched from the supported corpus and their own Corpus
  Units parsed: under a `Face: NAME (card: CARD)` line, the face's type line
  and then each rules-text line, numbered from 1, each at the grammar root
  with that face's own name in scope. A bracketed line of a cleave face is
  parsed as printed; the census reads it as its two texts.

Each argument must contain one entry.

The command prints the profiled Stratum and Syntactic Families, vocabulary gaps,
the Packed Forest's total derivation count, the Segments with their
Derivations, and the Surplus. Explicit `+`/`*` entries and
inputs with assertions also print the judgment Outcome. Outcomes use the full
judgment evaluator, including its round-trip checks and 4096-derivation limit;
`--limit` only limits the display, not assertion evaluation.

Up to eight cheapest admitted Readings are shown by default. `--limit 0` prints
only the summary. The summary also gives the cheapest tier: how many
Derivations share the least Ranking Key, and that key. Each Reading includes
its realized text, its Ranking Key (`class/association/cost`, in comparison
order) and
a constituent table: preorder node index (root 0), parent, Production id,
Category, byte span and surface in the realized text, grammatical function,
head lemma, lexeme id and Word Form, and Gap. The Word Form tells apart
Readings that differ only in a syncretic head (*cast* as present or preterite,
*Merfolk* as singular or plural). Judgment failures and unknown Categories are
reported without stopping later inputs; malformed entries and operational
errors fail the command. The dump is diagnostic output, not tracked judgment
evidence.

## Judgment set

[`judgments/`](judgments/README.md) holds the hand-written judgments, one line
per entry. `tests/judgments.rs` always enforces `seed.txt` on the seed lexicon
and enforces every other entry whose text profiles at or below
`COMPLETED_STRATUM`; the rest are pending. Every evaluated entry also checks
both round-trip laws on each of its Readings. `stratum_zero.txt` is also
always enforced in full, including its outer spines over seed clauses.
`tests/stratum_zero.rs` independently constructs keyword-list Readings with
both separators, checks multiline document boundaries, and proves that a new
catalog keyword's parameters bind without changing the grammar. The census uses the same entries
to find known-wrong-only units.
