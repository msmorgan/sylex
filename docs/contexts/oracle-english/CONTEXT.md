# Oracle English

The vocabulary of the English grammar, its parser and its corpus measurement.
Game concepts named here keep their [Game Model](../game-model/CONTEXT.md)
meanings.

## Authority

Oracle text is a controlled register (`docs/oracle-style-guide.md`). *The
Cambridge Grammar of the English Language* (CGEL) supplies the grammatical
terminology and constituent structure, never a mandate to admit literary
constructions Oracle does not use. CGEL terms (Noun Phrase, Complement,
Adjunct, Determinative, Gap and the rest) and Comprehensive Rules terms keep
their standard meanings and are not redefined here. Predicate, Subject and
Object here are CGEL's grammatical functions, not the Game Model terms.

## Grammar and parsing

**Construction**:
Spelling's pairing of a syntactic pattern with its Game Model meaning. The
grammar is syntax only and has none.
_Avoid_: Construction for a Schema or Production

**Schema**:
A declarative pattern in the grammar data file that compiles into one or more
Productions.

**Production**:
A compiled grammar rule forming a Category from an ordered pattern of
Categories or terminals.

**Derivation**:
One way a Constituent is built from Productions. Two Derivations of the same
Reading are not an Ambiguity.

**Reading**:
One grammatical analysis of an expression: one whole tree from the Packed
Forest.

**Reading Normal Form**:
A form of a Reading that identifies only the reviewed structural alternatives
as one analysis, so that two sets of Readings can be compared.

**Admission**:
The grammar's acceptance of a Reading at a Category. Preference never affects
it.

**Packing**:
Grouping alternatives that every parent treats alike, so the group is carried
as one item.

**Packed Forest**:
The shared representation of all Readings of an expression, each Constituent
occurring once per Category, span and grammatical summary.

**Preference**:
A ranking over admitted Readings. It orders them and never adds or removes
one.

**Ranking Key**:
The Preference that orders Readings: Attachment Class, Right Association,
Production Cost and a tie-break, compared lexicographically, lower first.

**Production Cost**:
A nonnegative weight on a Production, summed over a Reading as one component
of its Ranking Key.

**Attachment Class**:
A preference on where a kind of unselected dependent attaches, stated by host
kind and grammatical function.

**Right Association**:
The preference for attaching an unselected dependent to the nearest eligible
host, measured by the host's length in words.

**Realization**:
Producing the surface text of a Reading.

**Atom**:
One annotated unit of a realized surface: a word, an opening or closing
delimiter, a symbol or a line boundary.

## Spelling

**Hole**:
A place in a Construction's wording that the wording of one parameter's argument fills.

**Tail**:
Part of a Construction's wording printed away from the rest, at the edge of a phrase that
encloses the Construction's Hole (*for each creature in your party*, after its clause's
duration).

**Factoring**:
Printing once a phrase that a coordination binds as a parameter and that its arms each read
through a placeholder, the phrase standing in different roles in different arms (*this
creature blocks or becomes blocked*).

## Corpus and census

**Corpus Unit**:
The unit the Census counts: one Type Line, or one paragraph line of a Card
Face's rules text without its reminder text.

**Census**:
The measurement of every Corpus Unit against the grammar. A unit is read when
it has an admitted Reading, both Round-trip Laws hold, and it is not
known-wrong-only: a Judgment shows none of its Readings right.

**Round-trip Laws**:
Realizing a Reading of a text yields that text, and parsing the realization of
a grammatical Reading yields that Reading.

**Segment**:
A top-level stretch of a Corpus Unit, such as one sentence, an activation
cost or a label before its dash.

**Surplus**:
How far a Corpus Unit is from one Reading: the sum over its Segments of each
Segment's Derivation count minus one.

**Complexity Stratum**:
One of eight cumulative levels, 0 to 7, of syntactic complexity. Each owns
some Syntactic Families, and a Corpus Unit belongs to the highest Stratum of
any Family it contains.

**Completed Stratum**:
The highest Complexity Stratum at or below which every Corpus Unit that is not
a Deferred Unit is read.

**Deferred Unit**:
A Corpus Unit at or below the Completed Stratum left unread on purpose, listed
with the ticket that owns it.

**Syntactic Family**:
A named syntactic pattern, such as a keyword line or a bare relative, that a
Complexity Stratum owns.

**Syntactic Skeleton**:
A Corpus Unit with each open-class Catalog item replaced by a placeholder
naming its Catalog.

**Editorial Correction**:
A recorded replacement for part of a face's Oracle text that the census reads
instead. It is only for a data error against the printed card, such as a
dropped character or wrong symbol, or for ungrammatical English, and never for
a template difference.
_Avoid_: erratum (an official Oracle change)

**Judgment**:
A hand-written assertion that an expression is or is not grammatical,
optionally with its endorsed analysis. Judgments are the tracked linguistic
test evidence.

## Lexicon

**Lexeme**:
A lexical item underlying its Word Forms. It may span several orthographic
words, as *mana value* does.

**Adjacency Agreement**:
The selection of a word's form by a property of the word next to it, through
a feature of its Feature Bundle that the context fixes: the indefinite
article's by the Onset of the word after it (*a creature*, *an artifact*),
*non*'s by how the host after it takes a prefix (*nonland*, *non-Vampire*), and
the genitive's by the host before it: an apostrophe alone after a Plural noun
form ending in *s* (*players'*, *Thalakos'*), and *'s* after any other form
(*player's*, *Rats's*, *Spawn's*; CGEL, Ch. 18 §4.2 [35i], pp. 1595–1596). No
form is a free choice, and each has one spelling.
_Avoid_: Spelling Variant

**Closed Word**:
A core closed-class Lexeme that the grammar names individually.

**Lexical Class**:
A declared group of Lexemes eligible for the same grammatical slot.

**Lexical Environment**:
The declared vocabulary and frames available for analyzing an expression.

**Provenance**:
Where a lexical value comes from, part of its identity. It has three kinds:
**Core**, ordinary English declared in `sylex_vocabulary`; **Catalog**, game
vocabulary; and **Symbol**, notation symbols.

**Onset**:
Whether a word begins with a consonant or vowel sound. The indefinite article
agrees with the Onset of the word after it (Adjacency Agreement).

## Oracle text

**Catalog**:
An inventory of game vocabulary that the Comprehensive Rules define by
enumeration, such as card types and subtypes [CR#205.2a,205.3], keyword
actions [CR#701.1], keyword abilities [CR#702.1] and ability words
[CR#207.2c], together with card names and other names. Core English is never a
Catalog.

**Proper Name**:
The lexical Category of an opaque name: a card name or Shortened Name, a
predefined token name, an Introduced Name, a vote option, a cited name or an
Anchor Word. The code calls it `Category::Catalog`.
_Avoid_: Catalog for this Category

**Document**:
The top grammatical Category: the rules text of one text box or named face
portion.

**Keyword Line**:
A line of one or more keyword abilities with their Keyword Parameters.

**Keyword Parameter**:
What a keyword ability takes with its name on a Keyword Line, such as an
amount (*bushido 2*), a cost (*equip {2}*) or a Keyword Quality.

**Keyword Phrase**:
A Constituent headed by a keyword ability, with its Keyword Parameters when
supplied.

**Keyword Mention**:
A Keyword Phrase that names a keyword ability without supplying one, and may
omit its parameters (*with flashback*).

**Keyword Quality**:
The quality a keyword ability takes as a parameter, such as *black* in
*protection from black*.

**Bound Keyword Surface**:
A keyword written as a suffix on its Keyword Quality's stem, as *walk* in
*islandwalk* [CR#702.14a].

**Bound Keyword Stem**:
The noun stem a Bound Keyword Surface attaches to, as *island* in
*islandwalk*. It is never an independent noun.

**Keyword Variant**:
A keyword ability the CR defines as a variant of another, as megamorph is of
morph [CR#702.37b]. It is its own Lexeme with its own Keyword Parameters.

**Italic Head**:
The italic run, an Ability Word or Flavor Word, that opens some abilities
before an em dash.

**Ability Word**:
The CR term [CR#207.2c], here also covering ability words printed on
Vintage-legal cards that the CR's list omits.

**Flavor Word**:
The CR term [CR#207.2d]. It is the one open lexical slot: no inventory lists
them, so the italic run is captured verbatim.
_Avoid_: flavor text

**Anchor Word**:
The CR term [CR#614.12c]. It is in scope only on the cards that print it.

**Shortened Name**:
A shortened card name [CR#201.5c]: the Shortened form of the card name's
Lexeme, beside its Full form, in scope only in the text of the face whose name
it shortens.

**Own Name**:
A card's name, full or Shortened, in the text of its own face [CR#201.5]. The
grammar reads it as any other Proper Name.

**Another Card's Name**:
A card's full name in the text of any face but its own. Only a position that
opts in admits it, through the grammar's `OtherCardName()` symbol: the
predicative complement of *named* (*cards named Relentless Rats*) and the
Keyword Parameter of *Partner with* [CR#702.124j].

**Introduced Name**:
A name a card's own text gives an object that is not a card, such as the name
of a token it creates, even where another card has that name (*a Tarmogoyf
token*). It is in scope only on that card.

**Name Number**:
The declared grammatical Number of a name, for agreement.

**Targeting Marker**:
The Determinative *target* that ends the Determiner of a target description
(*two target creatures*). It is distinct from the noun and the verb *target*.

**Symbol**:
The lexical Category and Provenance of notation marks, such as mana and tap
symbols.

**Uncleaved Text**:
A cleave line read without its square-bracket characters, the bracketed words
kept in place [CR#702.148a]. The brackets are restored around those words only
when the line is rebuilt, outside any Reading.
_Avoid_: optional text (the brackets mark removal, not optionality)

**Cleaved Text**:
A cleave line read with its bracketed text removed, as when its cleave cost
was paid [CR#702.148a].

## Frames and dependents

**Subcategorization Frame**:
A head's declared pattern of Complements and fixed markers. Verbs,
prepositions, nouns, adjectives and determinatives declare them.

**Frame Slot**:
One position in a Subcategorization Frame: its grammatical function and the
Category that fills it.

**Frame Family**:
The kind of a Subcategorization Frame, named by its slots' shape, such as
monotransitive or copular.

**Selected Frame**:
The one Subcategorization Frame of its head that a Constituent realizes.

**Valence Alternation**:
A relation between realizations of one frame that the engine derives, such as
passive, Extraction or a Shared Complement. It is never declared per Lexeme.

**Active-only Frame**:
A verb frame with no passive, as for *cost* and *have*.

**Host Category**:
A Category an unselected Preposition Phrase may modify, declared for each
pairing of a preposition with one of its frames.

**Auxiliary Complement**:
The Catenative Complement of an auxiliary verb, as *attacked* in *has
attacked*.

**Amount Complement**:
A Complement realized by a number or variable, as *2* in *scry 2*.

**Cost Complement**:
A Complement realized by a Cost, as *{3}* in *waterbend {3}*.

**Comparative-bounded Amount**:
A number coordinated by *or* with a comparative whose understood standard is
that number, as *3 or less*.

**Measure Noun Phrase**:
A bare slash measure (*+2/+2*) as a Noun Phrase where a head selects one, as
in *gets +2/+2*.

**Position Phrase**:
An ordinal with a *from* phrase naming the end of a zone it counts from
(*second from the top*).

## Coordination and agreement

**Linked Coordination**:
A Coordination with no Coordinator, whose later Conjunct, its Linked
Conjunct, opens with a connective adverb (*Draw a card, then discard a card*).

**Complement Cluster**:
Parallel sequences of a verb's Complements, optionally ending in an Adjunct,
coordinated after the shared verb (*put one into your hand and the rest into
your graveyard*).

**Adjunct Cluster**:
Parallel sequences of Preposition Phrase Adjuncts, coordinated after a verb
with no Complement.

**Word-Part Coordination**:
A Coordination of Compound Nouns' first bases sharing the last Conjunct's head
word, as in *a +1/+1, first strike, or vigilance counter*.

**Medial Remnant**:
A medial Conjunct whose initial element is elided and recovered from the first
Conjunct (*is enchanted, equipped, and has a counter on it*).

**Conjunct Number**:
The Number of each Conjunct of a nominal Coordination, as distinct from the
Coordination's own (*each creature and planeswalker*).
