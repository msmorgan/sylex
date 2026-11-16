---
needs: [english-names-read-from-text, english-set-names-read-from-text]
---

# Vote options are read from a face's text

## Why

`crates/vocabulary/catalogs/vote_options.ron` (37 entries across 20 in-scope faces plus 5
out-of-scope Plane cards) prepares each vote option [CR#701.38a,701.38b] and the cards that print
it before parsing, loading card-scoped `vote_option:` Lexemes through `ScopedName`. Vote options are
words offered by the card's own vote instruction ("votes for sickness or psychosis", "votes for
aid or fellowship") and mentioned later on the same face ("If psychosis gets more votes", "for each
numbers vote"). Pre-listing them per card records facts about individual cards before parsing,
including options shared with English nouns, adjectives, verbs (`time`, `free`, `wild`, `return`,
`embark`, `past`, `present`) and card names (*Mines of Moria*). `vote_options.ron` and `ScopedName`
go.

## Design

- Vote options that are mentioned words rather than object or player Noun Phrases ("votes for a
  player") are read from the face's vote-introduction unit (`votes for A or B` / serial
  `votes for A, B, or C` [CR#701.38a,701.38b]) in a shared `sylex_english` face-scope pass (used by
  `census`, `forests`, `english parse --card` and `english forestwalk --card`) and held in
  `CardScope` across the face's units.
- Where `CardScope` holds a vote option, `Lexicon` and the grammar admit it as a cited vote-option
  `Name` in the vote positions (`votes for …`, subject of `gets more/the most votes`, `each <option>
  vote`, `voted for <option>`), including multi-word capitalized options (*Mines of Moria*,
  *Redhorn Pass*), without pre-preparing any card list.
- Standalone single-line judgment entries (`mentions_and_votes.txt`) and `english parse` accept an
  explicit `<vote=...>` / `--vote` scope tag when the introducing sentence is not in the unit.
- `vote_options.ron`, `Sources::vote_options`, the `vote_option:` loader path and `ScopedName` are
  deleted, leaving zero card-scoped catalogs in `sylex_vocabulary`.

## Done when

- `vote_options.ron`, `Sources::vote_options`, `vote_option:` Lexemes and `schema::ScopedName` are
  deleted, and `crates/vocabulary/README.md` lists no card-scoped catalog.
- Every unit of all 20 in-scope vote-option cards (*Bite of the Black Rose*, *Capital Punishment*,
  *Coercive Portal*, *Elrond of the White Council*, *Emissary Green*, *Expropriate*, *Fateful
  Tempest*, *Galadriel, Elven-Queen*, *Lieutenants of the Guard*, *Magister of Worth*, *Messenger
  Jays*, *Orchard Elemental*, *Plea for Power*, *Sail into the West*, *Selvala's Stampede*, *Split
  Decision*, *Tivit, Seller of Secrets*, *Travel Through Caradhras*, *Trial of a Time Lord*, *Truth
  or Consequences*, *Tyrant's Choice*) and object/player vote cards (*Ballot Broker*, *Brago's
  Representative*, *Council's Judgment*, *Illusion of Choice*) reads and round-trips cleanly.
- `crates/english/judgments/mentions_and_votes.txt` and standalone CLI/loader/scope tests pin vote
  option introduction, later mention on the unit and across units on the face, multi-word options
  (*Mines of Moria*) and off-scope refusal.
- `docs/design/lexicon.md` §2 and §6, `docs/contexts/oracle-english/CONTEXT.md` (Card Scope, Proper
  Name) and `docs/tickets/planned/catalogs-into-plugins.md` drop `vote_options.ron` and state how
  vote options are read.
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
  `crates/english/judgments/mentions_and_votes.txt`.
- `crates/xtask/src/english.rs`, `crates/xtask/src/english/forests.rs`.
- `docs/design/lexicon.md` §2, §6; `docs/oracle-style-guide.md` §12;
  `docs/contexts/oracle-english/CONTEXT.md`.
