---
needs: [english-names-read-from-text, english-set-names-read-from-text]
---

# Vote options are read from a face's text

## Why

`crates/vocabulary/catalogs/vote_options.ron` (42 entries across 26 in-scope faces) prepares each
vote option [CR#701.38a,701.38b] and the cards that print it before parsing, loading card-scoped
`vote_option:` Lexemes through `ScopedName`. Vote options are words offered by the card's own vote
instruction ("votes for sickness or psychosis", "votes for aid or fellowship") and mentioned later
on the same face ("If psychosis gets more votes", "for each numbers vote", "twice the number of
profit votes"). Pre-listing them per card records facts about individual cards before parsing,
including options shared with English nouns, adjectives, verbs (`time`, `free`, `wild`, `return`,
`embark`, `past`, `present`, `chaos`) and card names (*Mines of Moria*). `vote_options.ron` and
`ScopedName` go.

## Design

- Vote options that are mentioned words rather than object, player or quality Noun Phrases ("votes
  for a player", *Council Guardian*'s `"votes for blue, black, red, or green"`) are read from a
  face's binary vote-introduction unit (`votes for A or B` [CR#701.38a,701.38b]) in a shared
  `sylex_english` face-scope pass (used by `census`, `forests`, `english parse --card` and
  `english forestwalk --card`), confirmed when a vote-result or vote-count mention of `{A, B}`
  occurs on the face (`<option> gets more/the most votes`, `<option> is tied for the most votes`,
  `each <option> vote`, `<option> votes`, `voted for <option>`), and held in `CardScope`.
- Where `CardScope` holds a vote option, `Lexicon` and the grammar admit it in those vote positions
  (including multi-word capitalized options such as *Mines of Moria* and *Redhorn Pass*), tying
  subject consumption to vote-tally predicates so homographs in ordinary clauses (`"chaos ensues"`
  on the five *Path of the …* cards) gain no duplicate `Name` Reading, and naming no `Vote`
  identifier in `english.fug` (`crates/english/tests/catalog_leak.rs`).
- Standalone single-line judgment entries (`mentions_and_votes.txt`) and `english parse` accept an
  explicit `<vote=...>` / `--vote` scope tag when the introducing or confirming unit is not present.
- `vote_options.ron`, `Sources::vote_options`, the `vote_option:` loader path and `ScopedName` are
  deleted, leaving zero card-scoped catalogs in `sylex_vocabulary`.

## Done when

- `vote_options.ron`, `Sources::vote_options`, `vote_option:` Lexemes and `schema::ScopedName` are
  deleted, and `crates/vocabulary/README.md` lists no card-scoped catalog.
- Every unit of all 26 in-scope word-vote cards (*Bite of the Black Rose*, *Capital Punishment*,
  *Coercive Portal*, *Elrond of the White Council*, *Emissary Green*, *Expropriate*, *Fateful
  Tempest*, *Galadriel, Elven-Queen*, *Lieutenants of the Guard*, *Magister of Worth*, *Messenger
  Jays*, *Orchard Elemental*, *Path of the Animist*, *Path of the Enigma*, *Path of the
  Ghosthunter*, *Path of the Pyromancer*, *Path of the Schemer*, *Plea for Power*, *Sail into the
  West*, *Selvala's Stampede*, *Split Decision*, *Tivit, Seller of Secrets*, *Travel Through
  Caradhras*, *Trial of a Time Lord*, *Truth or Consequences*, *Tyrant's Choice*) and non-word vote
  cards (*Ballot Broker*, *Brago's Representative*, *Council Guardian*, *Council's Judgment*,
  *Illusion of Choice*) reads and round-trips cleanly.
- `crates/english/judgments/mentions_and_votes.txt`, `crates/english/tests/catalog_leak.rs` and
  standalone CLI/loader/scope tests pin vote-option introduction, later singular and plural
  vote-count and tally mentions, multi-word options (*Mines of Moria*), *Council Guardian* and
  `"chaos ensues"` non-leakage, and off-scope refusal.
- `docs/design/lexicon.md` §2 and §6 and `docs/contexts/oracle-english/CONTEXT.md` (Card Scope,
  Proper Name) drop `vote_options.ron` and state how vote options are read.
- `cargo xtask english strata --check` passes, and `cargo test` and
  `cargo clippy --tests -- -D warnings` pass for `sylex_lexicon`, `sylex_vocabulary`,
  `sylex_syntax`, `sylex_english` and `xtask`.

## Not in scope

- Authoring or running the voting procedure in `.syl` or the engine (`engine-voting-procedure`,
  `workbench-secret-choices-and-votes`).

## Pointers

- `crates/vocabulary/catalogs/vote_options.ron`, `crates/vocabulary/src/lib.rs`
  (`scoped_name_catalogs`), `crates/vocabulary/src/schema.rs` (`ScopedName`).
- `crates/lexicon/src/analysis.rs`, `crates/lexicon/src/scope.rs` (`CardScope`).
- `crates/english/grammar/english.fug`, `crates/english/src/judgment.rs`,
  `crates/english/judgments/mentions_and_votes.txt`, `crates/english/tests/catalog_leak.rs`.
- `crates/xtask/src/english.rs`, `crates/xtask/src/english/forests.rs`.
- `docs/design/lexicon.md` §2, §6; `docs/oracle-style-guide.md` §12;
  `docs/contexts/oracle-english/CONTEXT.md`.
