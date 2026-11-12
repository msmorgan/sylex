---
needs: [lexicon-dungeon-names]
---

# Names a card's text introduces or cites are read from the text

## Why

`crates/vocabulary/catalogs/introduced_names.ron` (102 names on 125 faces) and the expansion
entries of `cited_names.ron` (3 names on 3 cards) record facts about individual cards before
parsing: each name, and the cards on which it stands. A card's text sets the name of a token it
creates [CR#111.4], so the name is a string read from that text (*Rinoa Heartilly*'s "create
Angelo, a legendary 1/1 green and white Dog creature token"). The same holds for an expansion the
text cites (*Golgothian Sylex*'s "originally printed in the Antiquities expansion"). Both lists go.

The lists also miss names. Nine in-scope cards create "a tapped Planet land token named New
Planet" (*Genesis Effect*, *Artificial Anomaly*, ...). "New Planet" is not listed, and each line's
only Reading today reads "named New Planet" as a participle premodifying the nominal "New Planet"
(adjective *new* and subtype *Planet*).

Evidence (gitignored), all from `data/mtgjson/AllPrintings.sqlite` (in-scope faces, reminder
text removed): `docs/evidence/names-read-from-text/`.

Every listed name is introduced or cited by one of these constructions (`classify.txt`):

| Construction | Names | Example |
|---|---|---|
| `create NAME, a … token` (the description a Supplement) | 44 | *Rinoa Heartilly* (Angelo) |
| `… token(s) … named NAME` | 46 | *Peer Review* (Cadet, 11 cards) |
| `becomes / is / she's … named NAME` (a permanent the text changes) | 7 | *The Curse of Fenric* (Fenric), *Princess Yue* (Moon) |
| `except its name is NAME` | 1 | *Mishra, Eminent One* (Mishra's Warform) |
| `create a NAME token`, NAME a card's name [CR#111.11] | 4 | *Disa the Restless* (Tarmogoyf) |
| `originally printed in the NAME expansion` | 3 | *City in a Bottle* (Arabian Nights) |

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

Today the lexicon loads each listed name as a card-scoped Catalog Lexeme (`introduced_name:`,
`cited_name:`). `Lexicon::stands_in` admits it only on its listed cards, and the grammar reads it
through `Name -> Lex(Catalog)` wherever a Name stands (`ProperName`, `NamePremodifier`,
`ProperPossessor`, `NameAsComplement`). With both lists emptied, 134 of the 299 distinct lines on
these cards lose every Reading (`parse-lists.txt` against `parse-nolists.txt`).

What a read name needs was measured by emulation. `longest.py` reads, at each position that
introduces a name (after *named*, the Object of `create … , a … token`, after `except its name
is`, before `expansion`), the longest run of words that starts with a capital. The run continues
over capitalized words, over lowercase *of*, *the* and *to*, and over ", " when a capital or
*the* follows. It stops before *and*. Three refusals keep card names out: a run that a card name
spells is refused, a run that is a proper prefix of a card name printed at that position is
refused, and ", " does not continue a run after a part that a card name spells. Over every
in-scope face this rule finds exactly the 98 listed names other than the four card names of
`create a NAME token`, on exactly their listed faces, plus the 3 expansions and New Planet, and
nothing else (`longest_pool.ron`; the emulation also reads after *completed*, which finds only
Tomb of Annihilation, a dungeon name).

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
`Lex(FlavorWord)` only at the head of an ability or mode (`FlavorLabelledAbility`,
`FlavorLabelledModalHeader`, `LabelledCostedMode`). Its value is the string
(`LexicalReading::FlavorWord { label }`), not a Lexeme. A name fits the same route with three
differences. Its end is not fixed by punctuation, so the shape rule has to choose it. It stands
in the middle of a sentence at several positions. And it is used again after its introduction.

## Design

- A name a card's text introduces (the name of a token it creates [CR#111.4], or of a permanent
  or copy it changes) and an expansion name the text cites are strings read from the text. No
  catalog lists them, and nothing keyed by card holds them.
- A name is read where the text introduces or cites it: the complement of *named*, the
  Object of *create* before a token-description Supplement, the complement of `its name is` in a
  copy's exception, and the premodifier of the noun *expansion*. These are the in-scope
  positions of the table.
- A read name is a Proper Name, unmarked for Number (`lexicon.md` §6). Its genitive is formed as a
  card name's is (*Beau's power*).
- `introduced_names.ron`, `cited_names.ron`, the `IntroducedName` schema type, the
  `cited_names` loader path and the `introduced_name:` and `cited_name:` Lexemes are deleted.
  `ScopedName` stays for `vote_options.ron`.

## Open questions

- **How the read span is proposed and licensed.**
  (a) Analysis proposes a run by shape at every capitalized word, and the grammar admits the
  read-name value only in the licensing productions, as `Lex(FlavorWord)` is admitted. Analysis
  stays free of grammar (`lexicon.md` §2), and the license lives where the position is defined.
  This needs new productions for the `create NAME, a … token` Supplement and for `its name is`,
  because today both go through the general `NounPhraseSupplement` and `ProperName`.
  (b) Analysis proposes a run only after fixed context words (*named*, *create*, `its name is`,
  `the … expansion`), which is what `longest.py` emulates. This needs no grammar change, but
  analysis has to know English words and their order, which `lexicon.md` §2 keeps out of it.
  Recommendation: (a). Confidence: medium.
- **Where a read name ends.** (a) One run per start, the longest the shape rule above allows,
  with its three refusals. This reproduces the lists exactly over the pool. (b) Every end of the
  run, with the chart choosing among them. This adds Readings on four lines, such as *Minor*
  beside *Minor Demon*, and a second Reading of *Nissa, Sage Animist*'s line with the run
  "Ashaya, the Awoken" and "World" as a depictive supertype adjective. Recommendation: (a).
  Confidence: medium-high. The run's word classes (*of*, *the*, *to*; no *and*; no digits or
  other lowercase words) are drawn from the names in the pool, and a new card can break them.
  Under (a) such a card loses its Reading and the census reports it.
- **Where a read name stands after its introduction.** Every in-scope reuse other than after
  *named* is on the introducing line, and *named* licenses a read name by itself (*Hand of
  Vecna*).
  (a) Unit-local: the names a unit's first parse reads at licensing positions stand at every
  Name position of that unit, and the unit is parsed a second time. This covers all 13 uses.
  (b) Face-wide: the same, with the names read anywhere on the face standing on every unit of
  the face. This covers reuse across lines, which no in-scope card prints.
  (c) Licensing positions only. This is one pass, but the 12 lines with a reuse lose their
  Reading.
  Recommendation: (a), the smallest scope the cards need. Confidence: medium.
- **`create a NAME token`.** A token created by name with no other characteristics takes the
  characteristics of the Oracle card of that name [CR#111.11]. So NAME is another card's name,
  not a name the text introduces: *Tarmogoyf* (*Disa the Restless*, *Tarmogoyf Nest*),
  *Gingerbrute* (*Ginger, Queen of Sweets*), *Mutavault* (*Mutable Explorer*), *Spellgorger
  Weird* (*Ral and the Implicit Maze*).
  (a) A fourth position opts in to `OtherCardName()` (`lexicon.md` §6): the name premodifying
  the head *token* in the Object of *create*. It is refused where a predefined token or a subtype
  spells the name, because [CR#111.11] excludes predefined tokens and *Blood* is both a card and
  a predefined token (*Blood Fountain*'s "create a Blood token" has one Reading today).
  (b) Read it as a read name, with *token* as a fifth licensing position. A read name there
  would have to stand where a card name spells it, and it carries no link to the card the rule
  names.
  Recommendation: (a). Confidence: high on the classification, medium on the position's extent.

## Done when

- `introduced_names.ron` and `cited_names.ron` are deleted, with their loader paths, the
  `IntroducedName` schema type and their README rows. No card-scoped Catalog remains for these
  names.
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
- `docs/decisions/lexicon.md` and the glossary state the design:
  - §2's "The one open slot is the Flavor Word" names read names as the second open slot.
  - §6's "A name a card gives an object, or a vote option, spelled like another card's name is
    listed as that card's Introduced Name or vote option (*create a Tarmogoyf token*, …)" no
    longer says such a name is listed. §6's list of positions that opt in to `OtherCardName()`
    follows the ruling on `create a NAME token`.
  - In §6's first paragraph, "never one that other vocabulary in scope on that card already
    spells: a token name or Introduced Name always" no longer refers to a listed Introduced
    Name.
  - The glossary's Introduced Name drops "It is in scope only on that card" and the *Tarmogoyf*
    example. Flavor Word drops "the one open lexical slot". Card Scope drops "an Introduced
    Name … only where it holds one of its cards". Proper Name drops "a cited name".
  - `crates/vocabulary/README.md` no longer lists the two catalogs as hand-curated.
- `cargo xtask english strata --check` passes, and the census changes only on the New Planet
  units. `cargo test` and `cargo clippy --tests -- -D warnings` pass for `sylex_lexicon`,
  `sylex_vocabulary`, `sylex_syntax`, `sylex_english` and `xtask`.

## Not in scope

- `vote_options.ron` and `anchor_words.ron`, also card-scoped lists, which have no ruling.
- The participle premodifier that misreads "named New Planet" today
  (`Premodification:BareNominal/Verb/attach`).

## Pointers

- `crates/lexicon/src/analysis.rs` (`flavor_word_matches`, `stands_in`, `word_stands_in`,
  `analyze_source_scoped`), `crates/lexicon/src/scope.rs`.
- `crates/english/grammar/english.fug`: `Name`, `NameComplement`, `NamedParameter`,
  `ProperName`, `NamePremodifier`, `ProperPossessor`, `OtherCardNameComplement`,
  `NounPhraseSupplement`, `FlavorLabelledAbility`.
- `crates/vocabulary/src/lib.rs` (`introduced_names`, `scoped_name_catalogs`),
  `crates/vocabulary/src/schema.rs`, `crates/english/src/profile.rs` (`introduced_name` skeleton
  class), `crates/syntax/src/reading.rs` (admission's `stands_in`).
- `docs/decisions/lexicon.md` §2, §6; `docs/contexts/oracle-english/CONTEXT.md`.
- Evidence (gitignored), `docs/evidence/names-read-from-text/`: `classify.py` and `classify.txt`
  (the table), `longest.py` and `longest_pool.ron` (the shape rule), `compare.py` with the
  `parse-*.txt` runs, `pool_named.py` (longest runs at every licensing position).
