# Architecture

A rules engine should grow with the number of distinct constructions in Magic,
not with the number of cards. A card is a composition of constructions, and
each construction is implemented once, as shared rules machinery. No card has
code of its own. sylex reads a card's Oracle text into such a composition, plays
it, and writes it back as the same Oracle text.

## The layers

```
Oracle text
    ⇅   English      parse / realize
Reading
    ⇅   spelling     recover / spell       construction grammar, many-to-many
resolved term                              stored; keeps Template and Definition calls
    │   expansion
    ▼
expanded tree                              primitives; read by discourse
    │   lowering     one-way compile
    ▼
core
    │   executed by
    ▼
engine
```

**English** relates Oracle text to Readings, which are grammatical analyses of
English. `sylex_lexicon` does lexical analysis and realization. `sylex_vocabulary`
holds the core English lexicon and the Catalogs of vocabulary the Comprehensive
Rules enumerate. `sylex_syntax` is the data-driven syntax engine, and
`sylex_english` is the Oracle English grammar bound into it. This layer knows
English and Oracle's conventions, not Magic: it parses "exile target creature"
as a verb and its object, and nothing in it knows what exiling does. Its
round-trip laws carry the system's exact round trip at this layer
([English](english.md) §7; the crate split and the Catalog constraint are in
§1–2).

**Spelling** relates Readings to resolved terms. Recovery reads the Packed Forest
and assembles the resolved terms whose spellings match it. Spelling a resolved
term yields a Reading for the English layer to realize. Its data is not kept
apart: every `.syl` declaration carries its `lexical` and `spelling` fields
beside its meaning ([`.syl`](syl.md) §2), so a word's wording
and its meaning are one fact with one source. Spelling coverage is therefore
ingestion coverage: a card recovers when every phrase it prints has a spelling.
[Spelling](spelling.md) is the grammar's design.

**Semantics** is the card language: `.syl` declarations under `plugins/`.
Primitives, Templates, Definitions and rules rows (state-based actions, and
rules such as damage to a player causing that much life loss) live in
`plugins/builtin/`, and cards live in `plugins/canon/`. A Template is printed wording over other
declarations and adds no meaning; a Definition is vocabulary the rules define,
such as a keyword action or ability, checked on its own
([Semantics](semantics.md) §3). There are two levels. The **resolved term**
is a card's `.syl` with each name resolved by expected sort; it keeps its
Template and Definition calls, follows the card's English phrase by phrase, and
is what is stored, recovered and rendered. **Expanding** those calls yields the
**expanded tree** of primitives. The discourse reading reads the expanded tree
in printed order against a Discourse Representation Structure, which resolves
every reference and checks that the card is a faithful reading of its text
([Semantics](semantics.md), from the overview through §4, and
[`.syl`](syl.md)). `sylex_syl` parses `.syl`.

**Lowering** compiles a checked expanded tree, every reference in it resolved,
into core, in one direction only.
It does all the rearrangement a surface-faithful tree leaves undone: it gathers
a spell's targets into its announcement, turns each resolved reference into an
explicit read, and supplies game identity, such as a moved card becoming a new
object [CR#400.7].

**Core** is the engine's AST, the shapes the rules systems execute.
`sylex_core` holds the primitives, and `sylex_card` packages them into playable
cards and card faces: a face's characteristics together with its lowered
abilities, so lowering targets both. `sylex_card` depends on core, never the
reverse. Every
binding in core is declared and every reference is a register read, so core
resolves nothing; [Core](core.md) is its
contract.

**Engine.** `sylex_engine` holds game state and the rules systems (the stack,
the layers, combat and the rest) and executes core. It exposes player decisions
as explicit decision points, which its consumers drive: the terminal client
`sylex_tui` and the matchup suite `sylex_matchups`
([Engine](engine.md), law 2).

## Authority, canon and artifact

Three forms have three roles, and the roles never trade places.

- **Oracle text is the authoritative input.** For an existing Magic card, the
  rules text is the source. The other printed characteristics (name, mana cost,
  type line, power and toughness) arrive beside it as structured data. When a
  card's resolved term and its text disagree, the term is wrong.
- **The resolved term is the canonical meaning.** Recovery compiles the text
  into it once; spelling renders it, and lowering compiles its expansion.
  Storing it caches a compilation worth doing once across some thirty thousand
  cards. It does not make the stored file a second authority.
- **Core is a compiled artifact.** It is cached at most, keyed by content, and
  never edited or read as a source.

Why: one meaning needs one source. If core could be written by hand, a card
would have two meanings that must agree and no rule for which one wins.

There is one narrow exception. Engine play strategies are written directly in
core's expression vocabulary as plain data. They are engine configuration, not
card content, and sit outside this pipeline.

The authoring interface for custom content stays open. A plugin may accept
prose, `.syl`, or both, as long as every path produces the same checked resolved
term before rendering or lowering.

## Two-way above, one-way below

English, spelling and semantics are related in both directions because Oracle
text is both the input and a product: recovery reads cards, and rendering
writes card text for the client. Each direction checks the other.

Lowering is a function in one direction. Core has no meaning of its own: a card
means the image of its expanded tree under lowering. A reverse map from core
would be a second translation that must agree with the first, and its coverage
would shrink each time core's shape moves away from semantics, which it is meant
to do (see [The divergence trajectory](#the-divergence-trajectory)). The
lowering crate is instead the ledger of where the two shapes differ: each place
where core departs from the expanded tree's shape carries its justification
there, in place.

## Spelling is a construction grammar

Each Construction — a Primitive, Template or Definition with its printed
wording — pairs one fixed fragment of English structure with the call it
spells, the fragment's Holes linked to the call's parameters. Constructions
combine by substitution only, never by adjunction. Two closed rules cover two
places where English and the term part: a passive prints the complement its
Action's frame promotes as its context's subject, and a dependent printed in the
first sentence of a run of consecutive sentences, which a Hole may hold, may
govern the whole run. Three closed extensions cover more: optional and list
Holes, a Tail printed away from its Hole, and Factoring of what a coordination's arms share
([Spelling](spelling.md) §2–3). What these do not yet cover is open
([Spelling](spelling.md) §10). A derivation builds both sides at once, so the
same grammar recovers a resolved term from a Reading and spells a resolved term
as English, and the resolved term is that derivation: nothing records it beside the term
([Spelling](spelling.md) §4). It is not a bijection: several terms may word
identically, and one sentence has several grammatical Readings, which the
parser keeps rather than choosing ([English](english.md) §4). Meaning
resolves the ambiguity, never rank or rule order: recovery expands each
candidate and reads its discourse, and a candidate the checker or discourse
refuses drops out. The goal is exactly one surviving candidate per card; a card
with more is reported, never stored as a choice.

## Exact round trip

The goal is an exact round trip over the analyzed Oracle text, the text English
analysis produces with reminder text removed
([Lexicon](lexicon.md) §1): it goes
to its resolved term and back to the identical analyzed text. That is what
faithful means here, and it is the easy, visible demonstration that the system
works: a card whose rendering is not its own text has a wrong term, found
without playing a game. The identical text comes back because the resolved term
keeps the Template call that carries each wording and writes each list in its
printed order, so two wordings with the same meaning differ in which Template a
card calls or in the order of a list's elements, never in a flag stored beside
the term ([Spelling](spelling.md) §4, §6).

## Design principles

1. **Name things what they are.** Every crate, layer and concept carries its
   literal role; there are no coined names. Why: a reader new to the code finds
   a thing by what it does. A name that has stopped fitting also signals that
   the thing's role has changed. When that happens, rename the thing rather than
   reinterpret the name.
2. **The canonical form contains only what round-trips, and stays legible.**
   Anything one direction of translation would have to invent and the other
   would discard is decoration, and it is not stored: named binder variables,
   labels, files that map variants to author names, a parse trace, a redundant
   expansion, or a record of how an author happened to write an equivalent
   term. What the printed text chooses is not decoration (see
   [Exact round trip](#exact-round-trip)). The result is small enough that
   someone who knows the card, rather than the compiler, can read its resolved
   term and see where a translation went wrong; that debuggability is part of
   the representation's contract. Core is the opposite case: it is generated,
   so its verbosity costs nothing, and a printer is its debugging aid. Why: what
   is stored but not derivable from the text drifts, and nothing can check it.
3. **Sugar never replaces structure.** A Template is printed wording only. It
   expands into the expressions its parts would be written as and adds no
   meaning ([Semantics](semantics.md) §3). Its expansion carries the
   complete rules meaning, so a card's expanded tree means the same whichever
   Templates spell it. Why: a convenience that carried meaning of its own would be a
   second, hidden semantics.
4. **Productive compositions are machinery, not vocabulary.** A pattern that
   composes with an open class gets one mechanism; only a closed idiom gets a
   word. One Action per verb serves every clause context, whether instruction,
   trigger, replacement, prohibition or lookback
   ([Semantics](semantics.md) §4.1). The English grammar derives a verb's
   passive and gapped forms from its one Subcategorization Frame. "Dies" is a
   Template over the one zone-change Action [CR#700.4]. Why: a word for every
   combination multiplies as verbs times contexts, and the copies drift apart.

   The primitives under that machinery stay few and general. A primitive is
   judged by how it lowers and by its algebra, never by surface phrasing, and a
   meta construct that appears only inside an expansion is legitimate. A
   cross-sort phrase is a plain disjunction, not a dedicated constructor: "any
   target" [CR#115.4] is a Template that expands to a disjunction over
   creatures, planeswalkers, battles and players. The cost is overgeneration:
   the primitives can build terms no card prints. That is tolerated, because the
   spelling boundary refuses a term that has no English, as an authoring-time
   error rather than unsoundness in the model.
5. **Check the open corpus; test the closed mapping.** The mistakes worth ruling
   out (a reference resolved to the wrong antecedent, a sort that does not fit,
   a card that misreads its text) are made in recovery and authoring, over a
   corpus that keeps bringing unanticipated input. So every card is checked, at
   the semantic level, against its text
   ([Semantics](semantics.md) §1, item 3). Lowering is a closed,
   mechanical mapping written once, and it gets tests over its finite set of
   shapes. Core is generated and carries no proofs: an error in it is a
   lowering bug, caught by lowering's tests and the engine's.

## The divergence trajectory

Semantics and core began as one grammar and are meant to drift apart in named
directions. Each form spends its explicitness on what its consumer needs.

- **Semantics drifts toward English constructions.** A card's resolved term
  follows its English phrase by phrase, and nothing is hoisted
  ([Semantics](semantics.md) §1, item 1). What English leaves implicit,
  such as bracketing and attachment, is made explicit. Reference stays
  anaphoric, because English is anaphoric and spelling needs the anaphors: "it"
  and "that creature" are written as such and resolved by the discourse reading
  ([Semantics](semantics.md) §2). The drift lands in the data
  (Templates, Definitions and spellings) while the primitives stay generic.
- **Core drifts toward evaluation shape.** Reference becomes explicit. Every
  binding is a declared parameter or a numbered definition and every read is a
  register read, while grouping stays only as structured as evaluation needs
  ([Core](core.md)).
- **Reference is resolved once, on the way down.** The discourse reading
  resolves each reference in the expanded tree, lowering turns each resolution
  into a register read, and the engine never searches for an antecedent. Why:
  one resolution at compile time, with a diagnostic per card, replaces a search
  the engine would otherwise repeat on every resolution and could get wrong
  without anyone noticing.

## Rejected alternatives

- **Code per card.** It scales with the card count, and each card's code is a
  separate reading of the rules that nothing checks against its text.
- **A stored intermediate form between semantics and core.** It would be a third
  authority. The only intermediate states are transient, such as the discourse
  structure and lowering's working state, and none is serialized.
