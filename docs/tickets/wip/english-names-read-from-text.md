---
needs: [english-shortened-name-without-attestation]
---

# Names a card's text introduces are read from the text

## Why

`crates/vocabulary/catalogs/introduced_names.ron` (102 names on 125 faces) records facts about
individual cards before parsing: each name, and the cards on which it stands. A card's text sets
the name of a token it creates [CR#111.4], so the name is a string read from that text (*Rinoa
Heartilly*'s "create Angelo, a legendary 1/1 green and white Dog creature token"). The list goes.

The list also misses names. Nine in-scope cards create "a tapped Planet land token named New
Planet" (*Genesis Effect*, *Artificial Anomaly*, ...). "New Planet" is not listed, and each line's
only Reading today reads "named New Planet" as a participle premodifying the nominal "New Planet"
(adjective *new* and subtype *Planet*).

Evidence (gitignored), all from `data/mtgjson/AllPrintings.sqlite` (in-scope faces, reminder
text removed): `docs/evidence/names-read-from-text/`.

Every listed name is introduced by one of these constructions (`classify.txt`):

| Construction | Names | Example |
|---|---|---|
| `create NAME, a … token` (the description a Supplement) | 44 | *Rinoa Heartilly* (Angelo) |
| `… token(s) … named NAME` | 46 | *Peer Review* (Cadet, 11 cards) |
| `becomes / is / she's … named NAME` (a permanent the text changes) | 7 | *The Curse of Fenric* (Fenric), *Princess Yue* (Moon) |
| `except its name is NAME` | 1 | *Mishra, Eminent One* (Mishra's Warform) |
| `create a NAME token`, NAME a card's name [CR#111.11] | 4 | *Disa the Restless* (Tarmogoyf) |

*The Eleventh Hour*'s "create a token that's a copy of target creature, except it's a legendary
Alien named Prisoner Zero" is counted in the `… token(s) … named NAME` row.

A listed name is also used after its introduction, outside the position that introduces it:

- After *named*, where it is not introducing anything: 11 uses. Six are on the introducing line,
  before or after the introduction (*Rufus Shinra*'s "if you don't control a creature named
  Darkstar, create Darkstar, …"). Four are on another line of the same face (*The Curse of
  Fenric* III, *Volo, Itinerant Scholar*, *Jungle Patrol*, *Rite of the Raging Storm*). One is
  on a face that never introduces the name (*Hand of Vecna*'s "a creature you control named
  Vecna").
- As an ordinary Proper Name: 13 uses on 12 faces, every one on the introducing line. Eleven
  are inside a quoted ability the token has or grants, naming the token [CR#201.5,201.5a]
  (*Galactus*, *Smaug*, *Beau's power*, "Sacrifice Rock"). Two are in a later sentence of the
  same ability (*Kari Zev, Skyship Raider*'s "Ragavan enters tapped and attacking", *Stangg, Echo
  Warrior*'s "attached to Stangg Twin"). *Rinoa Heartilly*'s second line opens with "Angelo
  Cannon —", a Flavor Word, which is not a use of the name.

Today the lexicon loads each listed name as a card-scoped Catalog Lexeme (`introduced_name:`).
`Lexicon::stands_in` admits it only on its listed cards, and the grammar reads it through
`Name -> Lex(Catalog)` wherever a Name stands (`ProperName`, `NamePremodifier`, `ProperPossessor`,
`NameAsComplement`). (With `introduced_names.ron` and `cited_names.ron` both emptied, 134 of the
299 distinct lines in `inputs.bin` lose every Reading: `parse-lists.txt` against
`parse-nolists.txt`.)

What a read name needs was measured by emulation. `longest.py` reads, at each position that
introduces a name (after *named*, the Object of `create … , a … token`, after `except its name
is`), the longest run of words that starts with a capital. The run continues over capitalized
words, over lowercase *of*, *the* and *to*, and over ", " when a capital or *the* follows. It stops
before *and*. Three refusals keep card names out: a run that a card name spells is refused, a run
that is a proper prefix of a card name printed at that position is refused, and ", " does not
continue a run after a part that a card name spells. Over every in-scope face this rule finds
exactly the 98 listed names other than the four card names of `create a NAME token`, on exactly
their listed faces, plus New Planet, and nothing else (`longest_pool.ron`; the emulation also reads
before *expansion* and after *completed*, which `english-set-names-read-from-text` and
`lexicon-dungeon-names` own separately).

The refusals are each grounded in a real card:

- *Helm of Kaldra* and *Shield of Kaldra*: "Equipment named Helm of Kaldra, Sword of Kaldra, and
  Shield of Kaldra".
- *Urza, Lord Protector*: "an artifact named The Mightstone and Weakstone".
- *Skoa, Embermage*: "Discard another card named Skoa, Embermage, Sacrifice two Mountains".
- *Vanille, Cheerful l'Cie*: "a creature named Fang, Fearless l'Cie".

Loaded as card-scoped names that stand at every Name position on their faces, with the four card
names kept, the emulated names give the same derivation count as the lists on 290 of the 299
lines (`parse-longest.txt`). The other nine are the New Planet lines, which gain a Reading with
"New Planet" as the complement of *named* beside today's misreading. An emulation that proposed
every end of a run, not only the longest, added Readings on four more lines (*Boris Devilboon*'s "named Minor Demon" also read as
the Name *Minor* premodifying *Demon*; `parse-shape.txt`).

The Flavor Word is the precedent for a span no list holds. `flavor_word_matches`
(`crates/lexicon/src/analysis.rs`) proposes it by shape: from a line start, past a bullet, up to
the first " —" that is not after chapter numerals. Analysis keeps it only if it is title-cased and
no label registered in the Card Scope spells it (`stands_in`). The grammar admits
`Lex(FlavorWord)` only at the head of an ability or mode (`FlavorLabeledAbility`,
`FlavorLabeledModalHeader`, `LabeledCostedMode`). Its value is the string
(`LexicalReading::FlavorWord { label }`), not a Lexeme. A name fits the same route with three
differences. Its end is not fixed by punctuation, so the shape rule has to choose it. It stands
in the middle of a sentence at several positions. And it is used again after its introduction.

## Design

- A name a card's text introduces (the name of a token it creates [CR#111.4], or of a permanent
  or copy it changes) is a string read from the text. No catalog lists it, and nothing keyed by
  card holds it.
- `sylex_lexicon` proposes one longest title run per capitalized start at word boundaries (using the
  shape rule and three card-name refusals above, with the `", "` card-name refusal checking
  `NameForm::Full` only so a `Shortened` form such as *Icingdeath* never truncates `"Icingdeath,
  Frost Tongue"`), computing phonological `Edges` for genitive `'s` and `a`/`an`, and `english.fug`
  admits the read-name leaf only at the introducing positions: the complement of *named*, the Object
  of *create* before a token-description Supplement, and the complement of `its name is` in a copy's
  exception.
- Reuse is unit-local in two passes: Pass 1 harvests names licensed by completed introducing
  constituents on the unit (even when a later sentence in the unit cannot yet parse), and Pass 2
  admits those names across `Name` positions of that unit, verifying in both Pass 2 and
  `CompiledGrammar::admit` (`docs/design/english.md` §7, round-trip law 2) that any non-licensing
  read-name leaf co-occurs with a licensing introduction of that exact name in the same `Reading`.
- `<OtherCardName>` premodifying the head *token(s)* inside a modifiable `Nominal` in the Object of
  *create* (*Disa the Restless*'s `"create a Tarmogoyf token"`, *Mutable Explorer*'s `"create a
  tapped Mutavault token"` [CR#111.11]) opts in to `OtherCardName()` (`docs/design/lexicon.md` §6),
  refused where a predefined token or a subtype spells the name (*Blood Fountain*'s `"create a Blood
  token"` keeps its single Reading).
- A read name is a Proper Name, unmarked for Number (`lexicon.md` §6). Its genitive is formed as a
  card name's is (*Beau's power*).
- `introduced_names.ron`, the `IntroducedName` schema type, its loader path and the
  `introduced_name:` Lexemes are deleted.

## Done when

- `introduced_names.ron` is deleted, with its loader path, the `IntroducedName` schema type and its
  README row. No card-scoped Catalog remains for introduced names.
- Every line in `docs/evidence/names-read-from-text/inputs.bin` has the same derivation count as
  in `parse-lists.txt`. The exceptions are the nine New Planet lines, which have a Reading with
  "New Planet" as the complement of *named*.
- Grammar or judgment tests, each on the real card's line, pin one example per construction in
  the table. They also pin a reuse after *named* on another line (*The Curse of Fenric* III), one
  with no introduction on the face (*Hand of Vecna*), a reuse in a quoted ability (*The Coming
  of Galactus* IV) and a reuse in a later sentence (*Kari Zev, Skyship Raider*). For each comma
  name (*Voja, Friend to Elves*, *Mileva, the Stalwart*, *Ashaya, the Awoken World*) the whole
  name is the Object or complement.
- Negative tests on the real lines pin how the chosen end rule keeps card names out:
  *Helm of Kaldra*, *Urza, Lord Protector*, *Skoa, Embermage* and *Vanille, Cheerful l'Cie* read
  no part of the card names as a read name, and *Blood Fountain*'s "create a Blood token" keeps
  its single Reading.
- Judgments that assert a listed name standing where nothing licenses it are rewritten onto the
  card's line or retired, each with its reason: "Create Angelo." in `named_token_apposition.txt`
  and the bare `Angelo`, `create Angelo`, `Urami` and `create Marit Lage` entries in
  `vocabulary_batch_2.txt`. So are the tests built on the lists: `names.rs`
  `every_introducing_card_admits_its_introduced_names` and the `introduced_name` cases,
  `loader.rs`, `crates/lexicon/tests/analysis.rs` (Marit Lage) and
  `crates/syntax/tests/card_names.rs`
  (`admission_refuses_an_introduced_name_off_the_cards_that_introduce_it`).
  Admission still refuses a read name off its licensing position or scope.
- `docs/design/lexicon.md` and the glossary state the design:
  - §2's "The one open slot is the Flavor Word" names read names among the open slots.
  - §6's "A name a card gives an object, or a vote option, spelled like another card's name is
    listed as that card's Introduced Name or vote option (*create a Tarmogoyf token*, …)" no
    longer says such a name is listed, and §6's list of positions that opt in to `OtherCardName()`
    includes `<OtherCardName>` premodifying *token(s)* in the Object of *create* (refused where a
    predefined token or subtype spells the name).
  - In §6's first paragraph, "never one that other vocabulary in scope on that card already
    spells: a token name or Introduced Name always" no longer refers to a listed Introduced
    Name.
  - The glossary's Introduced Name drops "It is in scope only on that card" and the *Tarmogoyf*
    example. Flavor Word is no longer "the one open lexical slot". Card Scope drops "an Introduced
    Name … only where it holds one of its cards".
  - `crates/vocabulary/README.md` no longer lists `introduced_names.ron`.
- `cargo xtask english strata --check` passes, and the census changes only on the New Planet
  units. `cargo test` and `cargo clippy --tests -- -D warnings` pass for `sylex_lexicon`,
  `sylex_vocabulary`, `sylex_syntax`, `sylex_english` and `xtask`.

## Not in scope

- `vote_options.ron` (`english-vote-options-read-from-text`), `anchor_words.ron`
  (`english-anchor-words-read-from-text`), and the expansion entries of `cited_names.ron`
  (`english-set-names-read-from-text`).
- The participle premodifier that misreads "named New Planet" today
  (`Premodification:BareNominal/Verb/attach`).

## Pointers

- `crates/lexicon/src/analysis.rs` (`flavor_word_matches`, `stands_in`, `word_stands_in`,
  `analyze_source_scoped`), `crates/lexicon/src/scope.rs`.
- `crates/english/grammar/english.fug`: `Name`, `NameComplement`, `NamedParameter`,
  `ProperName`, `NamePremodifier`, `ProperPossessor`, `OtherCardNameComplement`,
  `NounPhraseSupplement`, `FlavorLabeledAbility`.
- `crates/vocabulary/src/lib.rs` (`introduced_names`),
  `crates/vocabulary/src/schema.rs`, `crates/english/src/profile.rs` (`introduced_name` skeleton
  class), `crates/syntax/src/reading.rs` (admission's `stands_in`).
- `docs/design/lexicon.md` §2, §6; `docs/contexts/oracle-english/CONTEXT.md`.
- Evidence (gitignored), `docs/evidence/names-read-from-text/`: `classify.py` and `classify.txt`
  (the table), `longest.py` and `longest_pool.ron` (the shape rule), `compare.py` with the
  `parse-*.txt` runs, `pool_named.py` (longest runs at every licensing position).
