# sylex_english

The Oracle English grammar
([`docs/design/english.md`](../../docs/design/english.md)): the
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

- **Card text justifies admission.** A Schema, Production, feature value or
  Reading is added or kept because card text needs it. CGEL analyzes the
  forms cards print and tells a grammatical Reading from an ungrammatical one;
  a form only CGEL attests is no reason to add or keep a rule, and no reason
  to retire a gate that refuses it
  ([`docs/design/english.md`](../../docs/design/english.md) §4, §6).
- **Census before and after.** Every change to the grammar, the engine, the
  lexicon or the census runs the census before and after it:

  ```
  cargo xtask english strata --output target/english/before.json   # before
  cargo xtask english strata --check                               # after
  ```

  No face is lost, and every unit whose verdict or Readings change is
  reported.
- **Canonicalizations.** A change that introduces or tightens a
  canonicalization ([`docs/design/english.md`](../../docs/design/english.md)
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
  ([`docs/design/english.md`](../../docs/design/english.md) §6), remove
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
(`docs/design/english.md` §3), Host-Category-limited attachment with
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
Card Face record with its own name in lexical scope, in its Shortened form
too for a legendary permanent (`Face::name_scope`, a function of the face's
name and type line). `face` reads a face's lines once each, in order, carrying
the words they introduce (Introduced Names, Anchor Words, Vote Options) to the
lines after them (`face::read_lines`; `docs/design/lexicon.md` §2). Each face
yields
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
evaluator's no-Reading checks use it, and `total_derivations` sums it. It
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
the least Ranking Key (`docs/design/english.md` §5), before the final
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

## The forest database

```
cargo xtask english forests [--card NAME]... [--output PATH]
```

writes every Corpus Unit's Packed Forest, with its exact Derivation counts
(`sylex_syntax::Counts`), to one SQLite file, by default
`data/derived/english-forests.sqlite` (untracked). Each unit is parsed as the
census parses it. Questions across the pool are SQL over it, and its units
join mtgjson's `cardIdentifiers.scryfallOracleId` by `units.oracle_id`. The
forest stays packed (`docs/design/english.md` §4): there is no row per
Reading. The schema (`sqlite3 FILE .schema`) describes every table and view:

- `units`; `nodes` (Category, span, surface, Gap, features, coordinator,
  settled); `alternatives` (Production, completed item); `items` (Production,
  dot, span); `families` (previous item, child node or leaf); `leaves` (a
  word with its lexeme, the face's own name with its name form, an Introduced
  Name, a Flavor Word, a Set Name, an Anchor Word, a Vote Option or a
  numeral);
- per alternative: `heads` (head leaf, lexeme and form),
  `alternative_embeddings` (the Embedding it stands under: `embeddings`, with
  its context, and `embedding_steps`) and, for a coordination, `conjuncts`;
  and the grammar's `productions` (with Selected Frame and variant),
  `categories` and `lexemes`;
- per unit, its forks (`sylex_syntax::ForkForest::forks`): `words`, `forks`
  (kind, word, context, conditional), `fork_aliases`, `fork_options` (with
  their Derivations) and `fork_answers` (each option's answer to the fork's
  question and to each alias's), and `head_lines` (each node with the words
  whose Head Line runs through it across Derivations, with counts);
- views `verb_phrases`, `verb_phrase_spans`, `verb_phrase_tokens`,
  `coordinations`, `coordination_heads` and `wide_narrow_attachments`.

Every `derivations` column is exact. A fact holds in some Derivation where it
has a row, and in every Derivation where the Derivations that have it equal
`units.derivations`. That is a sum, exact only over rows no two of which one
Derivation can hold: rows of one node; rows of nodes that share a span and a
Production (the chart packs a Constituent by its bound features too, so one
verb phrase is often several nodes, as *cast* Present and Preterite are in
*Whenever you cast …*); and verb phrases that share a head token. The writer
checks on every unit that no Derivation holds two of the second or third
kind. `verb_phrase_spans` and `verb_phrase_tokens` do these sums, with an
`every` column. Rows of different spans and head tokens are never summed.

A verb phrase's context (`effect clause`, `trigger clause`, `would clause`,
`deontic`, `relative clause`, `participial postmodifier` or `other`) is read
from the first step of its Embedding that is no wrapper: a passive or perfect
auxiliary's Complement, a conjunct, do-support and an adjectival passive under
*be* only carry it to its context
(`crates/xtask/src/english/forests/context.rs` states each rule).

Passive verb phrases by verb and frame, in units where some Derivation has
one, and where one head token is one in every Derivation (*blocked* is
passive or adjectival in most):

```sql
SELECT lemma, frame, count(DISTINCT unit) AS some_units,
  count(DISTINCT CASE WHEN every THEN unit END) AS every_units
FROM verb_phrase_tokens WHERE variant LIKE 'passive%'
GROUP BY lemma, frame ORDER BY some_units DESC LIMIT 6;
```

```
lemma  frame                                                                              some_units  every_units
block  Monotransitive(NP)                                                                        718           15
put    TransitiveWithPP(NP,PP[prep:into|prep:in|prep:on|prep:onto|prep:back|prep:there])         558          542
deal   TransitiveWithPP(NP,PP[prep:to])                                                          405          393
deal   Ditransitive(NP,NP)[PassiveOnly]                                                          390           84
name   ComplexTransitive(NP,Name)                                                                343          318
exile  Monotransitive(NP)                                                                        323          243
```

The verbs of *would* clauses, by Voice (1,084 units have one):

```sql
SELECT lemma, voice, count(DISTINCT unit) AS units
FROM verb_phrase_tokens WHERE context = 'would clause'
GROUP BY lemma, voice ORDER BY units DESC LIMIT 8;
```

```
lemma  voice    units
deal   Passive    449
deal   Active     207
put    Passive    125
be     Active     115
die    Active      95
draw   Active      49
gain   Active      20
create Passive     19
```

Verb phrases by context:

```sql
SELECT context, count(DISTINCT unit) AS some_units,
  count(DISTINCT CASE WHEN every THEN unit END) AS every_units
FROM verb_phrase_tokens GROUP BY context ORDER BY some_units DESC;
```

```
context                   some_units  every_units
effect clause                  45182        43642
other                          15528        14088
trigger clause                 13669        13399
relative clause                11500        11209
deontic                         7199         5202
participial postmodifier        1951         1269
would clause                    1084          919
```

Does every Derivation of a unit contain a passive verb phrase headed by
*deal*? Yes where one head token is one in every Derivation. Different head
tokens can share a Derivation, so their counts are never summed; a unit where
no one token covers every Derivation is not answered by these columns:

```sql
SELECT u.face_name, u.line, t.lemma, t.frame
FROM verb_phrase_tokens t JOIN units u USING (unit)
WHERE t.lemma = 'deal' AND t.variant LIKE 'passive%' AND t.every
ORDER BY u.face_name, u.line LIMIT 5;
```

```
face_name         line  lemma  frame
Abattoir Ghoul       1  deal   Ditransitive(NP,NP)[PassiveOnly]
Abuna Acolyte        0  deal   TransitiveWithPP(NP,PP[prep:to])
Abuna Acolyte        1  deal   TransitiveWithPP(NP,PP[prep:to])
Abuna's Chant        2  deal   TransitiveWithPP(NP,PP[prep:to])
Acolyte's Reward     0  deal   TransitiveWithPP(NP,PP[prep:to])
```

Coordinations by Category, coordinator (where it has none, a Linked
Coordination's identity: the name of its serial tail Category) and conjunct
count:

```sql
SELECT category, coalesce(coordinator_lemma, link) AS coordinator, conjuncts,
  count(DISTINCT unit || ':' || node) AS some_nodes,
  count(DISTINCT CASE WHEN settled THEN unit || ':' || node END) AS settled_nodes
FROM coordinations GROUP BY 1, 2, 3 ORDER BY some_nodes DESC LIMIT 6;
```

```
category          coordinator          conjuncts  some_nodes  settled_nodes
Nominal           or                           2        4937           1007
VerbPhrase        and                          2        4504           1152
NounPhrase        and                          2        2867            467
Clause            Clause#LinkedClause          2        2505            458
Clause            and                          2        2436           1206
DeterminerPhrase  or                           2        1971           1709
```

Forks whose options are a wide and a narrow attachment over a coordination
(`wide_narrow_attachments`): in one option the dependent's governor is a
coordination (whose Head Line ends in its coordinator, or a Linked
Coordination's connective) of the governor's Category; in the other it is a
word inside the conjunct next to the dependent, of any Category and function,
in a Constituent that does not hold the coordinator. So *until end of turn*
modifies *gets +2/+2 and gains trample* or *gains trample*, and in *put a
+1/+1 counter on each creature you control and those creatures gain trample
until end of turn* the Clause coordination or the Verb Phrase *gain*. Each
option's count is the whole option's: where an option also packs Derivations
that coordinate differently, `narrow_derivations` counts those too. 5,753
forks in 4,533 units, by each option's function and governor Category:

```sql
SELECT wide_function, wide_governor_category, narrow_function, narrow_governor_category,
  count(DISTINCT unit || ':' || fork) AS forks, count(DISTINCT unit) AS units
FROM wide_narrow_attachments GROUP BY 1, 2, 3, 4 ORDER BY forks DESC LIMIT 6;
```

```
wide_function  wide_governor_category  narrow_function  narrow_governor_category  forks  units
modifier       Nominal                 modifier         Nominal                    1641   1367
adjunct        VerbPhrase              adjunct          VerbPhrase                 1409   1220
modifier       NounPhrase              modifier         Nominal                    1061    822
adjunct        Clause                  adjunct          VerbPhrase                  735    634
adjunct        Clause                  adjunct          Clause                      717    643
adjunct        Clause                  modifier         Nominal                     339    299
```

Units where the two options are every Derivation:

```sql
SELECT u.face_name, a.dependent_surface AS dependent, a.coordinator_surface AS wide,
  a.narrow_governor_surface AS narrow, a.wide_derivations, a.narrow_derivations,
  u.derivations
FROM wide_narrow_attachments a JOIN units u USING (unit)
WHERE u.derivations = a.wide_derivations + a.narrow_derivations
ORDER BY u.face_name, u.line LIMIT 6;
```

```
face_name                      dependent  wide  narrow       wide_derivations  narrow_derivations  derivations
A Killer Among Us              until      and   gains                       1                   1            2
Abandon Reason                 until      and   gain                        1                   1            2
Aberrant                       controls   or    enchantment                 1                   1            2
Aberrant Mind Sorcerer         When       then  choose                      5                   5           10
Abeyance                       Until      and   can't                       1                   1            2
Abomination                    creature   or    by                          1                   1            2
```

The census faces with no Vintage `Legal` or `Restricted` printing in mtgjson
(run from the repository root):

```sql
ATTACH 'data/mtgjson/AllPrintings.sqlite' AS mtg;
WITH legal AS (
  SELECT DISTINCT i.scryfallOracleId AS oracle_id
  FROM mtg.cardIdentifiers i JOIN mtg.cardLegalities l USING (uuid)
  WHERE l.vintage IN ('Legal', 'Restricted'))
SELECT count(*) AS faces,
  sum(oracle_id NOT IN (SELECT oracle_id FROM legal)) AS without_vintage_printing
FROM (SELECT DISTINCT oracle_id, face FROM units);
```

```
faces  without_vintage_printing
33295                         0
```

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
  card so named: the name is an input like the text, no card is fetched, and
  the name need be no printed card's. `--legendary` states that the card is a
  legendary permanent, which gives its name its Shortened form (*Abuelo* for
  `--as-card 'Abuelo, Ancestral Echo' --legendary`); without it only the full
  name is the card's own. An input in entry syntax may state its card itself
  with `<card=NAME>`, and `<legendary>` after it, instead; giving both is an
  error.
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
judgment evaluator, including its round-trip checks;
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

## Surveying a unit's forest

```sh
cargo xtask english forestwalk --card 'Draugr Necromancer'
cargo xtask english forestwalk --card 'Draugr Necromancer' --line 2
cargo xtask english forestwalk --card 'Draugr Necromancer' --type-line
cargo xtask english forestwalk --as-card 'Jace, Cunning Castaway' --legendary 'Jace draws two cards.'
```

`forestwalk` shows one unit's ambiguity without enumerating its Readings, so a
unit of thousands of Readings fits on a screen. `--card NAME` alone lists the
units of every matching face (`FACE type line`, `FACE line N`). To survey one
unit, give text (one entry, without assertions; `--as-card`, with `--legendary`
for a legendary permanent, or `<card=NAME>` for its card), or `--card NAME` with `--line N` or `--type-line`, where `NAME`
is a face's name (or the whole card's when the card has one face). Faces of two
cards that share a name (*Lightning Bolt*) give one unit when the picked units
have the same text. A wrong choice is an error that lists the face's units or
names.

It prints the Derivation count, then the settled spine
(`ForkForest::settled_spine`): expanded branching nodes print as `function:
Category [Production]` without repeating the span surface, a chain of single
children folds into one line, and a leaf prints as its word and lexical
reading. With one Derivation left (unless `--limit 0` is given), the whole
Reading expands on `Settled spine:`; while choices remain (or under
`--limit 0`), each settled subtree with no stop inside it folds to one
`function: Category "surface"` line, and each stop prints its `"surface"` with
its alternative count or child-split difference. Then come the forks
(`ForkForest::forks`), each named by its kind and word, with whether it is
conditional and its aliases, and under it each option's exact Derivation count
and answer, then under each option one line per alias in the header's order,
the alias's name and its answer. A word is printed `"surface"#N`, its
surface and 1-based occurrence. Options are in the forks' order, never ranked:
the survey decides nothing
([english.md](../../docs/design/english.md) §4, §5).

### Narrowing a forest

```sh
cargo xtask english forestwalk --card 'Draugr Necromancer' --line 2 \
  --pick 'dependency "from"#1 modifier of Nominal "spells"#1' \
  --exclude 'dependency "in"#1 modifier of Nominal "cards"#1'
cargo xtask english forestwalk --card 'Crook of Condemnation' --line 1 \
  --pick 'dependency "from"#1 modifier of Nominal "card"#1' --assertions
cargo xtask english forestwalk --card 'Crook of Condemnation' --line 1 --limit 2
```

`--pick` and `--exclude` narrow the forest by fork options before the survey
(`ForkForest::pick`, `ForkForest::exclude`), enumerating no Reading, so a unit
of any size narrows. A selector is one argument, `KIND "surface"#N ANSWER`: a
fork's name as the survey prints it (an alias's too) and, after it, one
option's answer without its count; the line the survey prints under an option
for an alias is a whole selector. The occurrence is always written, a
governor's too; a bare `"surface"` is an error, for a surface that occurs once
as well. A
selector begins with a lowercase `lexical`, `dependency` or `frame` and names
grammar child names (`adjunct`, `first`), which keeps it apart from a
judgment's assertions (`Category "surface" as Function of lemma`). Steps apply
in the order written, `--pick` and `--exclude` interleaved, and each prints the
Derivations left; then the survey prints the narrowed forest. A selector that
names no word, fork or option of the unit, an option
an earlier step has removed, and a step that leaves no Derivation are errors
naming the selector.

With at most `--limit` Derivations left (default 1), one Derivation expands the
whole Reading on `Settled spine:` (`--limit 0` folds its root to `Category
"surface"`), and two or more print each `Reading N:` as its folded subtree(s)
at the stop(s) of `Settled spine:` (`ForkForest::settled_readings`, each
carrying the function label its stop bears on `Settled spine:`, or the whole
Reading when no root Constituent is settled). `--assertions` accepts between 1
and `max(--limit, 1)` Derivations (`--limit 0 --assertions` isolates a single
Reading without printing its tree) and prints, for each remaining Reading,
judgment assertions that isolate it among the unit's Readings. They are found
by greedy set cover over the assertions the Reading's own Constituents suggest
(a Category and surface, with an occurrence, a head word, a function or a Gap
Antecedent), each holding on it (`Judge::holds`) and covering the unit's other
Readings it fails on, and it enumerates the unit's Readings as the judge does
(`Judge::evaluate`). The output is diagnostic, not a judgment to copy unread:
when no assertion of these forms tells some Readings apart from it, as when
they differ only in Productions, it reports how many.

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
