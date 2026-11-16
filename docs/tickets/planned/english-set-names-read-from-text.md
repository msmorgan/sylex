---
needs: [lexicon-dungeon-names]
---

# Expansion names a card's text cites are read from the text

## Why

`crates/vocabulary/catalogs/cited_names.ron` prepares the three expansion names cited by
Vintage-legal cards before parsing (*Arabian Nights* on *City in a Bottle*, *Antiquities* on
*Golgothian Sylex*, *Homelands* on *Apocalypse Chime* [CR#206.3,206.3a..206.3c]), scoping each to
its card. Across Vintage-legal cards, the noun *expansion* appears only in those four `"with a name
originally printed in the NAME expansion"` sentences. Parsing a cited set name needs neither a
card-scoped list nor a catalog of all printed Magic sets.

Set names in `data/mtgjson/SetList.json` (882 sets total, 121 `type: "expansion"`) do not fit the
introduced-name shape rule of `english-names-read-from-text`: 19 of the 121 `expansion` sets (23 of
882 overall) share their name with a card (*Apocalypse*, *Time Spiral*, *Planar Chaos*, *Future
Sight*, *Visions*, *Mirrodin Besieged*, *Urza's Saga*, *The Brothers' War*, ...), 13 `expansion`
sets (126 overall) contain `:`, 6 `expansion` sets (40 overall) contain `'` including word-final
`'` (*The Brothers' War*, *Intl. Collectors' Edition*), others contain `-`, `.`, `/`, `&`, `×`,
digits (*Magic 2015*, *30th Anniversary Edition*) and `,` inside numerals (*Warhammer 40,000
Commander*), and interior lowercase words include `in`, `for`, `into`, `at`, `over`, `and`, `a` and
`vs.` beyond `of`, `the` and `to`. None of the 882 set names contains the word *expansion*, and
every one starts with `A–Z` or `0–9`.

## Design

- A set name cited before *expansion* is an open lexical string read from the text by orthographic
  title shape in `sylex_lexicon`, with no card-name refusal and no lookahead for English words
  (`docs/design/lexicon.md` §2). Because *expansion* is lowercase and no set name contains it, a
  title run — space-separated tokens starting and ending on an `A–Z`/`0–9` token (allowing `:`, `'`,
  `-`, `.`, `,`, `/`) with internal connectors (`of`, `the`, `to`, `in`, `for`, `into`, `at`,
  `over`, `and`, `a`, `vs.`, `&`, `/`, `×`) — stops before `" expansion"` across all 121 `expansion`
  sets and 881 of 882 sets in `SetList.json` (all except *The List (Unfinity Foil Edition)*, whose
  parentheses reminder-text removal strips in `lexicon.md` §1).
- `nouns.ron` declares a prenominal `parameters` pattern (or frame) on `expansion` whose slot
  admits that open `<SetName>` leaf in `"the <SetName> expansion"` (matching [CR#206.3a..206.3c])
  without naming `expansion` as a closed word in `english.fug` (`crates/english/tests/catalog_leak.rs`),
  so it never stands in general `Name` positions and builds no second tree
  (`docs/design/english.md` §8) where a card's own name matches the set name.
- `cited_names.ron`, `Sources::cited_names` and the `cited_name:` loader path are deleted.

## Done when

- `cited_names.ron`, `Sources::cited_names` and the `cited_name:` loader path are deleted,
  `crates/vocabulary/tests/loader.rs` drops its `cited_names` setup, and
  `crates/vocabulary/README.md` no longer lists `cited_names.ron`.
- *Golgothian Sylex*, *City in a Bottle* (both abilities) and *Apocalypse Chime* keep their single
  Reading and exact round trip, and *Golgothian Sylex*'s judgment in
  `crates/english/judgments/mentions_and_votes.txt` holds without requiring `<card=...>` for the
  expansion name.
- Lexicon and grammar tests pin `"originally printed in the NAME expansion"` across representative
  `SetList.json` shapes with no catalog entry: a set name shared with a card name (*Time Spiral*,
  *Urza's Saga*, including when parsed in that card's own `CardScope` — 1 Reading), a colon subtitle
  (*Phyrexia: All Will Be One*), a word-final apostrophe (*The Brothers' War*), interior
  prepositions (*Adventures in the Forgotten Realms*, *Shadows over Innistrad*), digits and commas
  (*Warhammer 40,000 Commander*, *30th Anniversary Edition*), and admission's refusal of the open
  leaf off *expansion*.
- `docs/design/lexicon.md` §2 (open lexical slots, beside Flavor Word [CR#207.2d]) and
  `docs/contexts/oracle-english/CONTEXT.md` (Proper Name dropping `"a cited name"`, Flavor Word no
  longer `"the one open lexical slot"`) state the design.
- `cargo xtask english strata --check` passes, the census is unchanged, and `cargo test` and
  `cargo clippy --tests -- -D warnings` pass for `sylex_lexicon`, `sylex_vocabulary`,
  `sylex_syntax`, `sylex_english` and `xtask`.

## Not in scope

- Authoring or evaluating `"with a name originally printed in"` in `.syl` or the engine
  (`syl-name-originally-printed-in`, `engine-name-originally-printed-in`).
- Introduced token and object names (`english-names-read-from-text`), and `vote_options.ron` and
  `ScopedName` (`english-vote-options-read-from-text`).

## Pointers

- `crates/vocabulary/catalogs/cited_names.ron`, `crates/vocabulary/src/lib.rs`
  (`scoped_name_catalogs`), `crates/vocabulary/src/schema.rs` (`ScopedName`),
  `crates/vocabulary/lexicon/english/nouns.ron` (`expansion`).
- `crates/lexicon/src/analysis.rs` (`flavor_word_matches`, `realize`, `surface_features`, `edges`,
  `stands_in`), `crates/lexicon/src/types.rs` (`Category`, `LexicalReading`).
- `crates/syntax/src/step.rs`, `crates/syntax/src/prediction.rs`,
  `crates/syntax/src/alternatives.rs`.
- `crates/english/grammar/english.fug`, `crates/english/src/profile.rs`,
  `crates/english/judgments/mentions_and_votes.txt`, `crates/english/tests/catalog_leak.rs`.
- `data/mtgjson/SetList.json`, `docs/design/lexicon.md` §2, `docs/design/english.md` §7–8,
  `docs/contexts/oracle-english/CONTEXT.md`,
  `docs/tickets/planned/syl-name-originally-printed-in.md`.
