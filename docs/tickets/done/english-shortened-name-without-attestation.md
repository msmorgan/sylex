---
needs: []
---

# A legendary card's Shortened Name does not depend on its text using it

## Why

A card's name is a proper name with its forms: the Full form and, for a legendary card, a
Shortened form. [CR#201.5c] reads: "Text printed on some cards refers to that card by a shortened
version of its name. Instances of a card’s shortened name used in this manner are treated as
though they used the card’s full name." The Oracle convention gives the shortened form to a
legendary name (`docs/oracle-style-guide.md` §5: "An unambiguous shortened form may replace a
long legendary name").

The lexicon gives a card-name Lexeme its Shortened form under two conditions: the face is a
legendary permanent with a candidate from the rule (`shortening::short_name`, a function of the
name and the type line), and the face's own Oracle text uses that candidate
(`shortening::attested_short_name`). The second condition is wrong: whether a card has a
Shortened Name does not depend on its printed text happening to use it.

- *Abuelo, Ancestral Echo* is a Legendary Creature whose text never says *Abuelo*, so
  `cargo xtask english parse --as-card "Abuelo, Ancestral Echo" "When Abuelo enters, draw a card."`
  has a Vocabulary Gap at *Abuelo*. It should read.
- Attestation is computed by scanning every card's text (`AttestedShortenings::from_corpus`),
  each time `LexicalEnvironment::load` runs. That scan is the one read of
  `data/scryfall/oracle-cards.jsonl` left on every `english parse` call, about 1 s of 2 s.
  Without attestation, a Shortened form needs only the name and that the card is legendary.

Over the 4,466 supported legendary permanent faces: 4,192 have a candidate; 1,911 are attested;
2,281 have a candidate and no Shortened form today. In 2,268 of those the text uses no
shortening of the name at all, and in 13 the candidate occurs only inside a longer title the
text names (*Mishra* in *Mishra, Lost to Phyrexia* on *Mishra, Claimed by Gix*; *Nissa* in
*Nissa's Chosen* on *Nissa Revane*; *Marit* in *Marit Lage* on *Marit Lage's Slumber*). No
legendary face's text uses a shortening other than its candidate: attestation never chooses the
spelling, it only withholds the form.

A name a card's text gives another object (*Rinoa Heartilly*'s *create Angelo, a legendary ...
token*) is a different matter with its own ticket; this one is about a card's own name.

## Design

- No per-card characteristics are prepared before parsing: `data/gen/catalogs/card-names.txt` holds
  card names alone (for `OtherCardName()`), and `Lexicon` stores no precomputed `Shortened` forms per
  card; `AttestedShortenings` and `attested_short_name` are deleted.
- On faces (`census`, `forests`, `--card`), `CardScope` derives the face's `Shortened` form at parse
  time from the face's own parsed type line (`Legendary` supertype plus permanent card type) and
  `shortening::short_name`, keeping `SURNAME_TITLES` and `names::shortened_name`'s vocabulary
  collision filter (*Sliver* in *Sliver Queen*).
- `shortening::rule` rejects non-comma candidates ending in or preceding a possessive (`'s`, `’s`,
  `s'`, `s’`), so legendary possessives (*Urza's Saga*, *Marit Lage's Slumber*, *Gaea's Cradle*)
  never derive `"Urza's"` or `"Marit"`, and handles two-word `SURNAME_TITLES` (`[title, surname]` on
  *Dr. Eggman*) without falling through to a bare title with a trailing period (`"Dr."`).
- Longer titles in the card's own text (*Mishra, Lost to Phyrexia* on *Mishra, Claimed by Gix*) are
  not withheld from `CardScope`; `OtherCardName()` / longest-title admission handles the full title
  at its `named` position.
- Standalone single-line inputs (`--as-card NAME` and `<card=NAME>`) admit any stated name as the
  card's own Full name in `CardScope` (including unprinted names such as *Abuelo the Brave*, via a
  self-contained `LexicalReading` or scope-independent realization so `CompiledGrammar::realize` and
  `admit` preserve both round-trip laws) and derive its `Shortened` form when an explicit
  `--legendary` / `<legendary>` (or `--type-line`) flag is given; `CardScope` continues to track
  Full vs Shortened availability accordingly, and existing shortened-name judgment entries add
  `<legendary>`.

## Done when

- `--as-card "Abuelo, Ancestral Echo" --legendary "When Abuelo enters, draw a card."` (and an
  unprinted `--as-card "Abuelo the Brave" --legendary`) reads *Abuelo* as that card's own name in
  its Shortened form and round-trips under Laws 1 and 2, while without `--legendary` (and no
  legendary permanent type line) only the Full form stands.
- `LexicalEnvironment::load` reads no card text, and an `english parse` call that fetches no
  card reads no `oracle-cards.jsonl`.
- `docs/design/lexicon.md` §3 and §6 and `docs/contexts/oracle-english/CONTEXT.md` (Shortened Name)
  drop the Oracle-text attestation condition.
- The pool census is compared before and after, every unit whose verdict or count changes is checked
  and reported, and `cargo xtask english strata --check`, `cargo test` and
  `cargo clippy --tests -- -D warnings` pass for every crate changed.

## Pointers

- `crates/vocabulary/src/shortening.rs` (`short_name`, `attested_short_name`,
  `AttestedShortenings`, `SURNAME_TITLES`), `crates/vocabulary/src/names.rs` (`shortened_name`,
  `card_names`), `crates/vocabulary/src/lib.rs` (`LexicalEnvironment::load`).
- `crates/lexicon/src/scope.rs`, `crates/lexicon/src/types.rs` (`LexicalReading`),
  `crates/lexicon/src/analysis.rs`, `crates/syntax/src/reading.rs` (`realize`, `admit`).
- `crates/english/src/judgment.rs` (`Judgment::scope`), `crates/xtask/src/english.rs`
  (`stated_items`).
- `docs/design/lexicon.md` §3, §6; `docs/contexts/oracle-english/CONTEXT.md`;
  `docs/oracle-style-guide.md` §5.
