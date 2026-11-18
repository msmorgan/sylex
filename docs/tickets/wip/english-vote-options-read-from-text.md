---
needs: [english-names-read-from-text, english-set-names-read-from-text]
---

# Vote options are read from a face's text

## Why

`crates/vocabulary/catalogs/vote_options.ron` (42 entries across 26 in-scope faces) prepares each
vote option [CR#701.38a,701.38b] and the cards that print it before parsing, loading card-scoped
`vote_option:` Lexemes through `ScopedName`. Vote options are words offered by the card's own vote
instruction ("votes for sickness or psychosis", "votes for fellowship or aid") and mentioned later
on the same face, on every printed card in the same line ("If psychosis gets more votes", "for
each numbers vote", "twice the number of profit votes"). As Catalog names they stood wherever a
Noun Phrase does, so each one spelled like a mass noun forked its line at every mention (8
Derivations on each *Path of the …* card, `"chaos ensues"` included). Pre-listing them per card records facts about individual cards before parsing,
including options shared with English nouns, adjectives, verbs (`time`, `free`, `wild`, `return`,
`embark`, `past`, `present`, `chaos`) and card names (*Mines of Moria*). `vote_options.ron` and
`ScopedName` go.

## Design

- A Vote Option is a word a vote offers and the text then cites, not an object, player or quality
  Noun Phrase ("votes for a player", *Council Guardian*'s `"votes for blue, black, red, or green"`)
  [CR#701.38a,701.38b]. A face's Vote Options are read by parses made to find them
  (`sylex_english::options::vote_options`, called by `Face::scope` beside the Anchor Words, so
  `census`, `forests`, `english parse --card` and `english forestwalk --card` share it). A line
  that writes a word whose frame selects an Option Choice (*vote*) is parsed with every word and
  title run in it in scope as a Vote Option, and the words an `OptionChoice` in some Derivation
  offers are what the vote offers. The face's lines are then parsed with those words in scope,
  and an offer is confirmed when some Derivation cites one of its words in an `OptionMention`.
  The printed mentions are two: the Subject of *gets more votes*, and the premodifier of *vote*
  (*each numbers vote*, *the number of profit votes*). The confirmed words are held in
  `CardScope`. No string match on the text decides either step: CLAUDE.md, "Card text".
- `LexicalReading::VoteOption` stands where `CardScope` holds its word, one word or a title run
  (*Mines of Moria*, *Redhorn Pass*). The grammar admits it in three positions and names no
  keyword action (`english.fug` says `OptionWord`, `OptionChoice`, `OptionMention`,
  `OptionLicense`; `crates/english/tests/catalog_leak.rs`): the `OptionChoice` that *vote*'s frame
  selects after *for*; the parameter the noun *vote* takes before it, whose license
  (`OptionLicense`) keeps a noun of the option's spelling from premodifying *vote*; and the
  Subject of a clause whose Verb Phrase's Complement is headed by that noun (`OptionTally`), so
  `"chaos ensues"` on the five *Path of the …* cards has only the noun. Before *gets more votes*
  the noun *chaos* keeps its own Reading beside the cited word's: a noun may be the Subject of
  *gets more votes* (*If no creature got votes*, Vault 11: Voter's Dilemma), so no gate refuses
  it.
- A judgment entry and `english parse` / `english forestwalk` text state their Vote Options with
  `<vote=WORD>` / `--vote WORD`, as they state Anchor Words: an entry is read with the words it
  states, whether or not its text offers them.
- `vote_options.ron`, `Sources::vote_options`, the `vote_option:` loader path, `ScopedName`,
  `Lexeme::card_scope` and `CardScope::of_cards` are deleted, leaving zero card-scoped catalogs
  in `sylex_vocabulary` and no Lexeme scoped to cards.

## Done when

- `vote_options.ron`, `Sources::vote_options`, `vote_option:` Lexemes, `schema::ScopedName` and
  `Lexeme::card_scope` are deleted, and `crates/vocabulary/README.md` lists no card-scoped
  catalog.
- Every unit of all 26 in-scope word-vote cards (*Bite of the Black Rose*, *Capital Punishment*,
  *Coercive Portal*, *Elrond of the White Council*, *Emissary Green*, *Expropriate*, *Fateful
  Tempest*, *Galadriel, Elven-Queen*, *Lieutenants of the Guard*, *Magister of Worth*, *Messenger
  Jays*, *Orchard Elemental*, *Path of the Animist*, *Path of the Enigma*, *Path of the
  Ghosthunter*, *Path of the Pyromancer*, *Path of the Schemer*, *Plea for Power*, *Sail into the
  West*, *Selvala's Stampede*, *Split Decision*, *Tivit, Seller of Secrets*, *Travel Through
  Caradhras*, *Trial of a Time Lord*, *Truth or Consequences*, *Tyrant's Choice*) and of the
  other cards that vote (*Ballot Broker*, *Brago's Representative*, *Council Guardian*,
  *Council's Judgment*, *Illusion of Choice* among them) reads and round-trips cleanly, the
  first with the two words its vote offers in scope and the others with none
  (`crates/english/tests/vote_options.rs`).
- `crates/english/judgments/mentions_and_votes.txt`, `crates/english/tests/catalog_leak.rs`,
  `crates/english/tests/vote_options.rs` and the CLI and lexicon scope tests pin the offer, the
  two kinds of mention, multi-word options (*Mines of Moria*), *Council Guardian* and
  `"chaos ensues"` non-leakage, and off-scope refusal.
- `docs/design/lexicon.md` §2 and §6, `docs/design/english.md` §3 and §7 and
  `docs/contexts/oracle-english/CONTEXT.md` (Vote Option, Option Choice, Option Mention, Card
  Scope, Proper Name) drop `vote_options.ron` and state how Vote Options are read.
- `cargo xtask english strata --check` passes, and `cargo test` and
  `cargo clippy --tests -- -D warnings` pass for `sylex_lexicon`, `sylex_vocabulary`,
  `sylex_syntax`, `sylex_english` and `xtask`.

## Not in scope

- Authoring or running the voting procedure in `.syl` or the engine (`engine-voting-procedure`,
  `workbench-secret-choices-and-votes`).

## Pointers

- `crates/vocabulary/catalogs/keyword_actions.ron` (*vote*'s frames),
  `crates/vocabulary/lexicon/english/nouns.ron` (*vote*'s parameter pattern).
- `crates/lexicon/src/vote_option.rs`, `crates/lexicon/src/scope.rs` (`CardScope`).
- `crates/english/src/options.rs`, `crates/english/src/anchors.rs` (the precedent),
  `crates/english/grammar/english.fug`, `crates/english/src/judgment.rs`,
  `crates/english/judgments/mentions_and_votes.txt`, `crates/english/tests/catalog_leak.rs`.
- `crates/xtask/src/english.rs`, `crates/xtask/src/english/forests.rs`.
- `docs/design/lexicon.md` §2, §6; `docs/oracle-style-guide.md` §12;
  `docs/contexts/oracle-english/CONTEXT.md`.
